use super::lerp;
use super::PVocMiniPlugin;
use pvoc::Bin;

/// Repeater
///
/// Capture a sound then repeat it indefinitely
///
/// - Length: The length of the section to repeat, in frames [1, 2000]
/// - Hold: Mixer for input signal/signal from loop buffer [0.0, 1.0]
/// - Decay: Multiplier for buffer amplitude [0.0, 1.0]
/// - Mix: Amplitude dry/wet. [0.0, 1.0]
pub struct Repeater {
    buffer: Vec<Vec<Vec<Bin>>>,
    time: usize,
    pub length: usize,
    pub freq_hold: f64,
    pub amp_hold: f64,
    pub decay: f64,
    pub mix: f64,
}

impl Repeater {
    const MAX_LENGTH: usize = 2000;

    pub fn new(
        channels: usize,
        bins: usize,
        length: usize,
        freq_hold: f64,
        amp_hold: f64,
        decay: f64,
        mix: f64,
    ) -> Self {
        Self {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; Self::MAX_LENGTH],
            time: 0,
            length,
            freq_hold,
            amp_hold,
            decay,
            mix,
        }
    }
}

impl PVocMiniPlugin for Repeater {
    fn process(
        &mut self,
        _sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let length = self.length.clamp(1, Self::MAX_LENGTH);
        let freq_hold = self.freq_hold.clamp(0.0, 1.0);
        let amp_hold = self.amp_hold.clamp(0.0, 1.0);
        let decay = self.decay.clamp(0.0, 1.0);
        let mix = self.mix.clamp(0.0, 1.0);

        self.time %= length;
        for i in 0..channels {
            for j in 0..bins {
                self.buffer[self.time][i][j].amp =
                    lerp(self.buffer[self.time][i][j].amp, input[i][j].amp, amp_hold);
                self.buffer[self.time][i][j].freq = lerp(
                    self.buffer[self.time][i][j].freq,
                    input[i][j].freq,
                    freq_hold,
                );
                output[i][j].amp = lerp(self.buffer[self.time][i][j].amp, input[i][j].amp, mix);
                output[i][j].freq = self.buffer[self.time][i][j].freq;
                self.buffer[self.time][i][j].amp *= decay;
            }
        }
        self.time += 1;
    }
}
