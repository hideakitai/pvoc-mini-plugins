use super::lerp;
use super::PVocMiniPlugin;
use pvoc::Bin;

/// ## Time Blur
///
/// Uses exponential averaging to blur amplitude and frequency across time.
///
/// - Frequency alpha: exponential averaging alpha for frequency [0.0, 1.0]
/// - Amplitude alpha: exponential averaging alpha for amplitude [0.0, 1.0]
/// - Frequency mix: Mixer for original/modulated frequency [0.0, 1.0]
/// - Amplitude mix: Mixer for original/modulated amplitude [0.0, 1.0]
/// - Amplitude high replace: Mixer for replacing blurred amplitude with current amplitude when current amplitude exceeds blurred amplitude. [0.0, 1.0]
/// - Amplitude low replace: Mixer for replacing blurred amplitude with current amplitude when blurred amplitude exceeds current amplitude. [0.0, 1.0]
pub struct TimeBlur {
    buffer: Vec<Vec<Bin>>,
    pub freq_alpha: f64,
    pub amp_alpha: f64,
    pub freq_mix: f64,
    pub amp_mix: f64,
    pub replace_high: f64,
    pub replace_low: f64,
}

impl TimeBlur {
    pub fn new(
        channels: usize,
        bins: usize,
        freq_alpha: f64,
        amp_alpha: f64,
        freq_mix: f64,
        amp_mix: f64,
        replace_high: f64,
        replace_low: f64,
    ) -> Self {
        Self {
            buffer: vec![vec![Bin::new(0.0, 0.0); bins]; channels],
            freq_alpha,
            amp_alpha,
            freq_mix,
            amp_mix,
            replace_high,
            replace_low,
        }
    }
}

impl PVocMiniPlugin for TimeBlur {
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
        let replace_high = self.replace_high.clamp(0.0, 1.0);
        let replace_low = self.replace_low.clamp(0.0, 1.0);
        let buffer = &mut self.buffer;
        for i in 0..channels {
            for j in 0..bins {
                buffer[i][j].freq = lerp(buffer[i][j].freq, input[i][j].freq, freq_alpha);
                buffer[i][j].amp = lerp(buffer[i][j].amp, input[i][j].amp, amp_alpha);
                if input[i][j].amp > buffer[i][j].amp {
                    buffer[i][j].amp = lerp(input[i][j].amp, buffer[i][j].amp, replace_high);
                }
                if input[i][j].amp < buffer[i][j].amp {
                    buffer[i][j].amp = lerp(input[i][j].amp, buffer[i][j].amp, replace_low);
                }
                output[i][j].freq = lerp(buffer[i][j].freq, input[i][j].freq, freq_mix);
                output[i][j].amp = lerp(buffer[i][j].amp, input[i][j].amp, amp_mix);
            }
        }
    }
}
