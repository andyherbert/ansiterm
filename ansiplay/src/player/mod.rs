use crate::{
    Articulation, MusicalEntity, MusicalNote, MusicalOperation, NoteInfo, NoteSign, SoundCodeInfo,
};
use rodio::source::{SineWave, Source};
use rodio::{OutputStreamHandle, Sink};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

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

pub struct Player {
    tempo: usize,
    length: usize,
    octave: usize,
    articulation: Articulation,
    rx: Option<Receiver<Self>>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            tempo: 120,
            length: 1,
            octave: 4,
            articulation: Articulation::Normal,
            rx: None,
        }
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self {
            tempo: self.tempo,
            length: self.length,
            octave: self.octave,
            articulation: self.articulation.clone(),
            rx: None,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Default::default()
    }

    fn get_frequency(&self, note: MusicalNote, sign: NoteSign) -> f32 {
        let mut index = self.octave * 12;
        index += match note {
            MusicalNote::A => 9,
            MusicalNote::B => 11,
            MusicalNote::C => 0,
            MusicalNote::D => 2,
            MusicalNote::E => 4,
            MusicalNote::F => 5,
            MusicalNote::G => 7,
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

    fn play_frequency(&self, frequency: f32, play_ms: usize, pause_ms: usize, sink: &Sink) {
        let wave =
            SineWave::new(frequency as u32).take_duration(Duration::from_millis(play_ms as u64));
        sink.append(wave);
        sink.sleep_until_end();
        thread::sleep(Duration::from_millis(pause_ms as u64));
    }

    fn play_sound_code(&self, sound_code: SoundCodeInfo, sink: &Sink) {
        if let (Some(mut frequency), Some(duration)) = (sound_code.frequency, sound_code.duration) {
            let play_ms = (duration / 18.2 * 1000.0).ceil() as usize;
            let pause_ms = sound_code.delay.unwrap_or(0);
            let cycles = sound_code.cycles.unwrap_or(1);
            let variation = sound_code.variation.unwrap_or(0.0);
            for _ in 0..cycles {
                self.play_frequency(frequency, play_ms, pause_ms, sink);
                frequency += variation;
            }
        }
    }

    fn pause(&self, quarter_notes: usize) {
        let pause_ms = 60.0 * 1000.0 / self.tempo as f32 * 4.0 / quarter_notes as f32;
        thread::sleep(Duration::from_millis(pause_ms as u64));
    }

    fn play_note(&self, note: MusicalNote, info: NoteInfo, sink: &Sink) {
        let (play_ms, pause_ms) = self.calculate_length(info.length, info.dots);
        let frequency = self.get_frequency(note, info.sign);
        self.play_frequency(frequency, play_ms, pause_ms, sink);
    }

    fn play_raw_note(&self, value: usize, sink: &Sink) {
        let (play_ms, pause_ms) = self.calculate_length(None, 0);
        let frequency = FREQS[value];
        self.play_frequency(frequency, play_ms, pause_ms, sink);
    }

    pub fn play(&mut self, entities: Vec<MusicalEntity>, sink: Sink) {
        for entity in entities {
            match entity {
                MusicalEntity::Operation(MusicalOperation::Articulation(articulation)) => {
                    self.articulation = articulation
                }
                MusicalEntity::Operation(_operation) => {}
                MusicalEntity::Tempo(value) => self.tempo = value,
                MusicalEntity::Octave(value) => self.octave = value,
                MusicalEntity::Length(value) => self.length = value,
                MusicalEntity::RawNote(value) => self.play_raw_note(value, &sink),
                MusicalEntity::Pause(value) => self.pause(value),
                MusicalEntity::IncreaseOctave => self.octave += 1,
                MusicalEntity::DecreaseOctave => self.octave -= 1,
                MusicalEntity::Note(note, info) => self.play_note(note, info, &sink),
                MusicalEntity::SoundCode(info) => self.play_sound_code(info, &sink),
            }
        }
    }

    pub fn spawn_and_play(
        &mut self,
        entities: Vec<MusicalEntity>,
        stream_handle: &OutputStreamHandle,
    ) {
        let (tx, rx) = channel();
        let mut player = self.clone();
        let sink = Sink::try_new(stream_handle).expect("Failed to create sink");
        thread::spawn(move || {
            player.play(entities, sink);
            tx.send(player).unwrap();
        });
        self.rx = Some(rx);
    }

    pub fn is_playing(&mut self) -> bool {
        if let Some(rx) = &self.rx {
            match rx.try_recv() {
                Ok(player) => *self = player,
                Err(_) => return true,
            }
        }
        false
    }
}
