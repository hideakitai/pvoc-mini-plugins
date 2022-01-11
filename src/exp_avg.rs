use super::lerp;
use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Exponential Averaging
///
/// Modulates frequency and amplitude of bins based on exponential average of lower pitched bins.
///
/// - Frequency alpha: exponential averaging alpha for frequency [0.0, 1.0]
/// - Amplitude alpha: exponential averaging alpha for amplitude [0.0, 1.0]
/// - Frequency mix: Mixer for original/modulated frequency [0.0, 1.0]
/// - Amplitude mix: Mixer for original/modulated amplitude [0.0, 1.0]
pub struct ExpAvg {
    pub freq_alpha: f64,
    pub amp_alpha: f64,
    pub freq_mix: f64,
    pub amp_mix: f64,
}

impl ExpAvg {
    pub fn new(freq_alpha: f64, amp_alpha: f64, freq_mix: f64, amp_mix: f64) -> Self {
        Self {
            freq_alpha,
            amp_alpha,
            freq_mix,
            amp_mix,
        }
    }
}

impl PVocMiniPlugin for ExpAvg {
    fn process(
        &mut self,
        _sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let freq_alpha = self.freq_alpha.clamp(0.0, 1.0);
        let amp_alpha = self.amp_alpha.clamp(0.0, 1.0);
        let freq_mix = self.freq_mix.clamp(0.0, 1.0);
        let amp_mix = self.amp_mix.clamp(0.0, 1.0);
        for i in 0..channels {
            let mut avg_freq = input[i][0].freq;
            let mut avg_amp = input[i][0].amp;
            for j in 0..bins {
                output[i][j].freq = lerp(avg_freq, input[i][j].freq, freq_mix);
                output[i][j].amp = lerp(avg_amp, input[i][j].amp, amp_mix);
                avg_freq = lerp(avg_freq, input[i][j].freq, freq_alpha);
                avg_amp = lerp(avg_amp, input[i][j].amp, amp_alpha);
            }
        }
    }
}
