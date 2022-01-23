use codepage437::{CP437Error, CP437String};
use serde::{de, Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::COMNT_HEADER;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Comments {
    strings: Vec<CP437String>,
}

impl Comments {
    pub fn new() -> Comments {
        Comments { strings: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Comments {
        Comments {
            strings: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.strings
            .push(CP437String::from(bytes).strip_trailing_spaces());
    }

    pub fn push(&mut self, string: &str) -> Result<(), CP437Error> {
        let string = CP437String::try_from(string)?;
        self.strings.push(string.strip_trailing_spaces());
        Ok(())
    }
}

impl<'a> IntoIterator for &'a Comments {
    type Item = &'a CP437String;
    type IntoIter = std::slice::Iter<'a, CP437String>;

    fn into_iter(self) -> Self::IntoIter {
        self.strings.iter()
    }
}

impl TryFrom<&Vec<String>> for Comments {
    type Error = CP437Error;

    fn try_from(strings: &Vec<String>) -> Result<Self, Self::Error> {
        let mut comments = Comments::with_capacity(strings.len());
        for string in strings {
            comments.push(string)?;
        }
        Ok(comments)
    }
}

impl fmt::Display for Comments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .strings
            .iter()
            .map(|cp437| cp437.pad_with_spaces(64).to_string())
            .collect::<String>();
        write!(f, "{}", string)
    }
}

impl From<&Comments> for Vec<u8> {
    fn from(comments: &Comments) -> Self {
        let mut bytes = vec![0; comments.len() * 64 + COMNT_HEADER.len()];
        bytes[0..COMNT_HEADER.len()].copy_from_slice(&COMNT_HEADER);
        let comment_bytes = comments
            .strings
            .iter()
            .map(|cp437| cp437.pad_with_spaces(64).as_slice().to_vec())
            .collect::<Vec<Vec<u8>>>()
            .concat();
        bytes[COMNT_HEADER.len()..].copy_from_slice(comment_bytes.as_slice());
        bytes
    }
}

impl FromStr for Comments {
    type Err = CP437Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strings = s
            .chars()
            .collect::<Vec<char>>()
            .chunks_exact(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>();
        let comments = Comments::try_from(&strings)?;
        Ok(comments)
    }
}

impl Serialize for Comments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

struct StringVisitor;

impl<'de> de::Visitor<'de> for StringVisitor {
    type Value = Comments;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Comments::from_str(value) {
            Ok(comments) => Ok(comments),
            Err(err) => Err(E::custom(err)),
        }
    }
}

impl<'de> Deserialize<'de> for Comments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[test]
fn test() {
    let string = "piranha bbs +o caphood                                          acidic whq, acid outpost, agoranet us eastern hub, dezign ushq  telnet://piranha.acid.org                                                                                                       suggested modem speed for this ansimation is 14400.                                                                             enjoy!                                                          ";
    let comments = Comments::from_str(string).unwrap();
    assert_eq!(comments.len(), 7);
    assert_eq!(comments.to_string(), string);
}
