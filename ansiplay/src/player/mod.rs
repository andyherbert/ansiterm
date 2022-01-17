mod player_error;
mod player_thread;
use crate::music::*;
use basic_waves::{Source, SquareWave};
use player_error::PlayerError;
pub use player_thread::PlayerThread;
use player_thread::ThreadMessage;
use rand::prelude::*;
use rodio::Sink;
use std::{sync::mpsc, thread, time};

const FREQS: [f32; 84] = [
    65.406, 69.296, 73.416, 77.782, 82.406, 87.308, 92.498, 97.998, 103.826, 110.0, 116.54, 123.47,
    130.812, 138.592, 146.832, 155.564, 164.821, 174.614, 185.0, 195.998, 207.66, 220.0, 233.08,
    246.94, 261.62, 277.18, 296.66, 311.12, 329.62, 349.22, 370.0, 392.0, 415.3, 440.0, 466.16,
    493.88, 523.26, 554.36, 587.32, 622.26, 659.26, 698.46, 739.98, 784.0, 830.6, 880.0, 892.32,
    987.76, 1046.5, 1108.74, 1174.66, 1244.5, 1318.52, 1396.92, 1479.98, 1567.98, 1661.22, 1760.0,
    1864.66, 1975.54, 2093.0, 2217.4, 2349.4, 2489.0, 2637.0, 2793.8, 2960.0, 3136.0, 3322.4,
    3520.0, 3729.4, 3951.0, 4186.0, 4435.0, 4698.6, 4978.0, 5274.0, 5587.6, 5920.0, 6272.0, 6644.8,
    7040.0, 7458.6, 7902.2,
];

/// A struct which provides an interface to play [Music], or play music in a new thread.
#[derive(Debug)]
pub struct Player {
    tempo: usize,
    length: usize,
    octave: usize,
    articulation: Articulation,
    rx: Option<mpsc::Receiver<ThreadMessage>>,
    rng: StdRng,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            tempo: 120,
            length: 1,
            octave: 4,
            articulation: Articulation::Normal,
            rx: None,
            rng: StdRng::seed_from_u64(random()),
        }
    }
}

impl Player {
    /// Equivalent to [Player::default]
    pub fn new() -> Player {
        Default::default()
    }

    fn get_frequency(&self, note: Note, sign: NoteSign) -> f32 {
        let mut index = self.octave * 12;
        index += match note {
            Note::A => 9,
            Note::B => 11,
            Note::C => 0,
            Note::D => 2,
            Note::E => 4,
            Note::F => 5,
            Note::G => 7,
        };
        match sign {
            NoteSign::Sharp => FREQS[index + 1],
            NoteSign::Natural => FREQS[index],
            NoteSign::Flat => FREQS[index - 1],
        }
    }

    fn calculate_length(&self, length: Option<usize>, dots: usize) -> (usize, usize) {
        let length = length.unwrap_or(self.length);
        let full_note = 60.0 * 1000.0 / self.tempo as f32 * 4.0 / length as f32;
        let mut note_length = full_note;
        match self.articulation {
            Articulation::Legato => note_length *= 3.0 / 4.0,
            Articulation::Normal => note_length *= 7.0 / 8.0,
            Articulation::Stacato => {}
        }
        let mut extra = 0.0;
        for dot in 0..dots {
            extra += match dot {
                0 => note_length as f32 * 1.0 / 2.0,
                _ => 1.0 / 2.0,
            }
        }
        (
            (note_length + extra).ceil() as usize,
            (full_note - note_length).ceil() as usize,
        )
    }

    fn play_pause(&self, pause_ms: usize, sink: &Sink) {
        if pause_ms > 0 {
            let source = SquareWave::new(0.0, 48000)
                .take_duration(time::Duration::from_millis(pause_ms as u64))
                .amplify(0.0);
            sink.append(source);
        }
    }

    fn play_frequency(&self, frequency: f32, play_ms: usize, pause_ms: usize, sink: &Sink) {
        if play_ms > 0 {
            let source = SquareWave::new(frequency, 48000)
                .amplify(0.1)
                .take_duration(time::Duration::from_millis(play_ms as u64));
            sink.append(source);
        }
        self.play_pause(pause_ms, sink);
    }

    fn play_sound_code(&mut self, info: SoundCodeInfo, sink: &Sink) {
        if let (Some(mut frequency), Some(duration)) = (info.frequency, info.duration) {
            let play_ms = (duration / 18.2 * 1000.0).ceil() as usize;
            let pause_ms = info.delay.unwrap_or(0);
            let cycles = info.cycles.unwrap_or(1);
            if cycles == 0 {
                self.play_frequency(frequency, play_ms, pause_ms, sink);
            } else {
                for _ in 0..cycles {
                    self.play_frequency(frequency, play_ms, 0, sink);
                    frequency += match info.variation {
                        Some(Variation::Value(value)) => value,
                        Some(Variation::Random) => self.rng.gen_range(-512.0..=512.0),
                        None => 0.0,
                    };
                }
                self.play_pause(pause_ms, sink);
            }
        } else if let Some(delay) = info.delay {
            let cycles = (delay as f32 / (1000.0 / 60.0)).floor() as usize;
            let dur = time::Duration::from_millis(1000 / 60);
            // Clear the channel, so any buffered messages are ignored.
            if let Some(ref rx) = self.rx {
                while let Ok(message) = rx.try_recv() {
                    match message {
                        ThreadMessage::Interrupt => continue,
                        ThreadMessage::Abort => return,
                    }
                }
            }
            for _ in 0..cycles {
                // Interrupt if a message is received.
                if let Some(ref rx) = self.rx {
                    if rx.try_recv().is_ok() {
                        break;
                    }
                }
                thread::sleep(dur);
            }
        }
    }

    fn pause(&self, quarter_notes: usize, sink: &Sink) {
        let pause_ms = 60.0 * 1000.0 / self.tempo as f32 * 4.0 / quarter_notes as f32;
        self.play_pause(pause_ms as usize, sink)
    }

    fn play_note(&self, note: Note, info: NoteInfo, sink: &Sink) {
        let (play_ms, pause_ms) = self.calculate_length(info.length, info.dots);
        let frequency = self.get_frequency(note, info.sign);
        self.play_frequency(frequency, play_ms, pause_ms, sink);
    }

    fn play_raw_note(&self, value: usize, sink: &Sink) {
        let (play_ms, pause_ms) = self.calculate_length(None, 0);
        let frequency = FREQS[value];
        self.play_frequency(frequency, play_ms, pause_ms, sink);
    }

    /// Plays [Music] through the supplied [Sink] and blocks the current thread.
    pub fn play(&mut self, music: Music, sink: Sink) {
        for entity in music {
            match entity {
                MusicEntity::Operation(MusicOperation::Articulation(articulation)) => {
                    self.articulation = articulation.clone();
                }
                MusicEntity::Operation(_operation) => {}
                MusicEntity::Tempo(value) => self.tempo = value,
                MusicEntity::Octave(value) => self.octave = value,
                MusicEntity::Length(value) => self.length = value,
                MusicEntity::RawNote(value) => self.play_raw_note(value, &sink),
                MusicEntity::Pause(value) => self.pause(value, &sink),
                MusicEntity::IncreaseOctave => self.octave += 1,
                MusicEntity::DecreaseOctave => self.octave -= 1,
                MusicEntity::Note(note, info) => self.play_note(note, info, &sink),
                MusicEntity::SoundCode(info) => self.play_sound_code(info, &sink),
            }
        }
        if let Some(ref rx) = self.rx {
            let dur = time::Duration::from_millis(1000 / 60);
            while !sink.empty() {
                thread::sleep(dur);
                if let Ok(ThreadMessage::Abort) = rx.try_recv() {
                    break;
                }
            }
        } else {
            sink.sleep_until_end();
        }
    }
}
