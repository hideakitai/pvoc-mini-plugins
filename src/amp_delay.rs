use super::lerp;
use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Amplitude Scaled Delay
///
/// Each bin is delayed by an amount relative to it's amplitude. Delay is measured in frames that are bins/time-div/sample-rate seconds long.
///
/// - Delay: amount of time to delay by [0.0, 2000.0]
/// - Max delay: delay buffer size [1, 2000]
/// - Frequency/amplitude mix: mixer for delayed/original signal [0.0, 1.0]
/// - Frequency/amplitude feedback: multiplier for previously read events - at 1, samples will remain in the buffer until they are overwritten, possibly looping after the max delay. [0.0, 1.0]
pub struct AmpDelay {
    buffer: Vec<Vec<Vec<Bin>>>,
    time: usize,
    pub delay: f64,
    pub max_delay: usize,
    pub freq_mix: f64,
    pub amp_mix: f64,
    pub freq_feedback: f64,
    pub amp_feedback: f64,
}

impl AmpDelay {
    pub fn new(
        channels: usize,
        bins: usize,
        delay: f64,
        max_delay: usize,
        freq_mix: f64,
        amp_mix: f64,
        freq_feedback: f64,
        amp_feedback: f64,
    ) -> Self {
        Self {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; 0],
            time: 0,
            delay,
            max_delay,
            freq_mix,
            amp_mix,
            freq_feedback,
            amp_feedback,
        }
    }
}

impl PVocMiniPlugin for AmpDelay {
    fn process(
        &mut self,
        _sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let delay = self.delay.clamp(0.0, 2000.0);
        let max_delay = self.max_delay.clamp(1, 2000);
        let freq_mix = self.freq_mix.clamp(0.0, 1.0);
        let amp_mix = self.amp_mix.clamp(0.0, 1.0);
        let freq_feedback = self.freq_feedback.clamp(0.0, 1.0);
        let amp_feedback = self.amp_feedback.clamp(0.0, 1.0);
        let buffer = &mut self.buffer;

        if max_delay != self.max_delay {
            self.max_delay = max_delay;
            buffer.resize(max_delay, vec![vec![Bin::new(0.0, 0.0); bins]; channels]);
        }

        self.time %= max_delay;
        for i in 0..channels {
            for j in 0..bins {
                let bin_delay = ((input[i][j].amp + 1.0).log2() * delay) as usize;
                buffer[(self.time + bin_delay) % max_delay][i][j] = input[i][j];
                output[i][j].amp = lerp(buffer[self.time][i][j].amp, input[i][j].amp, amp_mix);
                output[i][j].freq = lerp(buffer[self.time][i][j].freq, input[i][j].freq, freq_mix);
                buffer[self.time][i][j].freq *= freq_feedback;
                buffer[self.time][i][j].amp *= amp_feedback;
            }
        }
        self.time += 1;
    }
}
