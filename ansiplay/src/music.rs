use crate::IntoMusicSequenceIter;
use std::fmt::{self, Display, Formatter};

/// A representation of ANSI Music
#[derive(Clone, Debug)]
pub struct Music {
    entities: Vec<MusicEntity>,
}

impl From<&str> for Music {
    fn from(string: &str) -> Self {
        let entities = string
            .chars()
            .map(|char| char as u8)
            .collect::<Vec<u8>>()
            .as_slice()
            .into_musical_sequence_iter()
            .collect::<Vec<MusicEntity>>();
        Music { entities }
    }
}

impl Music {
    /// Constructs a new instance based on a struct that implements [IntoMusicSequenceIter]
    pub fn new<'a>(iter: impl IntoMusicSequenceIter<'a>) -> Music {
        let entities = iter
            .into_musical_sequence_iter()
            .collect::<Vec<MusicEntity>>();
        Music { entities }
    }
}

impl IntoIterator for Music {
    type Item = MusicEntity;
    type IntoIter = std::vec::IntoIter<MusicEntity>;

    fn into_iter(self) -> Self::IntoIter {
        self.entities.into_iter()
    }
}

impl Display for Music {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = self
            .entities
            .iter()
            .map(|entity| entity.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{string}")
    }
}

#[derive(Clone, Debug)]
pub enum Articulation {
    Legato,
    Normal,
    Stacato,
}

#[derive(Clone, Debug)]
pub enum MusicEntity {
    Operation(MusicOperation),
    Tempo(usize),
    Octave(usize),
    Length(usize),
    RawNote(usize),
    Pause(usize),
    IncreaseOctave,
    DecreaseOctave,
    Note { note: Note, info: NoteInfo },
    SoundCode(SoundCodeInfo),
}

#[derive(Clone, Debug)]
pub enum MusicOperation {
    None,
    Foreground,
    Background,
    Articulation(Articulation),
}

#[derive(Clone, Debug)]
pub enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Clone, Debug)]
pub struct NoteInfo {
    pub sign: NoteSign,
    pub length: Option<usize>,
    pub dots: usize,
}

#[derive(Clone, Debug)]
pub enum NoteSign {
    Sharp,
    Natural,
    Flat,
}

#[derive(Clone, Debug)]
pub struct SoundCodeInfo {
    pub frequency: Option<f32>,
    pub duration: Option<f32>,
    pub cycles: Option<usize>,
    pub delay: Option<usize>,
    pub variation: Option<Variation>,
}

#[derive(Clone, Debug)]
pub enum Variation {
    Value(f32),
    Random,
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

impl Display for MusicEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MusicEntity::Operation(operation) => operation.fmt(f),
            MusicEntity::Tempo(value) => write!(f, "T{value}"),
            MusicEntity::Octave(value) => write!(f, "O{value}"),
            MusicEntity::Length(value) => write!(f, "L{value}"),
            MusicEntity::RawNote(value) => write!(f, "N{value}"),
            MusicEntity::Pause(length) => write!(f, "P{length}"),
            MusicEntity::IncreaseOctave => write!(f, ">"),
            MusicEntity::DecreaseOctave => write!(f, "<"),
            MusicEntity::Note { note, info } => write!(f, "{note}{info}"),
            MusicEntity::SoundCode(info) => info.fmt(f),
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

impl Display for NoteSign {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NoteSign::Sharp => write!(f, "+"),
            NoteSign::Natural => Ok(()),
            NoteSign::Flat => write!(f, "-"),
        }
    }
}

impl fmt::Display for SoundCodeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.frequency {
            Some(value) => write!(f, "{value};")?,
            None => write!(f, ";")?,
        }
        match self.duration {
            Some(value) => write!(f, "{value};")?,
            None => write!(f, ";")?,
        }
        match self.cycles {
            Some(value) => write!(f, "{value};")?,
            None => write!(f, ";")?,
        }
        match self.delay {
            Some(value) => write!(f, "{value};")?,
            None => write!(f, ";")?,
        }
        match self.variation {
            Some(ref variation) => write!(f, "{variation}"),
            None => Ok(()),
        }
    }
}

impl Display for Variation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Variation::Value(value) => write!(f, "{value}"),
            Variation::Random => write!(f, "*"),
        }
    }
}
