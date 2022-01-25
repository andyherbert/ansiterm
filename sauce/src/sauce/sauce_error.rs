pub use std::{error, fmt};

#[derive(Debug)]
pub enum SauceError {
    NoSauce,
    CommentHeaderMissing,
    CommentsMissing,
    InvalidDataType,
    InvalidFileType,
    InvalidLetterSpacingValue,
    InvalidAspectRatioValue,
    FileReadError(String),
    FileWriteError(String),
}

impl fmt::Display for SauceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SauceError::NoSauce => writeln!(f, "Header missing"),
            SauceError::CommentHeaderMissing => writeln!(f, "Comment header missing"),
            SauceError::InvalidDataType => writeln!(f, "Invalid datatype"),
            SauceError::InvalidFileType => writeln!(f, "Invalid filetype"),
            SauceError::InvalidLetterSpacingValue => writeln!(f, "Invalid letter spacing value"),
            SauceError::InvalidAspectRatioValue => writeln!(f, "Invalid aspect ratio value"),
            SauceError::CommentsMissing => writeln!(f, "Comments missing"),
            SauceError::FileReadError(path) => {
                writeln!(f, "An error occured whilst reading the file: {path}")
            }
            SauceError::FileWriteError(path) => {
                writeln!(f, "An error occured whilst writing the file: {path}")
            }
        }
    }
}

impl error::Error for SauceError {}
