use codepage437::ascii;

enum ParsedNumber {
    Some(usize),
    Invalid,
    None,
}

pub struct NumberSequenceIterator<'a> {
    bytes: &'a [u8],
    position: usize,
    default: Option<usize>,
}

impl<'a> Iterator for NumberSequenceIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_number = ParsedNumber::None;
        while let Some(byte) = self.bytes.get(self.position) {
            self.position += 1;
            current_number = match *byte {
                ascii::DIGIT_0..=ascii::DIGIT_9 => {
                    if let ParsedNumber::Some(value) = current_number {
                        ParsedNumber::Some((value * 10) + (byte - ascii::DIGIT_0) as usize)
                    } else {
                        ParsedNumber::Some((byte - ascii::DIGIT_0) as usize)
                    }
                }
                ascii::SEMI_COLON => match current_number {
                    ParsedNumber::Some(value) => return Some(value),
                    ParsedNumber::Invalid => return self.next(),
                    ParsedNumber::None => match self.default {
                        Some(value) => return Some(value),
                        None => ParsedNumber::None,
                    },
                },
                _ => ParsedNumber::Invalid,
            }
        }
        if let ParsedNumber::Some(current_number) = current_number {
            Some(current_number)
        } else {
            None
        }
    }
}

pub trait IntoNumberSequenceIter<'a> {
    fn into_sequence_iter(self) -> NumberSequenceIterator<'a>;
    fn into_sequence_iter_with_default(self, default: usize) -> NumberSequenceIterator<'a>;
}

impl<'a> IntoNumberSequenceIter<'a> for &'a [u8] {
    fn into_sequence_iter(self) -> NumberSequenceIterator<'a> {
        NumberSequenceIterator {
            bytes: self,
            position: 0,
            default: None,
        }
    }

    fn into_sequence_iter_with_default(self, default: usize) -> NumberSequenceIterator<'a> {
        NumberSequenceIterator {
            bytes: self,
            position: 0,
            default: Some(default),
        }
    }
}
