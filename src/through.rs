use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Through
///
/// Through from input to output without any effect
pub struct Through;

impl Through {
    pub fn new() -> Self {
        Self {}
    }
}

impl PVocMiniPlugin for Through {
    fn process(
        &mut self,
        _sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        for i in 0..channels {
            for j in 0..bins {
                output[i][j] = input[i][j];
            }
        }
    }
}
