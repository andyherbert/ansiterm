use std::{error, fmt};

/// Custom error used when generating, reading, and writing fonts
#[derive(Clone, Debug, PartialEq)]
pub enum FontError {
    CouldNotLoadFont,
    CouldNotLocateFont,
    IllegalFontSize,
    IllegalFontHeight,
    CannotReadFile,
    CannotWriteFile,
    CannotReadImage,
    CannotWriteImage,
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontError::CouldNotLoadFont => write!(f, "Could not load font"),
            FontError::CouldNotLocateFont => write!(f, "Could not locate font"),
            FontError::IllegalFontSize => write!(f, "Illegal font size"),
            FontError::IllegalFontHeight => write!(f, "Illegal font height"),
            FontError::CannotReadFile => write!(f, "Cannot read file"),
            FontError::CannotWriteFile => write!(f, "Cannot write file"),
            FontError::CannotReadImage => write!(f, "Cannot read image"),
            FontError::CannotWriteImage => write!(f, "Cannot write image"),
        }
    }
}

impl error::Error for FontError {}
