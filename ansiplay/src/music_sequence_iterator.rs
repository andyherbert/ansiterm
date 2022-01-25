use crate::music::*;
use codepage437::ascii;

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
            match *byte {
                ascii::COMMA => amount += 1,
                ascii::SPACE => {}
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
            match *byte {
                ascii::UPPERCASE_B => Some(MusicOperation::Background),
                ascii::UPPERCASE_F => Some(MusicOperation::Foreground),
                ascii::UPPERCASE_L | ascii::LOWERCASE_L => {
                    Some(MusicOperation::Articulation(Articulation::Legato))
                }
                ascii::UPPERCASE_N | ascii::LOWERCASE_N => {
                    Some(MusicOperation::Articulation(Articulation::Normal))
                }
                ascii::UPPERCASE_S | ascii::LOWERCASE_S => {
                    Some(MusicOperation::Articulation(Articulation::Stacato))
                }
                ascii::SPACE => Some(MusicOperation::None),
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
            match *byte {
                ascii::DIGIT_0..=ascii::DIGIT_9 => {
                    if let Some(value) = &mut number {
                        *value = (*value * 10) + (*byte as usize - ascii::DIGIT_0 as usize);
                    } else {
                        number = Some(*byte as usize - ascii::DIGIT_0 as usize);
                    }
                }
                ascii::SEMI_COLON if accept_semi_colon => break,
                ascii::SPACE if accept_whitespace => {}
                _ => {
                    self.position -= 1;
                    break;
                }
            }
        }
        number
    }

    fn parse_sound_code_number(&mut self) -> Option<f32> {
        let minus = match self.bytes.get(self.position).copied() {
            Some(ascii::MINUS) => {
                self.position += 1;
                true
            }
            Some(ascii::SEMI_COLON) => {
                self.position += 1;
                return None;
            }
            Some(ascii::SPACE) => return None,
            _ => false,
        };
        // '0..9'
        let prefix = self.parse_int(false, false).unwrap_or(0);
        let postfix = if let Some(ascii::PERIOD) = self.bytes.get(self.position).copied() {
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
        match self.bytes.get(self.position).copied() {
            Some(ascii::SEMI_COLON) => self.position += 1,
            None | Some(ascii::SPACE) => {}
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
        match self.bytes.get(self.position).copied()? {
            ascii::ASTERISK => {
                self.position += 1;
                Some(Variation::Random)
            }
            _ => Some(Variation::Value(self.parse_sound_code_number()?)),
        }
    }

    fn parse_sign(&mut self) -> NoteSign {
        if let Some(byte) = self.bytes.get(self.position).copied() {
            self.position += 1;
            match byte {
                ascii::PLUS | ascii::NUMBER_SIGN => NoteSign::Sharp,
                ascii::MINUS => NoteSign::Flat,
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
        while let Some(byte) = self.bytes.get(self.position).copied() {
            self.position += 1;
            match byte {
                ascii::MINUS
                | ascii::PERIOD
                | ascii::DIGIT_0..=ascii::DIGIT_9
                | ascii::SEMI_COLON => {
                    self.position -= 1;
                    return Some(MusicEntity::SoundCode(self.parse_sound_code()?));
                }
                ascii::UPPERCASE_A | ascii::LOWERCASE_A => {
                    return Some(MusicEntity::Note {
                        note: Note::A,
                        info: self.parse_note_info(),
                    })
                }
                ascii::UPPERCASE_B | ascii::LOWERCASE_B => {
                    return Some(MusicEntity::Note {
                        note: Note::B,
                        info: self.parse_note_info(),
                    })
                }
                ascii::UPPERCASE_C | ascii::LOWERCASE_C => {
                    return Some(MusicEntity::Note {
                        note: Note::C,
                        info: self.parse_note_info(),
                    })
                }
                ascii::UPPERCASE_D | ascii::LOWERCASE_D => {
                    return Some(MusicEntity::Note {
                        note: Note::D,
                        info: self.parse_note_info(),
                    })
                }
                ascii::UPPERCASE_E | ascii::LOWERCASE_E => {
                    return Some(MusicEntity::Note {
                        note: Note::E,
                        info: self.parse_note_info(),
                    })
                }
                ascii::UPPERCASE_F | ascii::LOWERCASE_F => {
                    return Some(MusicEntity::Note {
                        note: Note::F,
                        info: self.parse_note_info(),
                    })
                }
                ascii::UPPERCASE_G | ascii::LOWERCASE_G => {
                    return Some(MusicEntity::Note {
                        note: Note::G,
                        info: self.parse_note_info(),
                    })
                }
                ascii::UPPERCASE_L | ascii::LOWERCASE_L => {
                    return self.parse_int(true, false).map(MusicEntity::Length)
                }
                ascii::UPPERCASE_M | ascii::LOWERCASE_M => {
                    return self.parse_operation().map(MusicEntity::Operation)
                }
                ascii::UPPERCASE_N | ascii::LOWERCASE_N => {
                    return self.parse_int(true, false).map(MusicEntity::RawNote)
                }
                ascii::UPPERCASE_O | ascii::LOWERCASE_O => {
                    return self.parse_int(true, false).map(MusicEntity::Octave)
                }
                ascii::UPPERCASE_P | ascii::LOWERCASE_P => {
                    return Some(MusicEntity::Pause(self.parse_int(true, false)?))
                }
                ascii::UPPERCASE_T | ascii::LOWERCASE_T => {
                    return self.parse_int(true, false).map(MusicEntity::Tempo)
                }
                ascii::LESS_THAN_SIGN => return Some(MusicEntity::DecreaseOctave),
                ascii::GREATER_THAN_SIGN => return Some(MusicEntity::IncreaseOctave),
                ascii::SPACE => {}
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
    use crate::MusicSequenceIterator;
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
