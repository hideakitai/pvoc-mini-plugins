use super::fmod;
use super::PVocMiniPlugin;
use pvoc::Bin;

/// Modular Amplitude
///
/// Performs floating point modulus on the amplitude of each bin.
///
/// - Mod: Divisor [0.0, 25.0]
pub struct ModularAmp {
    pub factor: f64,
}

impl ModularAmp {
    pub fn new(factor: f64) -> Self {
        Self { factor }
    }
}

impl PVocMiniPlugin for ModularAmp {
    fn process(
        &mut self,
        _sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let factor = self.factor.clamp(0.0, 25.0);
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq;
                output[i][j].amp = fmod(input[i][j].amp, factor);
            }
        }
    }
}
