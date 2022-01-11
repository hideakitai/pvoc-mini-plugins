use super::lerp;
use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Domain Crossover
///
/// Modulates frequency of the bins based on their amplitude.
///
/// - Add: Ring modulation factor - intensity of frequency modulation. [0.0, 2.5]
/// Shift: Frequency offset. [0.0, 1.0]
/// Alpha: Exponential averaging alpha for amplitude estimate. [0.0, 1.0]
pub struct DomainXOver {
    pub add: f64,
    pub shift: f64,
    pub alpha: f64,
}

impl DomainXOver {
    pub fn new(add: f64, shift: f64, alpha: f64) -> Self {
        Self { add, shift, alpha }
    }
}

impl PVocMiniPlugin for DomainXOver {
    fn process(
        &mut self,
        sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let freq_per_bin = sample_rate / (bins as f64);
        let add = self.add.clamp(0.0, 25.0);
        let shift = self.shift.clamp(0.0, 1.0);
        let alpha = self.alpha.clamp(0.0, 1.0);
        for i in 0..channels {
            let mut avg = input[i][0].amp;
            for j in 0..bins {
                output[i][j].freq =
                    input[i][j].freq + shift * freq_per_bin + ((avg - input[i][j].amp) * add);
                output[i][j].amp = input[i][j].amp;
                avg = lerp(avg, input[i][j].amp, alpha);
            }
        }
    }
}
