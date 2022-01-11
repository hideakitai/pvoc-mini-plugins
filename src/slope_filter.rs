use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Slope Filter
/// Filter out sounds that are changing in frequency or amplitude
///
/// - Freq min/max: thresholds for filter to activate [0.0, 0.1]
/// - Amp min/max: thresholds for filter to activate [0.0, 8.0]
pub struct SlopeFilter {
    buffer: Vec<Vec<Bin>>,
    pub freq_min: f64,
    pub freq_max: f64,
    pub amp_min: f64,
    pub amp_max: f64,
}

impl SlopeFilter {
    pub fn new(
        channels: usize,
        bins: usize,
        freq_min: f64,
        freq_max: f64,
        amp_min: f64,
        amp_max: f64,
    ) -> Self {
        Self {
            buffer: vec![vec![Bin::new(0.0, 0.0); bins]; channels],
            freq_min,
            freq_max,
            amp_min,
            amp_max,
        }
    }
}

impl PVocMiniPlugin for SlopeFilter {
    fn process(
        &mut self,
        _: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let freq_min = self.freq_min.clamp(0.0, 0.1);
        let freq_max = self.freq_max.clamp(0.0, 0.1);
        let amp_min = self.amp_min.clamp(0.0, 8.0);
        let amp_max = self.amp_max.clamp(0.0, 8.0);
        for i in 0..channels {
            for j in 0..bins {
                output[i][j].freq = input[i][j].freq;

                let amp_slope =
                    ((input[i][j].amp + 1.0).log2() - (self.buffer[i][j].amp + 1.0).log2()).abs();
                let freq_slope =
                    ((input[i][j].freq + 1.0).log2() - (self.buffer[i][j].freq + 1.0).log2()).abs();
                output[i][j].amp = if amp_slope < amp_min
                    || amp_slope > amp_max
                    || freq_slope < freq_min
                    || freq_slope > freq_max
                {
                    0.0
                } else {
                    input[i][j].amp
                };
                self.buffer[i][j] = input[i][j];
            }
        }
    }
}
