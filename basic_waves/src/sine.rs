use rodio::source::Source;
use std::f32::consts::PI;
use std::time::Duration;

/// An infinite source that produces a sine wave.
/// Has a definable sample rate and one channel.
#[derive(Debug, Clone)]
pub struct SineWave {
    freq: f32,
    number_of_samples: usize,
    sample_rate: u32,
}

impl SineWave {
    /// The frequency and sample rate of the sine wave.
    #[inline]
    pub fn new(freq: f32, sample_rate: u32) -> SineWave {
        SineWave {
            freq,
            sample_rate,
            ..Default::default()
        }
    }
}

impl Default for SineWave {
    /// Defines a sine wave of 440Hz (A above middle C)
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

impl Iterator for SineWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.number_of_samples = self.number_of_samples.wrapping_add(1);
        let value =
            2.0 * PI * self.freq * (self.number_of_samples as f32 / self.sample_rate as f32);
        Some(value.sin())
    }
}

impl Source for SineWave {
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
