use crate::music::*;

/// A struct that implements [Iterator] that can be used to produce [Music].
pub struct MusicSequenceIterator<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> MusicSequenceIterator<'a> {
    fn new(bytes: &'a [u8]) -> MusicSequenceIterator {
        MusicSequenceIterator { bytes, position: 0 }
    }

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

    fn parse_operation(&mut self) -> Option<MusicOperation> {
        if let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                // 'B'
                0x42 => Some(MusicOperation::Background),
                // 'F'
                0x46 => Some(MusicOperation::Foreground),
                // 'L' | 'l'
                0x4c | 0x6d => Some(MusicOperation::Articulation(Articulation::Legato)),
                // 'N' | 'n'
                0x4e | 0x6e => Some(MusicOperation::Articulation(Articulation::Normal)),
                // 'S' | 's'
                0x53 | 0x73 => Some(MusicOperation::Articulation(Articulation::Stacato)),
                // ' '
                0x20 => Some(MusicOperation::None),
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

    fn parse_sound_code_number(&mut self) -> Option<f32> {
        let minus = match self.bytes.get(self.position) {
            // '-'
            Some(0x2d) => {
                self.position += 1;
                true
            }
            // ';'
            Some(0x3b) => {
                self.position += 1;
                return None;
            }
            // ' '
            Some(0x20) => return None,
            _ => false,
        };
        // '0..9'
        let prefix = self.parse_int(false, false).unwrap_or(0);
        // '.'
        let postfix = if let Some(0x2e) = self.bytes.get(self.position) {
            self.position += 1;
            let post_fix = self.parse_int(false, false).unwrap_or(0);
            if post_fix == 0 {
                0.0
            } else {
                let mut value = post_fix;
                let mut mul: usize = 0;
                while value > 0 {
                    value /= 10;
                    if mul == 0 {
                        mul = 10;
                    } else {
                        mul *= 10;
                    }
                }
                post_fix as f32 / mul as f32
            }
        } else {
            0.0
        };
        match self.bytes.get(self.position) {
            // ';'
            Some(0x3b) => self.position += 1,
            None | Some(0x20) => {}
            Some(_) => return None,
        }
        let value = prefix as f32 + postfix as f32;
        if minus {
            Some(-value)
        } else {
            Some(value)
        }
    }

    fn parse_sound_code_number_or_wildcard(&mut self) -> Option<Variation> {
        match self.bytes.get(self.position)? {
            // '*'
            0x2a => {
                self.position += 1;
                Some(Variation::Random)
            }
            _ => Some(Variation::Value(self.parse_sound_code_number()?)),
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
        let frequency = self.parse_sound_code_number();
        let duration = self.parse_sound_code_number();
        let cycles = self.parse_int(false, true);
        let delay = self.parse_int(false, true);
        let variation = self.parse_sound_code_number_or_wildcard();
        Some(SoundCodeInfo {
            frequency,
            duration,
            cycles,
            delay,
            variation,
        })
    }
}

impl<'a> Iterator for MusicSequenceIterator<'a> {
    type Item = MusicEntity;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            match byte {
                // '-' | '.' | '0'..='9' | ';'
                0x2d | 0x2e | 0x30..=0x39 | 0x3b => {
                    self.position -= 1;
                    return Some(MusicEntity::SoundCode(self.parse_sound_code()?));
                }
                // 'a' | 'A'
                0x61 | 0x41 => return Some(MusicEntity::Note(Note::A, self.parse_note_info())),
                // 'b' | 'B'
                0x62 | 0x42 => return Some(MusicEntity::Note(Note::B, self.parse_note_info())),
                // 'c' | 'C'
                0x63 | 0x43 => return Some(MusicEntity::Note(Note::C, self.parse_note_info())),
                // 'd' | 'd'
                0x64 | 0x44 => return Some(MusicEntity::Note(Note::D, self.parse_note_info())),
                // 'e' | 'E'
                0x65 | 0x45 => return Some(MusicEntity::Note(Note::E, self.parse_note_info())),
                // 'f' | 'F'
                0x66 | 0x46 => return Some(MusicEntity::Note(Note::F, self.parse_note_info())),
                // 'g' | 'G'
                0x67 | 0x47 => return Some(MusicEntity::Note(Note::G, self.parse_note_info())),
                // 'L' | 'l'
                0x4c | 0x6c => return self.parse_int(true, false).map(MusicEntity::Length),
                // 'M' | 'm'
                0x4d | 0x6d => return self.parse_operation().map(MusicEntity::Operation),
                // 'N' | 'n'
                0x4e | 0x6e => return self.parse_int(true, false).map(MusicEntity::RawNote),
                // 'O' | 'o'
                0x4f | 0x6f => return self.parse_int(true, false).map(MusicEntity::Octave),
                // 'P' | 'p'
                0x50 | 0x70 => return Some(MusicEntity::Pause(self.parse_int(true, false)?)),
                // 'T' | 't'
                0x54 | 0x74 => return self.parse_int(true, false).map(MusicEntity::Tempo),
                // '<'
                0x3c => return Some(MusicEntity::DecreaseOctave),
                // '>'
                0x3e => return Some(MusicEntity::IncreaseOctave),
                // ' '
                0x20 => {}
                _ => {}
            }
        }
        None
    }
}

/// A Trait that returns a [MusicSequenceIterator].
pub trait IntoMusicSequenceIter<'a> {
    fn into_musical_sequence_iter(self) -> MusicSequenceIterator<'a>;
}

impl<'a> IntoMusicSequenceIter<'a> for &'a [u8] {
    fn into_musical_sequence_iter(self) -> MusicSequenceIterator<'a> {
        MusicSequenceIterator::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::MusicSequenceIterator;
    fn to_bytes(string: &str) -> Vec<u8> {
        string.chars().map(|char| char as u8).collect::<Vec<u8>>()
    }

    fn test_number(string: &str) -> Option<f32> {
        let bytes = to_bytes(string);
        let mut mus = MusicSequenceIterator::new(&bytes);
        mus.parse_sound_code_number()
    }

    #[test]
    fn test_operation() {
        assert_eq!(test_number(";"), None);
        assert_eq!(test_number("a"), None);
        assert_eq!(test_number("1").unwrap(), 1.0);
        assert_eq!(test_number("100").unwrap(), 100.0);
        assert_eq!(test_number("1.0").unwrap(), 1.0);
        assert_eq!(test_number("1.12345").unwrap(), 1.12345);
        assert_eq!(test_number("10.0").unwrap(), 10.0);
        assert_eq!(test_number(".0").unwrap(), 0.0);
        assert_eq!(test_number("0..9"), None);
        assert_eq!(test_number("0.9.0"), None);
        assert_eq!(test_number(".9").unwrap(), 0.9);
        assert_eq!(test_number("0.").unwrap(), 0.0);
        assert_eq!(test_number("9.").unwrap(), 9.0);
        assert_eq!(test_number(".").unwrap(), 0.0);
        assert_eq!(test_number("-a"), None);
        assert_eq!(test_number("-1").unwrap(), -1.0);
        assert_eq!(test_number("-100").unwrap(), -100.0);
        assert_eq!(test_number("-1.0").unwrap(), -1.0);
        assert_eq!(test_number("-10.0").unwrap(), -10.0);
        assert_eq!(test_number("-.0").unwrap(), 0.0);
        assert_eq!(test_number("-0..9"), None);
        assert_eq!(test_number("-0.9.0"), None);
        assert_eq!(test_number("-.9").unwrap(), -0.9);
        assert_eq!(test_number("-0.").unwrap(), -0.0);
        assert_eq!(test_number("-9.").unwrap(), -9.0);
        assert_eq!(test_number("-.").unwrap(), 0.0);
        assert_eq!(test_number("a;"), None);
        assert_eq!(test_number("1;").unwrap(), 1.0);
        assert_eq!(test_number("100;").unwrap(), 100.0);
        assert_eq!(test_number("1.0;").unwrap(), 1.0);
        assert_eq!(test_number("10.0;").unwrap(), 10.0);
        assert_eq!(test_number(".0;").unwrap(), 0.0);
        assert_eq!(test_number("0..9;"), None);
        assert_eq!(test_number("0.9.0;"), None);
        assert_eq!(test_number(".9;").unwrap(), 0.9);
        assert_eq!(test_number("0.;").unwrap(), 0.0);
        assert_eq!(test_number("9.;").unwrap(), 9.0);
        assert_eq!(test_number(".;").unwrap(), 0.0);
        assert_eq!(test_number("-a;"), None);
        assert_eq!(test_number("-1;").unwrap(), -1.0);
        assert_eq!(test_number("-100;").unwrap(), -100.0);
        assert_eq!(test_number("-1.0;").unwrap(), -1.0);
        assert_eq!(test_number("-10.0;").unwrap(), -10.0);
        assert_eq!(test_number("-.0;").unwrap(), 0.0);
        assert_eq!(test_number("-0..9;"), None);
        assert_eq!(test_number("-0.9.0;"), None);
        assert_eq!(test_number("-.9;").unwrap(), -0.9);
        assert_eq!(test_number("-0.;").unwrap(), -0.0);
        assert_eq!(test_number("-9.;").unwrap(), -9.0);
        assert_eq!(test_number("-.;").unwrap(), 0.0);
        assert_eq!(test_number("a "), None);
        assert_eq!(test_number("1 ").unwrap(), 1.0);
        assert_eq!(test_number("100 ").unwrap(), 100.0);
        assert_eq!(test_number("1.0 ").unwrap(), 1.0);
        assert_eq!(test_number("10.0 ").unwrap(), 10.0);
        assert_eq!(test_number(".0 ").unwrap(), 0.0);
        assert_eq!(test_number("0..9 "), None);
        assert_eq!(test_number("0.9.0 "), None);
        assert_eq!(test_number(".9 ").unwrap(), 0.9);
        assert_eq!(test_number("0. ").unwrap(), 0.0);
        assert_eq!(test_number("9. ").unwrap(), 9.0);
        assert_eq!(test_number(". ").unwrap(), 0.0);
        assert_eq!(test_number("-a "), None);
        assert_eq!(test_number("-1 ").unwrap(), -1.0);
        assert_eq!(test_number("-100 ").unwrap(), -100.0);
        assert_eq!(test_number("-1.0 ").unwrap(), -1.0);
        assert_eq!(test_number("-10.0 ").unwrap(), -10.0);
        assert_eq!(test_number("-.0 ").unwrap(), 0.0);
        assert_eq!(test_number("-0..9 "), None);
        assert_eq!(test_number("-0.9.0 "), None);
        assert_eq!(test_number("-.9 ").unwrap(), -0.9);
        assert_eq!(test_number("-0. ").unwrap(), -0.0);
        assert_eq!(test_number("-9. ").unwrap(), -9.0);
        assert_eq!(test_number("-. ").unwrap(), 0.0);
    }
}
