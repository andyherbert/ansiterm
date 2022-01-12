/*! A Basic Selection of wave generators for rodio */
mod noise;
mod saw;
mod sine;
mod square;
mod triangle;
pub use noise::NoiseWave;
pub use rodio::Source;
pub use saw::SawWave;
pub use sine::SineWave;
pub use square::SquareWave;
pub use triangle::TriangleWave;

#[cfg(test)]
mod test {
    use crate::SquareWave;
    use rodio::{source::Source, OutputStream, Sink};
    use std::time::Duration;

    fn play_freq(sink: &Sink, freq: f32) {
        let source = SquareWave::new(freq, 44100)
            .take_duration(Duration::from_secs_f32(0.25))
            .amplify(0.1);
        sink.append(source);
        std::thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn main() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        play_freq(&sink, 440.0); // A
        play_freq(&sink, 494.0); // B
        play_freq(&sink, 523.0); // C
        play_freq(&sink, 587.0); // D
        play_freq(&sink, 659.0); // E
        play_freq(&sink, 698.0); // F
        play_freq(&sink, 783.0); // G
        sink.sleep_until_end();
    }
}
