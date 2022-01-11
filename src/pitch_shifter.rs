use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Pitch Shifter
///
/// - Shift: Shift factor [0.0, 8.0]
pub struct PitchShifter {
    pub shift: f64,
}

impl PitchShifter {
    pub fn new(shift: f64) -> Self {
        Self { shift }
    }
}

impl PVocMiniPlugin for PitchShifter {
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
                    output[i][index].amp += input[i][j].amp;
                }
            }
        }
    }
}
