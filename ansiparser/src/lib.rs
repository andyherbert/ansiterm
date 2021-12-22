mod sequence_iterator;
use ansiplay::{IntoMusicalSequenceIter, MusicalEntity};
use sequence_iterator::IntoNumberSequenceIter;

const SAUCE_HEADER: [u8; 7] = [0x53, 0x41, 0x55, 0x43, 0x45, 0x30, 0x30];
const COMNT_HEADER: [u8; 5] = [0x43, 0x4F, 0x4D, 0x4E, 0x54];

enum State {
    Literal,
    Escape,
    EndOfFile(usize),
    Sequence(usize),
    SauceRecord(usize),
    SauceComment(usize),
    Music(usize),
}

pub enum Sequence {
    Literal(u8),
    CarriageReturn,
    LineFeed,
    Tab,
    CursorUp(usize),
    CursorDown(usize),
    CursorForward(usize),
    CursorBack(usize),
    CursorPosition(usize, usize),
    SetScreenMode(usize),
    ResetScreenMode(usize),
    EraseDisplay(usize),
    EraseInLine(usize),
    SelectGraphicsRendition(Vec<usize>),
    SavePosition,
    RestorePosition,
    SauceRecord(Vec<u8>),
    SauceComment(Vec<u8>),
    PabloTrueColourBackground(u8, u8, u8),
    PabloTrueColourForeground(u8, u8, u8),
    Music(Vec<MusicalEntity>),
    Unknown(Vec<u8>, u8),
    Update,
}

pub struct Parser {
    state: State,
    bytes: Vec<u8>,
    position: usize,
    baud_rate: usize,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            state: State::Literal,
            bytes: vec![],
            position: 0,
            baud_rate: 14400,
        }
    }
}

impl Parser {
    pub fn new(baud_rate: usize) -> Self {
        Self {
            baud_rate,
            ..Default::default()
        }
    }

    pub fn input(&mut self, mut bytes: Vec<u8>) {
        self.bytes.append(&mut bytes);
    }
}

impl Iterator for Parser {
    type Item = Sequence;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.position == self.bytes.len() {
                return None;
            } else {
                let byte = self.bytes[self.position];
                self.position += 1;
                match self.state {
                    State::Literal => match byte {
                        // Tab
                        0x09 => return Some(Sequence::Tab),
                        // LineFeed
                        0x0a => return Some(Sequence::LineFeed),
                        // CarriageReturn
                        0x0d => return Some(Sequence::CarriageReturn),
                        // EOF
                        0x1a => self.state = State::EndOfFile(self.position),
                        // Esc
                        0x1b => self.state = State::Escape,
                        _ => return Some(Sequence::Literal(byte)),
                    },
                    State::Escape => match byte {
                        // '['
                        0x5b => self.state = State::Sequence(self.position),
                        _ => {
                            self.state = State::Literal;
                            return Some(Sequence::Literal(byte));
                        }
                    },
                    State::Sequence(start) => match byte {
                        // 'A'
                        0x41 => {
                            self.state = State::Literal;
                            let amount = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1)
                                .next()
                                .unwrap_or(1);
                            return Some(Sequence::CursorUp(amount));
                        }
                        // 'B'
                        0x42 => {
                            self.state = State::Literal;
                            let amount = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1)
                                .next()
                                .unwrap_or(1);
                            return Some(Sequence::CursorDown(amount));
                        }
                        // 'C'
                        0x43 => {
                            self.state = State::Literal;
                            let amount = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1)
                                .next()
                                .unwrap_or(1);
                            return Some(Sequence::CursorForward(amount));
                        }
                        // 'D'
                        0x44 => {
                            self.state = State::Literal;
                            let amount = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1)
                                .next()
                                .unwrap_or(1);
                            return Some(Sequence::CursorBack(amount));
                        }
                        // 'f' | 'H'
                        0x66 | 0x48 => {
                            self.state = State::Literal;
                            let mut seq = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1);
                            let row = seq.next().unwrap_or(1);
                            let column = seq.next().unwrap_or(1);
                            return Some(Sequence::CursorPosition(row - 1, column - 1));
                        }
                        // 'h'
                        0x68 => {
                            self.state = State::Literal;
                            let mut seq = self.bytes[start..self.position - 1].into_sequence_iter();
                            if let Some(value) = seq.next() {
                                return Some(Sequence::SetScreenMode(value));
                            }
                        }
                        // 'l'
                        0x6c => {
                            self.state = State::Literal;
                            let mut seq = self.bytes[start..self.position - 1].into_sequence_iter();
                            if let Some(value) = seq.next() {
                                return Some(Sequence::ResetScreenMode(value));
                            }
                        }
                        // 'J'
                        0x4a => {
                            self.state = State::Literal;
                            let value = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(0)
                                .next()
                                .unwrap_or(0);
                            return Some(Sequence::EraseDisplay(value));
                        }
                        // 'K'
                        0x4b => {
                            self.state = State::Literal;
                            let value = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(0)
                                .next()
                                .unwrap_or(0);
                            return Some(Sequence::EraseInLine(value));
                        }
                        // 'M'
                        0x4d => {
                            self.state = State::Music(self.position - 1);
                        }
                        // 'm'
                        0x6d => {
                            self.state = State::Literal;
                            let vec = self.bytes[start..self.position - 1]
                                .into_sequence_iter()
                                .collect();
                            return Some(Sequence::SelectGraphicsRendition(vec));
                        }
                        // 's'
                        0x73 => {
                            self.state = State::Literal;
                            return Some(Sequence::SavePosition);
                        }
                        // 't'
                        0x74 => {
                            self.state = State::Literal;
                            let mut iter =
                                self.bytes[start..self.position - 1].into_sequence_iter();
                            if let (Some(fg_or_bg), Some(red), Some(green), Some(blue)) =
                                (iter.next(), iter.next(), iter.next(), iter.next())
                            {
                                if let (Ok(red), Ok(green), Ok(blue)) =
                                    (u8::try_from(red), u8::try_from(green), u8::try_from(blue))
                                {
                                    match fg_or_bg {
                                        0 => {
                                            return Some(Sequence::PabloTrueColourBackground(
                                                red, green, blue,
                                            ));
                                        }
                                        1 => {
                                            return Some(Sequence::PabloTrueColourForeground(
                                                red, green, blue,
                                            ));
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        // 'u'
                        0x75 => {
                            self.state = State::Literal;
                            return Some(Sequence::RestorePosition);
                        }
                        // '@'..='~'
                        0x40..=0x7e => {
                            self.state = State::Literal;
                            return Some(Sequence::Unknown(
                                self.bytes[start..self.position - 1].to_vec(),
                                byte,
                            ));
                        }
                        _ => {}
                    },
                    State::EndOfFile(start) => {
                        if self.position == start + COMNT_HEADER.len()
                            && self.bytes[start..self.position] == COMNT_HEADER
                        {
                            self.state = State::SauceComment(self.position);
                        } else if self.position == start + SAUCE_HEADER.len() {
                            if self.bytes[start..self.position] == SAUCE_HEADER {
                                self.state = State::SauceRecord(start);
                            } else {
                                self.state = State::Literal;
                                self.position -= 7;
                            }
                        }
                    }
                    State::SauceRecord(start) => {
                        if self.position == start + 128 {
                            return Some(Sequence::SauceRecord(
                                self.bytes[start..self.position].to_vec(),
                            ));
                        }
                    }
                    State::SauceComment(start) => {
                        if self.position == start + 64 {
                            if self.bytes[start..start + SAUCE_HEADER.len()] == SAUCE_HEADER {
                                self.state = State::SauceRecord(start);
                            } else {
                                self.state = State::SauceComment(self.position);
                                return Some(Sequence::SauceComment(
                                    self.bytes[start..self.position].to_vec(),
                                ));
                            }
                        }
                    }
                    State::Music(start) => {
                        if byte == 0x0e {
                            self.state = State::Literal;
                            return Some(Sequence::Music(
                                self.bytes[start..self.position - 1]
                                    .into_musical_sequence_iter()
                                    .collect(),
                            ));
                        }
                    }
                }
                if self.position % (self.baud_rate as f32 / 8.0 / 60.0) as usize == 0 {
                    return Some(Sequence::Update);
                }
            }
        }
    }
}
