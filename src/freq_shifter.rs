use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Frequency Shifter
///
/// - Shift: Shift factor [0.0, 8.0]
pub struct FreqShifter {
    pub shift: f64,
}

impl FreqShifter {
    pub fn new(shift: f64) -> Self {
        Self { shift }
    }
}

impl PVocMiniPlugin for FreqShifter {
    fn process(
        &mut self,
        _sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let shift = self.shift.clamp(0.0, 8.0);

        for i in 0..channels {
            for j in 0..bins / 2 {
                let index = ((j as f64) * shift) as usize;
                if index < bins / 2 {
                    output[i][index].freq = input[i][j].freq * shift;
                }
                output[i][j].amp = input[i][j].amp;
            }
        }
    }
}
