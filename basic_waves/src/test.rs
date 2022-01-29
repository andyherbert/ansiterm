use crate::*;
use rodio::{OutputStream, OutputStreamHandle, Source};
use std::time::Duration;

fn play_freq<F, W>(freq: f32, wave: F, stream_handle: &OutputStreamHandle)
where
    F: Fn(f32, u32) -> W,
    W: Source<Item = f32> + Send + 'static,
{
    let source = wave(freq, 44100)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.1);

    stream_handle.play_raw(source).expect("sound");
    std::thread::sleep(Duration::from_millis(300));
}

fn play_wave<F, W>(wave: &F, stream_handle: &OutputStreamHandle)
where
    F: Fn(f32, u32) -> W,
    W: Source<Item = f32> + Send + 'static,
{
    play_freq(440.0, wave, stream_handle); // A
    play_freq(494.0, wave, stream_handle); // B
    play_freq(523.0, wave, stream_handle); // Middle C
    play_freq(587.0, wave, stream_handle); // D
    play_freq(659.0, wave, stream_handle); // E
    play_freq(698.0, wave, stream_handle); // F
    play_freq(783.0, wave, stream_handle); // G
}

#[test]
fn play_waves() {
    let (_stream, stream_handle) = OutputStream::try_default().expect("stream");
    play_wave(&SawWave::new, &stream_handle);
    play_wave(&SawWave::reverse, &stream_handle);
    play_wave(&SineWave::new, &stream_handle);
    play_wave(&SquareWave::new, &stream_handle);
    play_wave(&TriangleWave::new, &stream_handle);
    let source = NoiseWave::new(0, 48000)
        .take_duration(Duration::from_secs_f32(1.0))
        .amplify(0.1);
    stream_handle.play_raw(source).expect("sound");
    std::thread::sleep(Duration::from_millis(1300));
}
