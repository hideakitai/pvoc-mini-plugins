use super::PVocMiniPlugin;
use pvoc::Bin;

pub struct Stencil {
    buffer: Vec<Vec<Vec<Bin>>>,
    time: usize,
    pub stencil: usize,
}

impl Stencil {
    const SIZE: usize = 4;

    pub fn new(channels: usize, bins: usize, stencil: usize) -> Self {
        Self {
            buffer: vec![vec![vec![Bin::new(0.0, 0.0); bins]; channels]; Self::SIZE],
            time: 0,
            stencil,
        }
    }
}

impl PVocMiniPlugin for Stencil {
    fn process(
        &mut self,
        sample_rate: f64,
        channels: usize,
        bins: usize,
        input: &[Vec<Bin>],
        output: &mut [Vec<Bin>],
    ) {
        let stencil = self.stencil.clamp(0, (1 << Self::SIZE * Self::SIZE) - 1);
        let _freq_per_bin = sample_rate / (bins as f64);
        self.time %= Self::SIZE;
        for i in 0..channels {
            for j in 0..bins {
                self.buffer[self.time][i][j] = input[i][j];
            }
            for j in 0..bins {
                let mut tmp = stencil;
                let mut ncontrib = 0;
                for x in 0..Self::SIZE {
                    for y in 0..Self::SIZE {
                        if tmp & 1 == 1 {
                            let bin = j + x;
                            if bin < 2 || bin >= bins - 2 {
                                continue;
                            }
                            let bin = bin - 2;
                            let frame = (self.time + Self::SIZE - y) % Self::SIZE;
                            output[i][j].amp += self.buffer[frame][i][bin].amp;
                            // TODO adjust frequency by bin difference
                            output[i][j].freq += self.buffer[frame][i][bin].freq;
                            ncontrib += 1;
                        }
                        tmp >>= 1;
                    }
                }
                if ncontrib > 0 {
                    // TODO configurable gain compensation
                    output[i][j].amp /= ncontrib as f64;
                    output[i][j].freq /= ncontrib as f64;
                }
            }
        }
        self.time += 1;
    }
}
