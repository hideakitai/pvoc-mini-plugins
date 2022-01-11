use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Bin Flipper
///
/// This linearly inverts the frequency of each bin.
///
/// - Nyquist multiplier: multiplier for the center frequencies of the bins. [0.0, 1.0]
pub struct BinFlipper {
    pub nyquist_multiplier: f64,
}

impl BinFlipper {
    pub fn new(nyquist_multiplier: f64) -> Self {
        Self { nyquist_multiplier }
    }
}

impl PVocMiniPlugin for BinFlipper {
    fn process(
        &mut self,
        sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let mult = self.nyquist_multiplier.clamp(0.0, 1.0);
        let freq_per_bin = sample_rate / (bins as f64) * mult;
        for i in 0..channels {
            for j in 0..bins {
                let expect = freq_per_bin * (j as f64) + freq_per_bin / 2.0;
                let new = -(input[i][j].freq - expect) + expect;
                output[i][j].amp = input[i][j].amp;
                output[i][j].freq = new;
            }
        }
    }
}
