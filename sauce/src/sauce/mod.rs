mod aspect_ratio;
mod comments;
mod datatype;
mod filetype;
mod info_s;
mod letter_spacing;
mod sauce_error;
pub use aspect_ratio::AspectRatio;
use codepage437::{ascii, CP437String};
pub use comments::Comments;
pub use datatype::DataType;
pub use filetype::FileType;
pub use info_s::InfoS;
pub use letter_spacing::LetterSpacing;
pub use sauce_error::SauceError;
use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

pub const SAUCE_HEAD: [u8; 7] = [
    ascii::UPPERCASE_S,
    ascii::UPPERCASE_A,
    ascii::UPPERCASE_U,
    ascii::UPPERCASE_C,
    ascii::UPPERCASE_E,
    ascii::DIGIT_0,
    ascii::DIGIT_0,
];

pub const COMNT_HEAD: [u8; 5] = [
    ascii::UPPERCASE_C,
    ascii::UPPERCASE_O,
    ascii::UPPERCASE_M,
    ascii::UPPERCASE_N,
    ascii::UPPERCASE_T,
];

fn pack(mut value: usize, bytes: &mut [u8]) {
    bytes.iter_mut().for_each(|byte| {
        *byte = (value & 255) as u8;
        value >>= 8;
    });
}

fn unpack(bytes: &[u8]) -> usize {
    let mut value: usize = 0;
    for byte in bytes.iter().rev() {
        value <<= 8;
        value += *byte as usize;
    }
    value
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Sauce {
    pub title: CP437String,
    pub author: CP437String,
    pub group: CP437String,
    pub year: CP437String,
    pub month: CP437String,
    pub date: CP437String,
    pub filesize: usize,
    pub datatype: DataType,
    pub filetype: FileType,
    pub info_1: usize,
    pub info_2: usize,
    pub info_3: usize,
    pub info_4: usize,
    pub ice_colors: bool,
    pub letter_spacing: LetterSpacing,
    pub aspect_ratio: AspectRatio,
    pub info_s: InfoS,
    pub comments: Comments,
    pub path: Option<PathBuf>,
}

impl Sauce {
    pub fn new() -> Sauce {
        Sauce::default()
    }

    pub fn read(path: impl AsRef<Path>) -> Result<Option<Sauce>, SauceError> {
        match fs::read(&path) {
            Ok(bytes) => match Sauce::try_from(bytes.as_slice()) {
                Ok(mut sauce) => {
                    sauce.path = Some(path.as_ref().to_owned());
                    let filesize = bytes.len() - sauce.size();
                    if sauce.filesize != filesize {
                        sauce.filesize = filesize;
                    }
                    Ok(Some(sauce))
                }
                Err(SauceError::NoSauce) => Ok(None),
                Err(err) => Err(err),
            },
            Err(_err) => Err(SauceError::FileReadError(
                path.as_ref().to_string_lossy().to_string(),
            )),
        }
    }

    pub fn remove(path: impl AsRef<Path>) -> Result<(), SauceError> {
        match fs::read(&path) {
            Err(_err) => Err(SauceError::FileReadError(
                path.as_ref().to_string_lossy().to_string(),
            )),
            Ok(mut file_bytes) => match Sauce::try_from(file_bytes.as_slice()) {
                Err(SauceError::NoSauce) => Ok(()),
                Err(err) => Err(err),
                Ok(sauce) => {
                    file_bytes.resize(file_bytes.len() - sauce.size(), 0);
                    match fs::write(&path, file_bytes) {
                        Ok(()) => Ok(()),
                        Err(_err) => Err(SauceError::FileWriteError(
                            path.as_ref().to_string_lossy().to_string(),
                        )),
                    }
                }
            },
        }
    }

    pub fn write(&self, path: impl AsRef<Path>) -> Result<(), SauceError> {
        let sauce_bytes = Vec::from(self);
        match fs::read(&path) {
            Err(_err) => Err(SauceError::FileReadError(
                path.as_ref().to_string_lossy().to_string(),
            )),
            Ok(mut file_bytes) => {
                if let Ok(sauce) = Sauce::try_from(file_bytes.as_slice()) {
                    file_bytes.resize(file_bytes.len() - sauce.size(), 0);
                }
                file_bytes.extend(sauce_bytes);
                match fs::write(&path, file_bytes) {
                    Err(_err) => Err(SauceError::FileWriteError(
                        path.as_ref().to_string_lossy().to_string(),
                    )),
                    Ok(()) => Ok(()),
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        if self.comments.is_empty() {
            128 + 1
        } else {
            128 + (self.comments.len() * 64) + COMNT_HEAD.len() + 1
        }
    }
}

impl TryFrom<&[u8]> for Sauce {
    type Error = SauceError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 128 {
            return Err(SauceError::NoSauce);
        }
        let sauce_start = bytes.len() - 128;
        match bytes.get(sauce_start..) {
            Some(sauce_bytes) if sauce_bytes[..SAUCE_HEAD.len()] == SAUCE_HEAD => {
                let comments_len = sauce_bytes[104] as usize;
                let mut comments = Comments::with_capacity(comments_len);
                if comments_len > 0 {
                    let comments_size = comments_len * 64 + COMNT_HEAD.len();
                    if comments_size > sauce_start {
                        return Err(SauceError::CommentsMissing);
                    }
                    match bytes.get(sauce_start - comments_size..sauce_start) {
                        Some(comment_bytes) if comment_bytes[0..COMNT_HEAD.len()] == COMNT_HEAD => {
                            for bytes in comment_bytes[COMNT_HEAD.len()..].chunks_exact(64) {
                                comments.push_bytes(bytes);
                            }
                        }
                        _ => return Err(SauceError::CommentsMissing),
                    };
                }
                let datatype = DataType::try_from(sauce_bytes[94])?;
                Ok(Sauce {
                    path: None,
                    title: CP437String::from(&sauce_bytes[7..=41]).strip_trailing_spaces(),
                    author: CP437String::from(&sauce_bytes[42..=61]).strip_trailing_spaces(),
                    group: CP437String::from(&sauce_bytes[62..=81]).strip_trailing_spaces(),
                    year: CP437String::from(&sauce_bytes[82..=85]),
                    month: CP437String::from(&sauce_bytes[86..=87]),
                    date: CP437String::from(&sauce_bytes[88..=89]),
                    filesize: unpack(&sauce_bytes[90..=93]),
                    filetype: datatype.get_filetype(sauce_bytes[95])?,
                    datatype,
                    info_1: unpack(&sauce_bytes[96..=97]),
                    info_2: unpack(&sauce_bytes[98..=99]),
                    info_3: unpack(&sauce_bytes[100..=101]),
                    info_4: unpack(&sauce_bytes[102..=103]),
                    ice_colors: (sauce_bytes[105] & 1) == 1,
                    letter_spacing: LetterSpacing::from(sauce_bytes[105]),
                    aspect_ratio: AspectRatio::from(sauce_bytes[105]),
                    info_s: InfoS::from(
                        &CP437String::from(&sauce_bytes[106..=127]).strip_trailing_nulls(),
                    ),
                    comments,
                })
            }
            _ => Err(SauceError::NoSauce),
        }
    }
}

impl From<&Sauce> for Vec<u8> {
    fn from(sauce: &Sauce) -> Self {
        let mut bytes = if sauce.comments.is_empty() {
            vec![0; 129]
        } else {
            let comment_bytes = Vec::from(&sauce.comments);
            let mut bytes = vec![0; 129 + comment_bytes.len()];
            bytes[1..=comment_bytes.len()].copy_from_slice(&comment_bytes);
            bytes
        };
        bytes[0] = ascii::END_OF_FILE;
        let sauce_start = bytes.len() - 128;
        let sauce_bytes = &mut bytes[sauce_start..];
        sauce_bytes[0..=6].copy_from_slice(&SAUCE_HEAD);
        sauce_bytes[7..=41].copy_from_slice(sauce.title.pad_with_spaces(35).as_slice());
        sauce_bytes[42..=61].copy_from_slice(sauce.author.pad_with_spaces(20).as_slice());
        sauce_bytes[62..=81].copy_from_slice(sauce.group.pad_with_spaces(20).as_slice());
        sauce_bytes[82..=85].copy_from_slice(sauce.year.as_slice());
        sauce_bytes[86..=87].copy_from_slice(sauce.month.as_slice());
        sauce_bytes[88..=89].copy_from_slice(sauce.date.as_slice());
        pack(sauce.filesize, &mut sauce_bytes[90..=93]);
        sauce_bytes[94] = u8::from(&sauce.datatype);
        sauce_bytes[95] = u8::from(&sauce.filetype);
        pack(sauce.info_1, &mut sauce_bytes[96..=97]);
        pack(sauce.info_2, &mut sauce_bytes[98..=99]);
        pack(sauce.info_3, &mut sauce_bytes[100..=101]);
        pack(sauce.info_4, &mut sauce_bytes[102..=103]);
        sauce_bytes[104] = sauce.comments.len() as u8;
        if sauce.ice_colors {
            sauce_bytes[105] = 1;
        }
        sauce_bytes[105] += u8::from(&sauce.letter_spacing);
        sauce_bytes[105] += u8::from(&sauce.aspect_ratio);
        sauce_bytes[106..=127].copy_from_slice(CP437String::from(&sauce.info_s).as_slice());
        bytes
    }
}

impl fmt::Display for Sauce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "title: {}", self.title)?;
        writeln!(f, "author: {}", self.author)?;
        writeln!(f, "group: {}", self.group)?;
        writeln!(f, "year: {}", self.year)?;
        writeln!(f, "month: {}", self.month)?;
        writeln!(f, "date: {}", self.date)?;
        writeln!(f, "filesize: {}", self.filesize)?;
        writeln!(f, "datatype: {}", self.datatype)?;
        writeln!(f, "filetype: {}", self.filetype)?;
        writeln!(f, "type info 1: {}", self.info_1)?;
        writeln!(f, "type info 2: {}", self.info_2)?;
        writeln!(f, "type info 3: {}", self.info_3)?;
        writeln!(f, "type info 4: {}", self.info_4)?;
        writeln!(f, "ice colors: {}", self.ice_colors)?;
        writeln!(f, "letter spacing: {}", self.letter_spacing)?;
        writeln!(f, "aspect ratio: {}", self.aspect_ratio)?;
        writeln!(f, "info string: {}", self.info_s)?;
        writeln!(f, "comments:")?;
        for comment in &self.comments {
            writeln!(f, "{comment}")?;
        }
        if let Some(ref path) = self.path {
            if let Some(path) = path.as_path().to_str() {
                writeln!(f, "path: {path}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test() {
        let sauce = Sauce::read("/Users/andyh/src/ansimation.js/docs/ans/LD-TFGS.ANS")
            .unwrap()
            .unwrap();
        println!("{}", serde_json::to_string_pretty(&sauce).unwrap());
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        wtr.serialize(&sauce).unwrap();
        wtr.flush().unwrap();
        let sauce_vec = Vec::from(&sauce);
        let bytes = std::fs::read("/Users/andyh/src/ansimation.js/docs/ans/LD-TFGS.ANS").unwrap();
        let sauce_start = bytes.len() - sauce_vec.len();
        assert_eq!(bytes[sauce_start..], sauce_vec);
        dbg!(sauce_vec);
    }
}
