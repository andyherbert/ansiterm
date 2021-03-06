use rand::prelude::*;
use rodio::source::Source;
use std::time::Duration;

/// An infinite source that produces a white noise wave.
/// Has a definable sample rate and one channel.
#[derive(Debug, Clone)]
pub struct NoiseWave {
    std_rng: StdRng,
    sample_rate: u32,
}

impl NoiseWave {
    /// The frequency and sample rate of the white noise wave.
    #[inline]
    pub fn new(seed: u64, sample_rate: u32) -> NoiseWave {
        NoiseWave {
            std_rng: StdRng::seed_from_u64(seed),
            sample_rate,
        }
    }
}

impl Default for NoiseWave {
    /// Defines a white noise wave of 440Hz (A above middle C)
    /// with a sample rate of 48000.
    #[inline]
    fn default() -> Self {
        Self {
            std_rng: StdRng::seed_from_u64(0),
            sample_rate: 48000,
        }
    }
}

impl Iterator for NoiseWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let value: f32 = self.std_rng.gen_range(-1.0..1.0);
        Some(value)
    }
}

impl Source for NoiseWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
