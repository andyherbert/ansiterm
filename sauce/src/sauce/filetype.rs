use crate::SauceError;
use regex::Regex;
use serde::{de, Deserialize, Serialize};
pub use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub enum FileType {
    None,
    Ascii,
    Ansi,
    Ansimation,
    RipScript,
    PcBoard,
    Avatar,
    Html,
    Source,
    TundraDraw,
    Gif,
    Pcx,
    LbmOrIff,
    Tga,
    Fli,
    Flc,
    Bmp,
    Gl,
    Dl,
    WpgBitmap,
    Png,
    Jpg,
    Mpg,
    Avi,
    Dxf,
    Dwg,
    WpgVector,
    Studio3ds,
    Mod,
    Renaissance669,
    Stm,
    S3m,
    Mtm,
    Far,
    Ult,
    Amf,
    Dmf,
    Okt,
    Rol,
    Cmf,
    Mid,
    Sadt,
    Voc,
    Wav,
    Smp8,
    Smp8s,
    Smp16,
    Smp16s,
    Patch8,
    Patch16,
    Xm,
    Hsc,
    It,
    Variable(u8),
    Xbin,
    Zip,
    Arj,
    Lzh,
    Arc,
    Tar,
    Zoo,
    Rar,
    Uc2,
    Pak,
    Sqz,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::None => write!(f, "None"),
            FileType::Ascii => write!(f, "ASCII"),
            FileType::Ansi => write!(f, "ANSI"),
            FileType::Ansimation => write!(f, "ANSImation"),
            FileType::RipScript => write!(f, "RIPScript"),
            FileType::PcBoard => write!(f, "PCBoard"),
            FileType::Avatar => write!(f, "Avatar"),
            FileType::Html => write!(f, "HTML"),
            FileType::Source => write!(f, "Source"),
            FileType::TundraDraw => write!(f, "TundraDraw"),
            FileType::Gif => write!(f, "GIF"),
            FileType::Pcx => write!(f, "PCX"),
            FileType::LbmOrIff => write!(f, "LBM/IFF"),
            FileType::Tga => write!(f, "TGA"),
            FileType::Fli => write!(f, "FLI"),
            FileType::Flc => write!(f, "FLC"),
            FileType::Bmp => write!(f, "BMP"),
            FileType::Gl => write!(f, "GL"),
            FileType::Dl => write!(f, "DL"),
            FileType::WpgBitmap => write!(f, "WPG Bitmap"),
            FileType::Png => write!(f, "PNG"),
            FileType::Jpg => write!(f, "JPG"),
            FileType::Mpg => write!(f, "MPG"),
            FileType::Avi => write!(f, "AVI"),
            FileType::Dxf => write!(f, "DXF"),
            FileType::Dwg => write!(f, "DWG"),
            FileType::WpgVector => write!(f, "WPG Vector"),
            FileType::Studio3ds => write!(f, "3DS"),
            FileType::Mod => write!(f, "MOD"),
            FileType::Renaissance669 => write!(f, "669"),
            FileType::Stm => write!(f, "STM"),
            FileType::S3m => write!(f, "S3M"),
            FileType::Mtm => write!(f, "MTM"),
            FileType::Far => write!(f, "FAR"),
            FileType::Ult => write!(f, "ULT"),
            FileType::Amf => write!(f, "AMF"),
            FileType::Dmf => write!(f, "DMF"),
            FileType::Okt => write!(f, "OKT"),
            FileType::Rol => write!(f, "ROL"),
            FileType::Cmf => write!(f, "CMF"),
            FileType::Mid => write!(f, "MID"),
            FileType::Sadt => write!(f, "SADT"),
            FileType::Voc => write!(f, "VOC"),
            FileType::Wav => write!(f, "WAV"),
            FileType::Smp8 => write!(f, "SMP8"),
            FileType::Smp8s => write!(f, "SMP8S"),
            FileType::Smp16 => write!(f, "SMP16"),
            FileType::Smp16s => write!(f, "SMP16S"),
            FileType::Patch8 => write!(f, "PATCH8"),
            FileType::Patch16 => write!(f, "PATCH16"),
            FileType::Xm => write!(f, "XM"),
            FileType::Hsc => write!(f, "HSC"),
            FileType::It => write!(f, "IT"),
            FileType::Variable(value) => write!(f, "Variable({value})"),
            FileType::Xbin => write!(f, "XBin"),
            FileType::Zip => write!(f, "ZIP"),
            FileType::Arj => write!(f, "ARJ"),
            FileType::Lzh => write!(f, "LZH"),
            FileType::Arc => write!(f, "ARC"),
            FileType::Tar => write!(f, "TAR"),
            FileType::Zoo => write!(f, "ZOO"),
            FileType::Rar => write!(f, "RAR"),
            FileType::Uc2 => write!(f, "UC2"),
            FileType::Pak => write!(f, "PAK"),
            FileType::Sqz => write!(f, "SQZ"),
        }
    }
}

impl From<&FileType> for u8 {
    fn from(file_type: &FileType) -> Self {
        match file_type {
            FileType::None => 0,
            FileType::Ascii => 0,
            FileType::Gif => 0,
            FileType::Dxf => 0,
            FileType::Mod => 0,
            FileType::Zip => 0,
            FileType::Ansi => 1,
            FileType::Pcx => 1,
            FileType::Dwg => 1,
            FileType::Renaissance669 => 1,
            FileType::Arj => 1,
            FileType::Ansimation => 2,
            FileType::LbmOrIff => 2,
            FileType::WpgVector => 2,
            FileType::Stm => 2,
            FileType::Lzh => 2,
            FileType::RipScript => 3,
            FileType::Tga => 3,
            FileType::Studio3ds => 3,
            FileType::S3m => 3,
            FileType::Arc => 3,
            FileType::PcBoard => 4,
            FileType::Fli => 4,
            FileType::Mtm => 4,
            FileType::Tar => 4,
            FileType::Avatar => 5,
            FileType::Flc => 5,
            FileType::Far => 5,
            FileType::Zoo => 5,
            FileType::Html => 6,
            FileType::Bmp => 6,
            FileType::Ult => 6,
            FileType::Rar => 6,
            FileType::Source => 7,
            FileType::Gl => 7,
            FileType::Amf => 7,
            FileType::Uc2 => 7,
            FileType::TundraDraw => 8,
            FileType::Dl => 8,
            FileType::Dmf => 8,
            FileType::Pak => 8,
            FileType::WpgBitmap => 9,
            FileType::Okt => 9,
            FileType::Sqz => 9,
            FileType::Png => 10,
            FileType::Rol => 10,
            FileType::Jpg => 11,
            FileType::Cmf => 11,
            FileType::Mpg => 12,
            FileType::Mid => 12,
            FileType::Avi => 13,
            FileType::Sadt => 13,
            FileType::Voc => 14,
            FileType::Wav => 15,
            FileType::Smp8 => 16,
            FileType::Smp8s => 17,
            FileType::Smp16 => 18,
            FileType::Smp16s => 19,
            FileType::Patch8 => 20,
            FileType::Patch16 => 21,
            FileType::Xm => 22,
            FileType::Hsc => 23,
            FileType::It => 24,
            FileType::Variable(value) => *value,
            FileType::Xbin => 0,
        }
    }
}

impl FromStr for FileType {
    type Err = SauceError;
    fn from_str(string: &str) -> Result<FileType, Self::Err> {
        match string {
            "None" => Ok(FileType::None),
            "ASCII" => Ok(FileType::Ascii),
            "ANSI" => Ok(FileType::Ansi),
            "ANSImation" => Ok(FileType::Ansimation),
            "RIPScript" => Ok(FileType::RipScript),
            "PCBoard" => Ok(FileType::PcBoard),
            "Avatar" => Ok(FileType::Avatar),
            "HTML" => Ok(FileType::Html),
            "Source" => Ok(FileType::Source),
            "TundraDraw" => Ok(FileType::TundraDraw),
            "GIF" => Ok(FileType::Gif),
            "PCX" => Ok(FileType::Pcx),
            "LBM/IFF" => Ok(FileType::LbmOrIff),
            "TGA" => Ok(FileType::Tga),
            "FLI" => Ok(FileType::Fli),
            "FLC" => Ok(FileType::Flc),
            "BMP" => Ok(FileType::Bmp),
            "GL" => Ok(FileType::Gl),
            "DL" => Ok(FileType::Dl),
            "WPG Bitmap" => Ok(FileType::WpgBitmap),
            "PNG" => Ok(FileType::Png),
            "JPG" => Ok(FileType::Jpg),
            "MPG" => Ok(FileType::Mpg),
            "AVI" => Ok(FileType::Avi),
            "DXF" => Ok(FileType::Dxf),
            "DWG" => Ok(FileType::Dwg),
            "WPG Vector" => Ok(FileType::WpgVector),
            "3DS" => Ok(FileType::Studio3ds),
            "MOD" => Ok(FileType::Mod),
            "669" => Ok(FileType::Renaissance669),
            "STM" => Ok(FileType::Stm),
            "S3M" => Ok(FileType::S3m),
            "MTM" => Ok(FileType::Mtm),
            "FAR" => Ok(FileType::Far),
            "ULT" => Ok(FileType::Ult),
            "AMF" => Ok(FileType::Amf),
            "DMF" => Ok(FileType::Dmf),
            "OKT" => Ok(FileType::Okt),
            "ROL" => Ok(FileType::Rol),
            "CMF" => Ok(FileType::Cmf),
            "MID" => Ok(FileType::Mid),
            "SADT" => Ok(FileType::Sadt),
            "VOC" => Ok(FileType::Voc),
            "WAV" => Ok(FileType::Wav),
            "SMP8" => Ok(FileType::Smp8),
            "SMP8S" => Ok(FileType::Smp8s),
            "SMP16" => Ok(FileType::Smp16),
            "SMP16S" => Ok(FileType::Smp16s),
            "PATCH8" => Ok(FileType::Patch8),
            "PATCH16" => Ok(FileType::Patch16),
            "XM" => Ok(FileType::Xm),
            "HSC" => Ok(FileType::Hsc),
            "IT" => Ok(FileType::It),
            "XBin" => Ok(FileType::Xbin),
            "ZIP" => Ok(FileType::Zip),
            "ARJ" => Ok(FileType::Arj),
            "LZH" => Ok(FileType::Lzh),
            "ARC" => Ok(FileType::Arc),
            "TAR" => Ok(FileType::Tar),
            "ZOO" => Ok(FileType::Zoo),
            "RAR" => Ok(FileType::Rar),
            "UC2" => Ok(FileType::Uc2),
            "PAK" => Ok(FileType::Pak),
            "SQZ" => Ok(FileType::Sqz),
            _ => {
                let regex = Regex::new(r"^Variable\((\d+)\)$").expect("illegal regex");
                if let Some(captures) = regex.captures(string) {
                    if let Some(capture) = captures.get(1) {
                        let value_string = capture.as_str();
                        if let Ok(value) = value_string.parse::<u8>() {
                            return Ok(FileType::Variable(value));
                        }
                    }
                }
                Err(SauceError::InvalidFileType)
            }
        }
    }
}

impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

struct StringVisitor;

impl<'de> de::Visitor<'de> for StringVisitor {
    type Value = FileType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match FileType::from_str(value) {
            Ok(filetype) => Ok(filetype),
            Err(err) => Err(E::custom(err)),
        }
    }
}

impl<'de> Deserialize<'de> for FileType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::sauce::FileType;
    use serde_json::{from_str, to_string};
    #[test]
    fn de() {
        let filetype = FileType::Variable(160);
        let json = to_string(&filetype).unwrap();
        assert_eq!(json, "\"Variable(160)\"");
        let de_filetype = from_str::<FileType>(&json).unwrap();
        assert_eq!(filetype, de_filetype);
    }
}
