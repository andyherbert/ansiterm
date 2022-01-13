use crate::{
    Articulation, MusicalEntity, MusicalNote, MusicalOperation, NoteInfo, NoteSign, SoundCodeInfo,
    Variation,
};

pub struct MusicalSequenceIterator<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl MusicalSequenceIterator<'_> {
    fn parse_dots(&mut self) -> usize {
        let mut amount = 0;
        while let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                // ','
                0x2e => amount += 1,
                // ' '
                0x20 => {}
                _ => {
                    self.position -= 1;
                    break;
                }
            }
        }
        amount
    }

    fn parse_operation(&mut self) -> Option<MusicalOperation> {
        if let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                // 'B'
                0x42 => Some(MusicalOperation::Background),
                // 'F'
                0x46 => Some(MusicalOperation::Foreground),
                // 'L' | 'l'
                0x4c | 0x6d => Some(MusicalOperation::Articulation(Articulation::Legato)),
                // 'N' | 'n'
                0x4e | 0x6e => Some(MusicalOperation::Articulation(Articulation::Normal)),
                // 'S' | 's'
                0x53 | 0x73 => Some(MusicalOperation::Articulation(Articulation::Stacato)),
                // ' '
                0x20 => Some(MusicalOperation::None),
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_int(&mut self, accept_whitespace: bool, accept_semi_colon: bool) -> Option<usize> {
        let mut number = None;
        while let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                // '0'..='9'
                0x30..=0x39 => {
                    if let Some(value) = &mut number {
                        *value = (*value * 10) + (*byte as usize - 0x30);
                    } else {
                        number = Some(*byte as usize - 0x30);
                    }
                }
                // ';'
                0x3b if accept_semi_colon => break,
                // ' '
                0x20 if accept_whitespace => {}
                _ => {
                    self.position -= 1;
                    break;
                }
            }
        }
        number
    }

    fn parse_sound_code_number(&mut self, accept_whitespace: bool) -> Option<f32> {
        let mut float = String::new();
        while let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                // '0'..='9' | '-' | '.'
                0x30..=0x39 | 0x2d | 0x2e => float.push(*byte as char),
                // ';'
                0x3b if float.is_empty() => return None,
                0x3b if !float.is_empty() => break,
                // ' '
                0x20 if accept_whitespace => {}
                _ => {
                    self.position -= 1;
                    break;
                }
            }
        }
        float.parse().ok()
    }

    fn parse_sound_code_number_or_wildcard(
        &mut self,
        accept_whitespace: bool,
    ) -> Option<Variation> {
        match self.bytes.get(self.position)? {
            // '*'
            0x2a => {
                self.position += 1;
                Some(Variation::Random)
            }
            _ => Some(Variation::Value(
                self.parse_sound_code_number(accept_whitespace)?,
            )),
        }
    }

    fn parse_sign(&mut self) -> NoteSign {
        if let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                // '+' | '#'
                0x2b | 0x23 => NoteSign::Sharp,
                // '-'
                0x2d => NoteSign::Flat,
                _ => {
                    self.position -= 1;
                    NoteSign::Natural
                }
            }
        } else {
            NoteSign::Natural
        }
    }

    fn parse_note_info(&mut self) -> NoteInfo {
        NoteInfo {
            sign: self.parse_sign(),
            length: self.parse_int(false, false),
            dots: self.parse_dots(),
        }
    }

    fn parse_sound_code(&mut self) -> Option<SoundCodeInfo> {
        let frequency = self.parse_sound_code_number(false);
        let duration = self.parse_sound_code_number(false);
        let cycles = self.parse_int(false, true);
        let delay = self.parse_int(false, true);
        let variation = self.parse_sound_code_number_or_wildcard(false);
        Some(SoundCodeInfo {
            frequency,
            duration,
            cycles,
            delay,
            variation,
        })
    }
}

impl<'a> Iterator for MusicalSequenceIterator<'a> {
    type Item = MusicalEntity;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                // '-' | '.' | '0'..='9' | ';'
                0x2d | 0x2e | 0x30..=0x39 | 0x3b => {
                    self.position -= 1;
                    return Some(MusicalEntity::SoundCode(self.parse_sound_code()?));
                }
                // 'a' | 'A'
                0x61 | 0x41 => {
                    return Some(MusicalEntity::Note(MusicalNote::A, self.parse_note_info()));
                }
                // 'b' | 'B'
                0x62 | 0x42 => {
                    return Some(MusicalEntity::Note(MusicalNote::B, self.parse_note_info()));
                }
                // 'c' | 'C'
                0x63 | 0x43 => {
                    return Some(MusicalEntity::Note(MusicalNote::C, self.parse_note_info()));
                }
                // 'd' | 'd'
                0x64 | 0x44 => {
                    return Some(MusicalEntity::Note(MusicalNote::D, self.parse_note_info()));
                }
                // 'e' | 'E'
                0x65 | 0x45 => {
                    return Some(MusicalEntity::Note(MusicalNote::E, self.parse_note_info()));
                }
                // 'f' | 'F'
                0x66 | 0x46 => {
                    return Some(MusicalEntity::Note(MusicalNote::F, self.parse_note_info()));
                }
                // 'g' | 'G'
                0x67 | 0x47 => {
                    return Some(MusicalEntity::Note(MusicalNote::G, self.parse_note_info()));
                }
                // 'L' | 'l'
                0x4c | 0x6c => return self.parse_int(true, false).map(MusicalEntity::Length),
                // 'M' | 'm'
                0x4d | 0x6d => return self.parse_operation().map(MusicalEntity::Operation),
                // 'N' | 'n'
                0x4e | 0x6e => return self.parse_int(true, false).map(MusicalEntity::RawNote),
                // 'O' | 'o'
                0x4f | 0x6f => return self.parse_int(true, false).map(MusicalEntity::Octave),
                // 'P' | 'p'
                0x50 | 0x70 => return Some(MusicalEntity::Pause(self.parse_int(true, false)?)),
                // 'T' | 't'
                0x54 | 0x74 => return self.parse_int(true, false).map(MusicalEntity::Tempo),
                // '<'
                0x3c => return Some(MusicalEntity::DecreaseOctave),
                // '>'
                0x3e => return Some(MusicalEntity::IncreaseOctave),
                // ' '
                0x20 => {}
                _ => {}
            }
        }
        None
    }
}

pub trait IntoMusicalSequenceIter<'a> {
    fn into_musical_sequence_iter(self) -> MusicalSequenceIterator<'a>;
}

impl<'a> IntoMusicalSequenceIter<'a> for &'a [u8] {
    fn into_musical_sequence_iter(self) -> MusicalSequenceIterator<'a> {
        MusicalSequenceIterator {
            bytes: self,
            position: 0,
        }
    }
}
