mod music_sequence_iterator;
pub mod player;
pub use music_sequence_iterator::{IntoMusicSequenceIter, MusicSequenceIterator};
use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
pub enum Articulation {
    Legato,
    Normal,
    Stacato,
}

#[derive(Clone)]
pub enum MusicOperation {
    None,
    Foreground,
    Background,
    Articulation(Articulation),
}

pub enum NoteSign {
    Sharp,
    Natural,
    Flat,
}

pub struct NoteInfo {
    pub sign: NoteSign,
    pub length: Option<usize>,
    pub dots: usize,
}

pub enum Variation {
    Value(f32),
    Random,
}

impl Display for Variation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Variation::Value(value) => write!(f, "{}", value),
            Variation::Random => write!(f, "*"),
        }
    }
}

pub struct SoundCodeInfo {
    pub frequency: Option<f32>,
    pub duration: Option<f32>,
    pub cycles: Option<usize>,
    pub delay: Option<usize>,
    pub variation: Option<Variation>,
}

pub enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

pub enum MusicEntity {
    Operation(MusicOperation),
    Tempo(usize),
    Octave(usize),
    Length(usize),
    RawNote(usize),
    Pause(usize),
    IncreaseOctave,
    DecreaseOctave,
    Note(Note, NoteInfo),
    SoundCode(SoundCodeInfo),
}

impl Display for Articulation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Articulation::Legato => write!(f, "ML"),
            Articulation::Normal => write!(f, "MN"),
            Articulation::Stacato => write!(f, "MS"),
        }
    }
}

impl Display for MusicOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MusicOperation::None => write!(f, "M"),
            MusicOperation::Foreground => write!(f, "MF"),
            MusicOperation::Background => write!(f, "MB"),
            MusicOperation::Articulation(articulation) => articulation.fmt(f),
        }
    }
}

impl Display for NoteSign {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NoteSign::Sharp => write!(f, "+"),
            NoteSign::Natural => Ok(()),
            NoteSign::Flat => write!(f, "-"),
        }
    }
}

impl Display for NoteInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.sign.fmt(f)?;
        if let Some(length) = self.length {
            length.fmt(f)?;
        }
        for _ in 0..self.dots {
            write!(f, ".")?;
        }
        Ok(())
    }
}

impl fmt::Display for SoundCodeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.frequency {
            value.fmt(f)?;
        }
        match self.duration {
            Some(value) => write!(f, ";{}", value)?,
            None => write!(f, ";")?,
        }
        match self.cycles {
            Some(value) => write!(f, ";{}", value)?,
            None => write!(f, ";")?,
        }
        match self.delay {
            Some(value) => write!(f, ";{}", value)?,
            None => write!(f, ";")?,
        }
        match self.variation {
            Some(ref variation) => write!(f, ";{}", variation),
            None => Ok(()),
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Note::A => write!(f, "A"),
            Note::B => write!(f, "B"),
            Note::C => write!(f, "C"),
            Note::D => write!(f, "D"),
            Note::E => write!(f, "E"),
            Note::F => write!(f, "F"),
            Note::G => write!(f, "G"),
        }
    }
}

impl Display for MusicEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MusicEntity::Operation(operation) => operation.fmt(f),
            MusicEntity::Tempo(value) => write!(f, "T{}", value),
            MusicEntity::Octave(value) => write!(f, "O{}", value),
            MusicEntity::Length(value) => write!(f, "L{}", value),
            MusicEntity::RawNote(value) => write!(f, "N{}", value),
            MusicEntity::Pause(length) => write!(f, "P{}", length),
            MusicEntity::IncreaseOctave => write!(f, ">"),
            MusicEntity::DecreaseOctave => write!(f, "<"),
            MusicEntity::Note(note, info) => write!(f, "{}{}", note, info),
            MusicEntity::SoundCode(info) => info.fmt(f),
        }
    }
}

pub fn entities_from_str(string: &str) -> Vec<MusicEntity> {
    format!("\x1b[M {}\x0e", string)
        .chars()
        .map(|char| char as u8)
        .collect::<Vec<u8>>()
        .as_slice()
        .into_musical_sequence_iter()
        .collect::<Vec<MusicEntity>>()
}

#[cfg(test)]
mod test {
    use crate::*;
    use rodio::{OutputStream, Sink};

    #[test]
    fn test_play() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let entities = entities_from_str("1991;0.25;2;50");
        let mut player = crate::player::Player::default();
        let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
        player.play(entities, sink)
    }

    #[test]
    fn test_play_2() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let entities = entities_from_str("100;2;10;5;*");
        let mut player = crate::player::Player::default();
        let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
        player.play(entities, sink)
    }

    #[test]
    fn test_play_3() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let entities = entities_from_str("ABC M ");
        let mut player = crate::player::Player::default();
        let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
        player.play(entities, sink)
    }
}
