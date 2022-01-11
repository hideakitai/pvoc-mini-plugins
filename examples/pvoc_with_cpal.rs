//! Apply pvoc to mic input and output decoded one.
//! Uses a delay of milliseconds in case the input and output streams are not synchronized.
//!
//! Please use `-h` option to see the cli options

use anyhow::Result;
use clap::Parser;
use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::RingBuffer;

use pvoc::{Bin, PhaseVocoder};
use pvoc_mini_plugins::{
    AmpDelay, BinFlipper, Centroid, DomainXOver, ExpAvg, FormantShifter, FreqShifter, Gate,
    ModularAmp, PVocMiniPlugin, PitchShifter, Repeater, Scrambler, SlopeFilter, Stencil, Through,
    TimeBlur,
};

#[derive(Parser, Debug)]
#[clap(
    name = "pvoc_with_cpal",
    version = "0.1.0",
    author = "Hideaki Tai",
    about = "pvoc with cpal"
)]
struct Opt {
    /// Specify the delay between input and output
    #[clap(short, long, default_value = "Through")]
    plugin: String,
    /// Specify the delay between input and output
    #[clap(short, long, default_value_t = 150.0)]
    latency: f32,
    /// The input audio device to use
    #[clap(short, long, default_value = "default")]
    input_device: String,
    /// The output audio device to use
    #[clap(short, long, default_value = "default")]
    output_device: String,
}

fn main() -> Result<()> {
    // parse cli options
    let opt = Opt::parse();

    // create host
    let host = cpal::default_host();

    // list available devices
    for devices in host.input_devices() {
        devices.for_each(|device| {
            dbg!(device.name().unwrap());
        });
    }

    // find input device
    let input_device = if opt.input_device == "default" {
        host.default_input_device()
    } else {
        host.input_devices()?
            .find(|x| x.name().map(|y| y == opt.input_device).unwrap_or(false))
    }
    .expect("failed to find input device");

    // find output device
    let output_device = if opt.output_device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.output_device).unwrap_or(false))
    }
    .expect("failed to find output device");

    println!("Using input device: \"{}\"", input_device.name()?);
    println!("Using output device: \"{}\"", output_device.name()?);

    // get default configuration of input
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    // create a delay in case the input and output devices aren't synced
    let latency_frames = (opt.latency / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // the fifo buffer to share samples
    let ring = RingBuffer::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    // fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        producer.push(0.0).unwrap();
    }

    // Bins log2: the number of frequency bins used for the phase vocoder.
    // Few will likely be low quality and many will blur the audio through time.
    // Somewhere between 6 and 13 is usually what you want.
    let bins = 256;
    // Time divs: the number of overlapping frames to use.
    // Powers of two between 4 and 32 are good choices.
    let time_div = 32;
    // create PhaseVocoder
    let mut pvoc = PhaseVocoder::new(
        config.channels as usize,
        config.sample_rate.0 as f64,
        bins,
        time_div,
    );

    // create input data handler for cpal
    let input_data_fn = move |data: &[f32], _info: &cpal::InputCallbackInfo| {
        let channels = config.channels as usize;
        let sample_rate = config.sample_rate.0 as f64;
        let sample_len = data.len() / config.channels as usize;

        // temporary buffer to get Vec<&[f32]> and Vec<&mut [f32]>
        let mut input_vec: Vec<Vec<f32>> = vec![vec![]; channels];
        let mut output_vec: Vec<Vec<f32>> = vec![vec![]; channels];

        // reorder and separate input data to each channel
        for frame in data.chunks(channels) {
            for (ch, sample) in frame.iter().enumerate() {
                input_vec[ch].push(*sample);
                output_vec[ch].push(0.0);
            }
        }

        // we need Vec<&[f32]> and Vec<&mut [f32]> to feed to pvoc
        let mut input: Vec<&[f32]> = Vec::new();
        for iv in input_vec {
            input.push(unsafe { std::mem::transmute(&iv[..] as *const [f32]) });
        }
        let mut output: Vec<&mut [f32]> = Vec::new();
        for ov in output_vec {
            output.push(unsafe { std::mem::transmute(&ov[..] as *const [f32]) });
        }

        // execute pvoc
        pvoc.process(
            &input,
            &mut output,
            |channels: usize, bins: usize, input: &[Vec<Bin>], output: &mut [Vec<Bin>]| {
                let mut plugin: Box<dyn PVocMiniPlugin> = match opt.plugin.as_str() {
                    "AmpDelay" => {
                        Box::new(AmpDelay::new(channels, bins, 1.0, 1, 1.0, 1.0, 1.0, 1.0))
                    }
                    "BinFlipper" => Box::new(BinFlipper::new(0.01)),
                    "Centroid" => Box::new(Centroid::new()),
                    "DomainXOver" => Box::new(DomainXOver::new(15.0, 0.5, 0.5)),
                    "ExpAvg" => Box::new(ExpAvg::new(0.8, 0.2, 0.3, 0.7)),
                    "FormantShifter" => Box::new(FormantShifter::new(0.0)),
                    "FreqShifter" => Box::new(FreqShifter::new(8.0)),
                    "Gate" => Box::new(Gate::new(0.5, 7.0)),
                    "ModularAmp" => Box::new(ModularAmp::new(12.5)),
                    "PitchShifter" => Box::new(PitchShifter::new(8.0)),
                    "Repeater" => Box::new(Repeater::new(channels, bins, 10, 0.5, 0.2, 0.1, 0.5)),
                    "Scrambler" => Box::new(Scrambler::new(channels, bins, 10, 1)),
                    "SlopeFilter" => Box::new(SlopeFilter::new(channels, bins, 0.1, 0.8, 0.1, 0.8)),
                    "Stencil" => Box::new(Stencil::new(channels, bins, 100)),
                    "TimeBlur" => {
                        Box::new(TimeBlur::new(channels, bins, 0.5, 0.5, 0.5, 1.0, 0.8, 0.2))
                    }
                    _ => Box::new(Through::new()),
                };
                plugin.process(sample_rate, channels, bins, input, output);
            },
        );

        // reorder and integrate output data to one array
        let mut output_fell_behind = false;
        for i in 0..sample_len {
            for ch in 0..channels {
                if producer.push(output[ch][i]).is_err() {
                    output_fell_behind = true;
                }
            }
        }

        // if pushing to ring_buffer is failed
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    // create output data handler for cpal
    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // set pvoc output to output buffer of cpal
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }

        // if popping from ring_buffer is failed
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };

    // build input/output stream of cpal
    println!("Build streams with f32 samples and `{:?}`.", config);
    let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn)?;
    let output_stream = output_device.build_output_stream(&config, output_data_fn, err_fn)?;
    println!("Successfully built streams.");

    // play the streams
    println!("Start streams with `{}` ms of latency", opt.latency);
    input_stream.play()?;
    output_stream.play()?;

    println!("Press q + Enter to quit the app");
    loop {
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if !buffer.is_empty() {
            buffer.truncate(1);
            if buffer == "q" {
                break;
            }
        }
    }

    // destruct streams
    drop(input_stream);
    drop(output_stream);

    println!("Done!");
    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
