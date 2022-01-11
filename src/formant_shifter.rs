use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Formant Shifter
///
/// - Shift: Shift factor [0.0, 8.0]
pub struct FormantShifter {
    pub shift: f64,
}

impl FormantShifter {
    pub fn new(shift: f64) -> Self {
        Self { shift }
    }
}

impl PVocMiniPlugin for FormantShifter {
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
                    output[i][j].amp = input[i][index].amp;
                }
                output[i][j].freq = input[i][j].freq;
            }
        }
    }
}
