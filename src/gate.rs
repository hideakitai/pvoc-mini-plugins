use super::PVocMiniPlugin;
use pvoc::Bin;

/// Gate
///
/// Filter out loud/quiet sounds
///
/// - Gate: don't let sounds through that are quieter than this threshold [0.0, 8.0]
/// - Duck: don't let sounds through that are louder than this threshold [0.0, 8.0]
pub struct Gate {
    pub gate: f64,
    pub duck: f64,
}

impl Gate {
    pub fn new(gate: f64, duck: f64) -> Self {
        Self { gate, duck }
    }
}

impl PVocMiniPlugin for Gate {
    fn process(
        &mut self,
        _sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let gate = self.gate.clamp(0.0, 8.0);
        let duck = self.duck.clamp(0.0, 8.0);
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq;
                let amp = (input[i][j].amp + 1.0).log2();
                // TODO smooth it out a bit at the boundary
                output[i][j].amp = if amp < gate || amp > duck {
                    0.0
                } else {
                    input[i][j].amp
                }
            }
        }
    }
}
