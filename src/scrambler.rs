use super::PVocMiniPlugin;
use pvoc::Bin;

/// Scrambler
///
/// Reorder frames by indexing into a circular buffer by some specified increment.
///
/// - Length: Circular buffer length [1, 4096]
/// - Increment: Circular buffer index increment [1, 4096]
pub struct Scrambler {
    pub length: usize,
    pub increment: usize,
    buffer: Vec<Vec<Vec<Bin>>>,
    time: usize,
    k: usize,
}

impl Scrambler {
    const MAX_LENGTH: usize = 4096;

    pub fn new(channels: usize, bins: usize, length: usize, increment: usize) -> Self {
        Self {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; Self::MAX_LENGTH],
            time: 0,
            k: 0,
            length,
            increment,
        }
    }
}

impl PVocMiniPlugin for Scrambler {
    fn process(
        &mut self,
        _sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let length = self.length.clamp(1, Self::MAX_LENGTH);
        let increment = self.increment.clamp(1, Self::MAX_LENGTH);

        self.time %= length;
        self.k %= length;
        for i in 0..channels {
            for j in 0..bins {
                self.buffer[self.time][i][j].amp = input[i][j].amp;
                self.buffer[self.time][i][j].freq = input[i][j].freq;
                output[i][j].amp = self.buffer[self.k][i][j].amp;
                output[i][j].freq = self.buffer[self.k][i][j].freq;
            }
        }
        self.time += 1;
        self.k += increment;
    }
}
