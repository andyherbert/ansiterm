use rodio::source::Source;
use std::f32::consts::PI;
use std::time::Duration;

/// An infinite source that produces a triangle wave.
/// Has a definable sample rate and one channel.
#[derive(Debug, Clone)]
pub struct TriangleWave {
    freq: f32,
    number_of_samples: usize,
    sample_rate: u32,
}

impl TriangleWave {
    /// The frequency and sample rate of the triangle wave.
    #[inline]
    pub fn new(freq: f32, sample_rate: u32) -> TriangleWave {
        TriangleWave {
            freq,
            sample_rate,
            ..Default::default()
        }
    }
}

impl Default for TriangleWave {
    /// Defines a triangle wave of 440Hz (A above middle C)
    /// with a sample rate of 48000.
    #[inline]
    fn default() -> Self {
        Self {
            freq: 440.0,
            number_of_samples: 0,
            sample_rate: 48000,
        }
    }
}

impl Iterator for TriangleWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.number_of_samples = self.number_of_samples.wrapping_add(1);
        let value = 2.0 / PI
            * (2.0 * PI * self.freq * self.number_of_samples as f32 / self.sample_rate as f32)
                .sin()
                .asin();
        Some(value)
    }
}

impl Source for TriangleWave {
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
