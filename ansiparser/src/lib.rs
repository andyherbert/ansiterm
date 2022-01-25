mod sequence_iterator;
use ansiplay::Music;
pub use codepage437::{ascii, CP437String};
pub use sauce::{Sauce, COMNT_HEAD, SAUCE_HEAD};
use sequence_iterator::IntoNumberSequenceIter;

enum State {
    Literal,
    Escape,
    EndOfFile(usize),
    Sequence(usize),
    SauceRecord {
        eof_start: usize,
        sauce_start: usize,
    },
    SauceComment {
        eof_start: usize,
        comments_start: usize,
    },
    Music(usize),
}

#[derive(Clone, Debug)]
pub enum Sequence {
    Literal(u8),
    CarriageReturn,
    LineFeed,
    Tab,
    CursorUp(usize),
    CursorDown(usize),
    CursorForward(usize),
    CursorBack(usize),
    CursorPosition { row: usize, column: usize },
    SetScreenMode(usize),
    ResetScreenMode(usize),
    EraseDisplay(usize),
    EraseInLine(usize),
    SelectGraphicsRendition(Vec<usize>),
    SavePosition,
    RestorePosition,
    SauceRecord(Box<Sauce>),
    PabloTrueColourBackground { red: u8, green: u8, blue: u8 },
    PabloTrueColourForeground { red: u8, green: u8, blue: u8 },
    Music(Music),
    Unknown { bytes: Vec<u8>, terminator: u8 },
    Update,
}

pub struct Parser {
    state: State,
    bytes: Vec<u8>,
    position: usize,
    baud_rate: Option<usize>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            state: State::Literal,
            bytes: vec![],
            position: 0,
            baud_rate: Some(14400),
        }
    }
}

impl Parser {
    pub fn new(baud_rate: impl Into<Option<usize>>) -> Self {
        Self {
            baud_rate: baud_rate.into(),
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
            if let Some(byte) = self.bytes.get(self.position) {
                self.position += 1;
                match self.state {
                    State::Literal => match *byte {
                        ascii::HORIZONTAL_TAB => return Some(Sequence::Tab),
                        ascii::LINE_FEED => return Some(Sequence::LineFeed),
                        ascii::CARRIAGE_RETURN => return Some(Sequence::CarriageReturn),
                        ascii::END_OF_FILE => self.state = State::EndOfFile(self.position),
                        ascii::ESCAPE => self.state = State::Escape,
                        _ => return Some(Sequence::Literal(*byte)),
                    },
                    State::Escape => match *byte {
                        ascii::LEFT_SQUARE_BRACKET => self.state = State::Sequence(self.position),
                        _ => {
                            self.state = State::Literal;
                            return Some(Sequence::Literal(*byte));
                        }
                    },
                    State::Sequence(start) => match *byte {
                        ascii::UPPERCASE_A => {
                            self.state = State::Literal;
                            let amount = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1)
                                .next()
                                .unwrap_or(1);
                            return Some(Sequence::CursorUp(amount));
                        }
                        ascii::UPPERCASE_B => {
                            self.state = State::Literal;
                            let amount = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1)
                                .next()
                                .unwrap_or(1);
                            return Some(Sequence::CursorDown(amount));
                        }
                        ascii::UPPERCASE_C => {
                            self.state = State::Literal;
                            let amount = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1)
                                .next()
                                .unwrap_or(1);
                            return Some(Sequence::CursorForward(amount));
                        }
                        ascii::UPPERCASE_D => {
                            self.state = State::Literal;
                            let amount = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1)
                                .next()
                                .unwrap_or(1);
                            return Some(Sequence::CursorBack(amount));
                        }
                        ascii::LOWERCASE_F | ascii::UPPERCASE_H => {
                            self.state = State::Literal;
                            let mut seq = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(1);
                            let row = seq.next().unwrap_or(1) - 1;
                            let column = seq.next().unwrap_or(1) - 1;
                            return Some(Sequence::CursorPosition { row, column });
                        }
                        ascii::LOWERCASE_H => {
                            self.state = State::Literal;
                            let mut seq = self.bytes[start..self.position - 1].into_sequence_iter();
                            if let Some(value) = seq.next() {
                                return Some(Sequence::SetScreenMode(value));
                            }
                        }
                        ascii::LOWERCASE_L => {
                            self.state = State::Literal;
                            let mut seq = self.bytes[start..self.position - 1].into_sequence_iter();
                            if let Some(value) = seq.next() {
                                return Some(Sequence::ResetScreenMode(value));
                            }
                        }
                        ascii::UPPERCASE_J => {
                            self.state = State::Literal;
                            let value = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(0)
                                .next()
                                .unwrap_or(0);
                            return Some(Sequence::EraseDisplay(value));
                        }
                        ascii::UPPERCASE_K => {
                            self.state = State::Literal;
                            let value = self.bytes[start..self.position - 1]
                                .into_sequence_iter_with_default(0)
                                .next()
                                .unwrap_or(0);
                            return Some(Sequence::EraseInLine(value));
                        }
                        ascii::UPPERCASE_M => {
                            self.state = State::Music(self.position - 1);
                        }
                        ascii::LOWERCASE_M => {
                            self.state = State::Literal;
                            let vec = self.bytes[start..self.position - 1]
                                .into_sequence_iter()
                                .collect();
                            return Some(Sequence::SelectGraphicsRendition(vec));
                        }
                        ascii::LOWERCASE_S => {
                            self.state = State::Literal;
                            return Some(Sequence::SavePosition);
                        }
                        ascii::LOWERCASE_T => {
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
                                            return Some(Sequence::PabloTrueColourBackground {
                                                red,
                                                green,
                                                blue,
                                            });
                                        }
                                        1 => {
                                            return Some(Sequence::PabloTrueColourForeground {
                                                red,
                                                green,
                                                blue,
                                            });
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        ascii::LOWERCASE_U => {
                            self.state = State::Literal;
                            return Some(Sequence::RestorePosition);
                        }
                        ascii::AT_SIGN..=ascii::TILDE => {
                            self.state = State::Literal;
                            return Some(Sequence::Unknown {
                                bytes: self.bytes[start..self.position - 1].to_vec(),
                                terminator: *byte,
                            });
                        }
                        _ => {}
                    },
                    State::EndOfFile(eof_start) => {
                        if self.position == eof_start + COMNT_HEAD.len()
                            && self.bytes[eof_start..self.position] == COMNT_HEAD
                        {
                            self.state = State::SauceComment {
                                eof_start,
                                comments_start: self.position,
                            };
                        } else if self.position == eof_start + SAUCE_HEAD.len() {
                            if self.bytes[eof_start..self.position] == SAUCE_HEAD {
                                self.state = State::SauceRecord {
                                    eof_start,
                                    sauce_start: eof_start,
                                };
                            } else {
                                self.state = State::Literal;
                                self.position -= SAUCE_HEAD.len();
                            }
                        }
                    }
                    State::SauceRecord {
                        eof_start,
                        sauce_start,
                    } => {
                        if self.position == sauce_start + 128 {
                            match Sauce::try_from(&self.bytes[eof_start..self.position]) {
                                Ok(sauce) => return Some(Sequence::SauceRecord(Box::new(sauce))),
                                Err(err) => eprintln!("{err}"),
                            }
                        }
                    }
                    State::SauceComment {
                        eof_start,
                        comments_start,
                    } => {
                        if (self.position - comments_start) % 64 == 0
                            && self.bytes[self.position..self.position + SAUCE_HEAD.len()]
                                == SAUCE_HEAD
                        {
                            self.state = State::SauceRecord {
                                eof_start,
                                sauce_start: self.position,
                            };
                        }
                    }
                    State::Music(start) => {
                        if *byte == ascii::SHIFT_OUT {
                            self.state = State::Literal;
                            let music = Music::new(&self.bytes[start..self.position - 1]);
                            return Some(Sequence::Music(music));
                        }
                    }
                }
                if let Some(baud_rate) = self.baud_rate {
                    if self.position % (baud_rate as f32 / 8.0 / 60.0) as usize == 0 {
                        return Some(Sequence::Update);
                    }
                }
            } else {
                return None;
            }
        }
    }
}

#[test]
fn test() {
    // let bytes = std::fs::read("/Users/andyh/src/ansimation.js/docs/ans/rad-PIRANHA.ANS").unwrap();
    let bytes = std::fs::read("/Users/andyh/src/ansimation.js/docs/ans/LD-TFGS.ANS").unwrap();
    let mut parser = Parser::new(None);
    parser.input(bytes);
    if let Some(Sequence::SauceRecord(sauce)) = parser.last() {
        println!("{sauce}");
    }
}
