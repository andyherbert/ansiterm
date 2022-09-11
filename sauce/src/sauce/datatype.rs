use crate::{sauce::FileType, SauceError};
use serde::{Deserialize, Serialize};
pub use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum DataType {
    None,
    Character,
    Bitmap,
    Vector,
    Audio,
    BinaryText,
    XBin,
    Archive,
    Executable,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::None
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::None => write!(f, "None"),
            DataType::Character => write!(f, "Character"),
            DataType::Bitmap => write!(f, "Bitmap"),
            DataType::Vector => write!(f, "Vector"),
            DataType::Audio => write!(f, "Audio"),
            DataType::BinaryText => write!(f, "Binary Text"),
            DataType::XBin => write!(f, "XBin"),
            DataType::Archive => write!(f, "Archive"),
            DataType::Executable => write!(f, "Executable"),
        }
    }
}

impl DataType {
    pub fn get_filetype(&self, value: u8) -> Result<FileType, SauceError> {
        match self {
            DataType::Executable | DataType::None => {
                if value != 0 {
                    Err(SauceError::InvalidFileType)
                } else {
                    Ok(FileType::None)
                }
            }
            DataType::Character => match value {
                0 => Ok(FileType::Ascii),
                1 => Ok(FileType::Ansi),
                2 => Ok(FileType::Ansimation),
                3 => Ok(FileType::RipScript),
                4 => Ok(FileType::PcBoard),
                5 => Ok(FileType::Avatar),
                6 => Ok(FileType::Html),
                7 => Ok(FileType::Source),
                8 => Ok(FileType::TundraDraw),
                _ => Err(SauceError::InvalidFileType),
            },
            DataType::Bitmap => match value {
                0 => Ok(FileType::Gif),
                1 => Ok(FileType::Pcx),
                2 => Ok(FileType::LbmOrIff),
                3 => Ok(FileType::Tga),
                4 => Ok(FileType::Fli),
                5 => Ok(FileType::Flc),
                6 => Ok(FileType::Bmp),
                7 => Ok(FileType::Gl),
                8 => Ok(FileType::Dl),
                9 => Ok(FileType::WpgBitmap),
                10 => Ok(FileType::Png),
                11 => Ok(FileType::Jpg),
                12 => Ok(FileType::Mpg),
                13 => Ok(FileType::Avi),
                _ => Err(SauceError::InvalidFileType),
            },
            DataType::Vector => match value {
                0 => Ok(FileType::Dxf),
                1 => Ok(FileType::Dwg),
                2 => Ok(FileType::WpgVector),
                3 => Ok(FileType::Studio3ds),
                _ => Err(SauceError::InvalidFileType),
            },
            DataType::Audio => match value {
                0 => Ok(FileType::Mod),
                1 => Ok(FileType::Renaissance669),
                2 => Ok(FileType::Stm),
                3 => Ok(FileType::S3m),
                4 => Ok(FileType::Mtm),
                5 => Ok(FileType::Far),
                6 => Ok(FileType::Ult),
                7 => Ok(FileType::Amf),
                8 => Ok(FileType::Dmf),
                9 => Ok(FileType::Okt),
                10 => Ok(FileType::Rol),
                11 => Ok(FileType::Cmf),
                12 => Ok(FileType::Mid),
                13 => Ok(FileType::Sadt),
                14 => Ok(FileType::Voc),
                15 => Ok(FileType::Wav),
                16 => Ok(FileType::Smp8),
                17 => Ok(FileType::Smp8s),
                18 => Ok(FileType::Smp16),
                19 => Ok(FileType::Smp16s),
                20 => Ok(FileType::Patch8),
                21 => Ok(FileType::Patch16),
                22 => Ok(FileType::Xm),
                23 => Ok(FileType::Hsc),
                24 => Ok(FileType::It),
                _ => Err(SauceError::InvalidFileType),
            },
            DataType::BinaryText => Ok(FileType::Variable(value)),
            DataType::XBin => {
                if value != 0 {
                    Err(SauceError::InvalidFileType)
                } else {
                    Ok(FileType::Xbin)
                }
            }
            DataType::Archive => match value {
                0 => Ok(FileType::Zip),
                1 => Ok(FileType::Arj),
                2 => Ok(FileType::Lzh),
                3 => Ok(FileType::Arc),
                4 => Ok(FileType::Tar),
                5 => Ok(FileType::Zoo),
                6 => Ok(FileType::Rar),
                7 => Ok(FileType::Uc2),
                8 => Ok(FileType::Pak),
                9 => Ok(FileType::Sqz),
                _ => Err(SauceError::InvalidFileType),
            },
        }
    }
}

impl From<&DataType> for u8 {
    fn from(data_type: &DataType) -> Self {
        match data_type {
            DataType::None => 0,
            DataType::Character => 1,
            DataType::Bitmap => 2,
            DataType::Vector => 3,
            DataType::Audio => 4,
            DataType::BinaryText => 5,
            DataType::XBin => 6,
            DataType::Archive => 7,
            DataType::Executable => 8,
        }
    }
}

impl FromStr for DataType {
    type Err = SauceError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "None" => Ok(DataType::None),
            "Character" => Ok(DataType::Character),
            "Bitmap" => Ok(DataType::Bitmap),
            "Vector" => Ok(DataType::Vector),
            "Audio" => Ok(DataType::Audio),
            "Binary Text" => Ok(DataType::BinaryText),
            "XBin" => Ok(DataType::XBin),
            "Archive" => Ok(DataType::Archive),
            "Executable" => Ok(DataType::Executable),
            _ => Err(SauceError::InvalidDataType),
        }
    }
}

impl TryFrom<u8> for DataType {
    type Error = SauceError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DataType::None),
            1 => Ok(DataType::Character),
            2 => Ok(DataType::Bitmap),
            3 => Ok(DataType::Vector),
            4 => Ok(DataType::Audio),
            5 => Ok(DataType::BinaryText),
            6 => Ok(DataType::XBin),
            7 => Ok(DataType::Archive),
            8 => Ok(DataType::Executable),
            _ => Err(SauceError::InvalidDataType),
        }
    }
}
