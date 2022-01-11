//! Port of [pvoc-plugin](https://github.com/nwoeanhinnogaehr/pvoc-plugins) that can be available without [LADSPA](https://github.com/nwoeanhinnogaehr/ladspa.rs).
//!
//! ## License
//! GPL-3.0

use pvoc::Bin;

mod amp_delay;
mod bin_flipper;
mod centroid;
mod domain_xover;
mod exp_avg;
mod formant_shifter;
mod freq_shifter;
mod gate;
mod modular_amp;
mod pitch_shifter;
mod repeater;
mod scrambler;
mod slope_filter;
mod stencil;
mod through;
mod time_blur;

pub use amp_delay::AmpDelay;
pub use bin_flipper::BinFlipper;
pub use centroid::Centroid;
pub use domain_xover::DomainXOver;
pub use exp_avg::ExpAvg;
pub use formant_shifter::FormantShifter;
pub use freq_shifter::FreqShifter;
pub use gate::Gate;
pub use modular_amp::ModularAmp;
pub use pitch_shifter::PitchShifter;
pub use repeater::Repeater;
pub use scrambler::Scrambler;
pub use slope_filter::SlopeFilter;
pub use stencil::Stencil;
pub use through::Through;
pub use time_blur::TimeBlur;

/// The trait that is implemented to each plugins.
/// - Bins log2: the number of frequency bins used for the phase vocoder. Few will likely be low quality and many will blur the audio through time. Somewhere between 6 and 13 is usually what you want.
/// - Time divs: the number of overlapping frames to use. Powers of two between 4 and 32 are good choices.
pub trait PVocMiniPlugin {
    fn process(
        &mut self,
        sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    );
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a * x + b * (1.0 - x)
}

fn fmod(a: f64, b: f64) -> f64 {
    if b.is_infinite() {
        a
    } else {
        a - b * (a / b).floor()
    }
}
