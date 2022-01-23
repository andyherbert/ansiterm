pub use std::{error, fmt};

#[derive(Debug)]
pub enum SauceError {
    HeaderMissing,
    CommentHeaderMissing,
    CommentsMissing,
    InvalidDataType,
    InvalidFileType,
    InvalidLetterSpacingValue,
    InvalidAspectRatioValue,
}

impl fmt::Display for SauceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SauceError::HeaderMissing => writeln!(f, "Header missing"),
            SauceError::CommentHeaderMissing => writeln!(f, "Comment header missing"),
            SauceError::InvalidDataType => writeln!(f, "Invalid datatype"),
            SauceError::InvalidFileType => writeln!(f, "Invalid filetype"),
            SauceError::InvalidLetterSpacingValue => writeln!(f, "Invalid letter spacing value"),
            SauceError::InvalidAspectRatioValue => writeln!(f, "Invalid aspect ratio value"),
            SauceError::CommentsMissing => writeln!(f, "Comments missing"),
        }
    }
}

impl error::Error for SauceError {}
