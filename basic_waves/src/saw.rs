use rodio::source::Source;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SawWave {
    freq: f32,
    number_of_samples: usize,
    sample_rate: u32,
    reverse: bool,
}

impl SawWave {
    #[inline]
    pub fn new(freq: f32, sample_rate: u32) -> SawWave {
        SawWave {
            freq,
            sample_rate,
            reverse: false,
            ..Default::default()
        }
    }

    #[inline]
    pub fn reverse(freq: f32, sample_rate: u32) -> SawWave {
        SawWave {
            freq,
            sample_rate,
            reverse: true,
            ..Default::default()
        }
    }
}

impl Default for SawWave {
    #[inline]
    fn default() -> Self {
        Self {
            freq: 440.0,
            number_of_samples: 0,
            sample_rate: 48000,
            reverse: false,
        }
    }
}

impl Iterator for SawWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.number_of_samples = self.number_of_samples.wrapping_add(1);
        let value = 2.0 / PI
            * (self.freq * (PI * self.number_of_samples as f32) / self.sample_rate as f32)
                .tan()
                .recip()
                .atan();
        if self.reverse {
            Some(-value)
        } else {
            Some(value)
        }
    }
}

impl Source for SawWave {
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
