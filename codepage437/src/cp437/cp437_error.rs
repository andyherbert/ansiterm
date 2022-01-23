use std::{error, fmt};

/// Error to indicate a string cannot be represented with Codepage 437 glyphs
#[derive(Clone, Debug, PartialEq)]
pub enum CP437Error {
    NoEquivalentGlyph,
}

impl fmt::Display for CP437Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CP437Error::NoEquivalentGlyph => write!(f, "No equivalent glyph for character"),
        }
    }
}

impl error::Error for CP437Error {}
