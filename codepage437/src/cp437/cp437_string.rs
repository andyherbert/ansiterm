use crate::{ascii, cp437::CP437Char, cp437::CP437Error};
use serde::{de, Deserialize, Serialize};
use std::{fmt, slice::Iter, str::FromStr};

/// Structure to store a string represented by Codepage 437 glyphs
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CP437String {
    bytes: Vec<u8>,
    chars: Vec<CP437Char>,
}

impl CP437String {
    /// Construct a new string
    pub fn new() -> CP437String {
        CP437String::default()
    }

    /// Constructs a string with a capacity
    pub fn with_capacity(capacity: usize) -> CP437String {
        CP437String {
            bytes: Vec::with_capacity(capacity),
            chars: Vec::with_capacity(capacity),
        }
    }

    /// Returns the string as a slice of bytes
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the length of the string
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    /// Returns if the string is empty
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    fn strip_trailing_value(&self, value: u8) -> CP437String {
        let mut cp437 = self.clone();
        while let Some(last) = cp437.last() {
            if last.byte == value {
                cp437.pop();
            } else {
                break;
            }
        }
        cp437
    }

    /// Strips trailing spaces from the string
    pub fn strip_trailing_spaces(&self) -> CP437String {
        self.strip_trailing_value(ascii::SPACE)
    }

    /// Strips trailing null values from the string
    pub fn strip_trailing_nulls(&self) -> CP437String {
        self.strip_trailing_value(ascii::NULL)
    }

    fn pad_with_value(&self, length: usize, value: u8) -> CP437String {
        let mut cp437 = self.clone();
        while cp437.len() < length {
            cp437.push_byte(value);
        }
        cp437
    }

    /// Pads the string with trailing spaces, to a set length
    pub fn pad_with_spaces(&self, length: usize) -> CP437String {
        self.pad_with_value(length, 0x20)
    }

    /// Pads the string with trailing null values, to a set length
    pub fn pad_with_nulls(&self, length: usize) -> CP437String {
        self.pad_with_value(length, 0x0)
    }

    /// Pushes a new value (0-255) to the string
    pub fn push_byte(&mut self, byte: u8) {
        let ch = CP437Char::from(byte);
        self.bytes.push(byte);
        self.chars.push(ch);
    }

    fn push(&mut self, ch: CP437Char) {
        self.bytes.push(ch.byte);
        self.chars.push(ch);
    }

    fn pop(&mut self) -> Option<CP437Char> {
        if let Some(ch) = self.chars.pop() {
            self.bytes.pop();
            Some(ch)
        } else {
            None
        }
    }

    fn last(&self) -> Option<&CP437Char> {
        self.chars.last()
    }

    fn chars(&self) -> Iter<CP437Char> {
        self.chars.iter()
    }
}

impl From<Vec<u8>> for CP437String {
    fn from(bytes: Vec<u8>) -> Self {
        let mut cp437 = CP437String::with_capacity(bytes.len());
        for byte in bytes {
            cp437.push_byte(byte);
        }
        cp437
    }
}

impl FromStr for CP437String {
    type Err = CP437Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cp437_string = CP437String::with_capacity(s.len());
        for ch in s.chars() {
            match CP437Char::try_from(ch) {
                Ok(ch) => cp437_string.push(ch),
                Err(err) => return Err(err),
            }
        }
        Ok(cp437_string)
    }
}

impl From<&[u8]> for CP437String {
    fn from(bytes: &[u8]) -> Self {
        CP437String::from(bytes.to_vec())
    }
}

impl From<&CP437String> for String {
    fn from(cp_437: &CP437String) -> Self {
        cp_437.chars().map(|cp_char| cp_char.ch).collect::<String>()
    }
}

impl fmt::Display for CP437String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{string}", string = String::from(self))
    }
}

impl Serialize for CP437String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

struct StringVisitor;

impl<'de> de::Visitor<'de> for StringVisitor {
    type Value = CP437String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match CP437String::from_str(value) {
            Ok(cp437) => Ok(cp437),
            Err(err) => Err(E::custom(err)),
        }
    }
}

impl<'de> Deserialize<'de> for CP437String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::CP437String;
    use serde_json::to_string;
    use std::str::FromStr;
    #[test]
    fn test() {
        let a = CP437String::from_str("☺☻♥♦♣♠").expect("cp437 string");
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let b = CP437String::from(bytes.as_slice());
        assert_eq!(a, b);
        assert_eq!(to_string(&a).unwrap(), "\"☺☻♥♦♣♠\"");
    }
}
