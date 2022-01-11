use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Centroid
///
/// Fixes the frequency of each bin directly to the center.
pub struct Centroid;

impl Centroid {
    pub fn new() -> Self {
        Centroid {}
    }
}

impl PVocMiniPlugin for Centroid {
    fn process(
        &mut self,
        sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let freq_per_bin = sample_rate / (bins as f64);
        for i in 0..channels {
            for j in 0..bins {
                let expect = freq_per_bin * (j as f64) + freq_per_bin / 2.0;
                output[i][j].amp = input[i][j].amp;
                output[i][j].freq = expect;
            }
        }
    }
}
