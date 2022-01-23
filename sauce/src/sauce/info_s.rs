use codepage437::{raw, CP437Error, CP437String, Font, FontError};
use serde::{de, Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum InfoS {
    IbmVga,
    IbmVga50,
    IbmVga25g,
    IbmEga,
    IbmEga43,
    IbmVga437,
    IbmVga50437,
    IbmVga25g437,
    IbmEga437,
    IbmEga43437,
    IbmVga720,
    IbmVga50720,
    IbmVga25g720,
    IbmEga720,
    IbmEga43720,
    IbmVga737,
    IbmVga50737,
    IbmVga25g737,
    IbmEga737,
    IbmEga43737,
    IbmVga775,
    IbmVga50775,
    IbmVga25g775,
    IbmEga775,
    IbmEga43775,
    IbmVga819,
    IbmVga50819,
    IbmVga25g819,
    IbmEga819,
    IbmEga43819,
    IbmVga850,
    IbmVga50850,
    IbmVga25g850,
    IbmEga850,
    IbmEga43850,
    IbmVga852,
    IbmVga50852,
    IbmVga25g852,
    IbmEga852,
    IbmEga43852,
    IbmVga855,
    IbmVga50855,
    IbmVga25g855,
    IbmEga855,
    IbmEga43855,
    IbmVga857,
    IbmVga50857,
    IbmVga25g857,
    IbmEga857,
    IbmEga43857,
    IbmVga858,
    IbmVga50858,
    IbmVga25g858,
    IbmEga858,
    IbmEga43858,
    IbmVga860,
    IbmVga50860,
    IbmVga25g860,
    IbmEga860,
    IbmEga43860,
    IbmVga861,
    IbmVga50861,
    IbmVga25g861,
    IbmEga861,
    IbmEga43861,
    IbmVga862,
    IbmVga50862,
    IbmVga25g862,
    IbmEga862,
    IbmEga43862,
    IbmVga863,
    IbmVga50863,
    IbmVga25g863,
    IbmEga863,
    IbmEga43863,
    IbmVga864,
    IbmVga50864,
    IbmVga25g864,
    IbmEga864,
    IbmEga43864,
    IbmVga865,
    IbmVga50865,
    IbmVga25g865,
    IbmEga865,
    IbmEga43865,
    IbmVga866,
    IbmVga50866,
    IbmVga25g866,
    IbmEga866,
    IbmEga43866,
    IbmVga869,
    IbmVga50869,
    IbmVga25g869,
    IbmEga869,
    IbmEga43869,
    IbmVga872,
    IbmVga50872,
    IbmVga25g872,
    IbmEga872,
    IbmEga43872,
    IbmVgaKam,
    IbmVga50Kam,
    IbmVga25gKam,
    IbmEgaKam,
    IbmEga43Kam,
    IbmVgaMaz,
    IbmVga50Maz,
    IbmVga25gMaz,
    IbmEgaMaz,
    IbmEga43Maz,
    IbmVgaMik,
    IbmVga50Mik,
    IbmVga25gMik,
    IbmEgaMik,
    IbmEga43Mik,
    AmigaTopaz1,
    AmigaTopaz1Plus,
    AmigaTopaz2,
    AmigaTopaz2Plus,
    AmigaP0tNoodle,
    AmigaMicroKnight,
    AmigaMicroKnightPlus,
    AmigaMosoul,
    C64PetsciiUnshifted,
    C64PetsciiShifted,
    AtariAtascii,
    Custom(CP437String),
}

impl TryFrom<&str> for InfoS {
    type Error = CP437Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "IBM VGA" => Ok(InfoS::IbmVga),
            "IBM VGA50" => Ok(InfoS::IbmVga50),
            "IBM VGA25G" => Ok(InfoS::IbmVga25g),
            "IBM EGA" => Ok(InfoS::IbmEga),
            "IBM EGA43" => Ok(InfoS::IbmEga43),
            "IBM VGA 437" => Ok(InfoS::IbmVga437),
            "IBM VGA50 437" => Ok(InfoS::IbmVga50437),
            "IBM VGA25G 437" => Ok(InfoS::IbmVga25g437),
            "IBM EGA 437" => Ok(InfoS::IbmEga437),
            "IBM EGA43 437" => Ok(InfoS::IbmEga43437),
            "IBM VGA 720" => Ok(InfoS::IbmVga720),
            "IBM VGA50 720" => Ok(InfoS::IbmVga50720),
            "IBM VGA25G 720" => Ok(InfoS::IbmVga25g720),
            "IBM EGA 720" => Ok(InfoS::IbmEga720),
            "IBM EGA43 720" => Ok(InfoS::IbmEga43720),
            "IBM VGA 737" => Ok(InfoS::IbmVga737),
            "IBM VGA50 737" => Ok(InfoS::IbmVga50737),
            "IBM VGA25G 737" => Ok(InfoS::IbmVga25g737),
            "IBM EGA 737" => Ok(InfoS::IbmEga737),
            "IBM EGA43 737" => Ok(InfoS::IbmEga43737),
            "IBM VGA 775" => Ok(InfoS::IbmVga775),
            "IBM VGA50 775" => Ok(InfoS::IbmVga50775),
            "IBM VGA25G 775" => Ok(InfoS::IbmVga25g775),
            "IBM EGA 775" => Ok(InfoS::IbmEga775),
            "IBM EGA43 775" => Ok(InfoS::IbmEga43775),
            "IBM VGA 819" => Ok(InfoS::IbmVga819),
            "IBM VGA50 819" => Ok(InfoS::IbmVga50819),
            "IBM VGA25G 819" => Ok(InfoS::IbmVga25g819),
            "IBM EGA 819" => Ok(InfoS::IbmEga819),
            "IBM EGA43 819" => Ok(InfoS::IbmEga43819),
            "IBM VGA 850" => Ok(InfoS::IbmVga850),
            "IBM VGA50 850" => Ok(InfoS::IbmVga50850),
            "IBM VGA25G 850" => Ok(InfoS::IbmVga25g850),
            "IBM EGA 850" => Ok(InfoS::IbmEga850),
            "IBM EGA43 850" => Ok(InfoS::IbmEga43850),
            "IBM VGA 852" => Ok(InfoS::IbmVga852),
            "IBM VGA50 852" => Ok(InfoS::IbmVga50852),
            "IBM VGA25G 852" => Ok(InfoS::IbmVga25g852),
            "IBM EGA 852" => Ok(InfoS::IbmEga852),
            "IBM EGA43 852" => Ok(InfoS::IbmEga43852),
            "IBM VGA 855" => Ok(InfoS::IbmVga855),
            "IBM VGA50 855" => Ok(InfoS::IbmVga50855),
            "IBM VGA25G 855" => Ok(InfoS::IbmVga25g855),
            "IBM EGA 855" => Ok(InfoS::IbmEga855),
            "IBM EGA43 855" => Ok(InfoS::IbmEga43855),
            "IBM VGA 857" => Ok(InfoS::IbmVga857),
            "IBM VGA50 857" => Ok(InfoS::IbmVga50857),
            "IBM VGA25G 857" => Ok(InfoS::IbmVga25g857),
            "IBM EGA 857" => Ok(InfoS::IbmEga857),
            "IBM EGA43 857" => Ok(InfoS::IbmEga43857),
            "IBM VGA 858" => Ok(InfoS::IbmVga858),
            "IBM VGA50 858" => Ok(InfoS::IbmVga50858),
            "IBM VGA25G 858" => Ok(InfoS::IbmVga25g858),
            "IBM EGA 858" => Ok(InfoS::IbmEga858),
            "IBM EGA43 858" => Ok(InfoS::IbmEga43858),
            "IBM VGA 860" => Ok(InfoS::IbmVga860),
            "IBM VGA50 860" => Ok(InfoS::IbmVga50860),
            "IBM VGA25G 860" => Ok(InfoS::IbmVga25g860),
            "IBM EGA 860" => Ok(InfoS::IbmEga860),
            "IBM EGA43 860" => Ok(InfoS::IbmEga43860),
            "IBM VGA 861" => Ok(InfoS::IbmVga861),
            "IBM VGA50 861" => Ok(InfoS::IbmVga50861),
            "IBM VGA25G 861" => Ok(InfoS::IbmVga25g861),
            "IBM EGA 861" => Ok(InfoS::IbmEga861),
            "IBM EGA43 861" => Ok(InfoS::IbmEga43861),
            "IBM VGA 862" => Ok(InfoS::IbmVga862),
            "IBM VGA50 862" => Ok(InfoS::IbmVga50862),
            "IBM VGA25G 862" => Ok(InfoS::IbmVga25g862),
            "IBM EGA 862" => Ok(InfoS::IbmEga862),
            "IBM EGA43 862" => Ok(InfoS::IbmEga43862),
            "IBM VGA 863" => Ok(InfoS::IbmVga863),
            "IBM VGA50 863" => Ok(InfoS::IbmVga50863),
            "IBM VGA25G 863" => Ok(InfoS::IbmVga25g863),
            "IBM EGA 863" => Ok(InfoS::IbmEga863),
            "IBM EGA43 863" => Ok(InfoS::IbmEga43863),
            "IBM VGA 864" => Ok(InfoS::IbmVga864),
            "IBM VGA50 864" => Ok(InfoS::IbmVga50864),
            "IBM VGA25G 864" => Ok(InfoS::IbmVga25g864),
            "IBM EGA 864" => Ok(InfoS::IbmEga864),
            "IBM EGA43 864" => Ok(InfoS::IbmEga43864),
            "IBM VGA 865" => Ok(InfoS::IbmVga865),
            "IBM VGA50 865" => Ok(InfoS::IbmVga50865),
            "IBM VGA25G 865" => Ok(InfoS::IbmVga25g865),
            "IBM EGA 865" => Ok(InfoS::IbmEga865),
            "IBM EGA43 865" => Ok(InfoS::IbmEga43865),
            "IBM VGA 866" => Ok(InfoS::IbmVga866),
            "IBM VGA50 866" => Ok(InfoS::IbmVga50866),
            "IBM VGA25G 866" => Ok(InfoS::IbmVga25g866),
            "IBM EGA 866" => Ok(InfoS::IbmEga866),
            "IBM EGA43 866" => Ok(InfoS::IbmEga43866),
            "IBM VGA 869" => Ok(InfoS::IbmVga869),
            "IBM VGA50 869" => Ok(InfoS::IbmVga50869),
            "IBM VGA25G 869" => Ok(InfoS::IbmVga25g869),
            "IBM EGA 869" => Ok(InfoS::IbmEga869),
            "IBM EGA43 869" => Ok(InfoS::IbmEga43869),
            "IBM VGA 872" => Ok(InfoS::IbmVga872),
            "IBM VGA50 872" => Ok(InfoS::IbmVga50872),
            "IBM VGA25G 872" => Ok(InfoS::IbmVga25g872),
            "IBM EGA 872" => Ok(InfoS::IbmEga872),
            "IBM EGA43 872" => Ok(InfoS::IbmEga43872),
            "IBM VGA KAM" => Ok(InfoS::IbmVgaKam),
            "IBM VGA50 KAM" => Ok(InfoS::IbmVga50Kam),
            "IBM VGA25G KAM" => Ok(InfoS::IbmVga25gKam),
            "IBM EGA KAM" => Ok(InfoS::IbmEgaKam),
            "IBM EGA43 KAM" => Ok(InfoS::IbmEga43Kam),
            "IBM VGA MAZ" => Ok(InfoS::IbmVgaMaz),
            "IBM VGA50 MAZ" => Ok(InfoS::IbmVga50Maz),
            "IBM VGA25G MAZ" => Ok(InfoS::IbmVga25gMaz),
            "IBM EGA MAZ" => Ok(InfoS::IbmEgaMaz),
            "IBM EGA43 MAZ" => Ok(InfoS::IbmEga43Maz),
            "IBM VGA MIK" => Ok(InfoS::IbmVgaMik),
            "IBM VGA50 MIK" => Ok(InfoS::IbmVga50Mik),
            "IBM VGA25G MIK" => Ok(InfoS::IbmVga25gMik),
            "IBM EGA MIK" => Ok(InfoS::IbmEgaMik),
            "IBM EGA43 MIK" => Ok(InfoS::IbmEga43Mik),
            "Amiga Topaz 1" => Ok(InfoS::AmigaTopaz1),
            "Amiga Topaz 1+" => Ok(InfoS::AmigaTopaz1Plus),
            "Amiga Topaz 2" => Ok(InfoS::AmigaTopaz2),
            "Amiga Topaz 2+" => Ok(InfoS::AmigaTopaz2Plus),
            "Amiga P0T-NOoDLE" => Ok(InfoS::AmigaP0tNoodle),
            "Amiga MicroKnight" => Ok(InfoS::AmigaMicroKnight),
            "Amiga MicroKnight+" => Ok(InfoS::AmigaMicroKnightPlus),
            "Amiga mOsOul" => Ok(InfoS::AmigaMosoul),
            "C64 PETSCII unshifted" => Ok(InfoS::C64PetsciiUnshifted),
            "C64 PETSCII shifted" => Ok(InfoS::C64PetsciiShifted),
            "Atari ATASCII" => Ok(InfoS::AtariAtascii),
            _ => {
                let cp437 = CP437String::try_from(string)?;
                Ok(InfoS::Custom(cp437))
            }
        }
    }
}

impl From<&CP437String> for InfoS {
    fn from(cp437_string: &CP437String) -> Self {
        InfoS::try_from(cp437_string.strip_trailing_nulls().to_string().as_str())
            .expect("legal cp437")
    }
}

impl From<&InfoS> for CP437String {
    fn from(info_s: &InfoS) -> Self {
        CP437String::try_from(info_s.to_string().as_str())
            .expect("legal cp437")
            .pad_with_nulls(22)
    }
}

impl fmt::Display for InfoS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfoS::IbmVga => write!(f, "IBM VGA"),
            InfoS::IbmVga50 => write!(f, "IBM VGA50"),
            InfoS::IbmVga25g => write!(f, "IBM VGA25G"),
            InfoS::IbmEga => write!(f, "IBM EGA"),
            InfoS::IbmEga43 => write!(f, "IBM EGA43"),
            InfoS::IbmVga437 => write!(f, "IBM VGA 437"),
            InfoS::IbmVga50437 => write!(f, "IBM VGA50 437"),
            InfoS::IbmVga25g437 => write!(f, "IBM VGA25G 437"),
            InfoS::IbmEga437 => write!(f, "IBM EGA 437"),
            InfoS::IbmEga43437 => write!(f, "IBM EGA43 437"),
            InfoS::IbmVga720 => write!(f, "IBM VGA 720"),
            InfoS::IbmVga50720 => write!(f, "IBM VGA50 720"),
            InfoS::IbmVga25g720 => write!(f, "IBM VGA25G 720"),
            InfoS::IbmEga720 => write!(f, "IBM EGA 720"),
            InfoS::IbmEga43720 => write!(f, "IBM EGA43 720"),
            InfoS::IbmVga737 => write!(f, "IBM VGA 737"),
            InfoS::IbmVga50737 => write!(f, "IBM VGA50 737"),
            InfoS::IbmVga25g737 => write!(f, "IBM VGA25G 737"),
            InfoS::IbmEga737 => write!(f, "IBM EGA 737"),
            InfoS::IbmEga43737 => write!(f, "IBM EGA43 737"),
            InfoS::IbmVga775 => write!(f, "IBM VGA 775"),
            InfoS::IbmVga50775 => write!(f, "IBM VGA50 775"),
            InfoS::IbmVga25g775 => write!(f, "IBM VGA25G 775"),
            InfoS::IbmEga775 => write!(f, "IBM EGA 775"),
            InfoS::IbmEga43775 => write!(f, "IBM EGA43 775"),
            InfoS::IbmVga819 => write!(f, "IBM VGA 819"),
            InfoS::IbmVga50819 => write!(f, "IBM VGA50 819"),
            InfoS::IbmVga25g819 => write!(f, "IBM VGA25G 819"),
            InfoS::IbmEga819 => write!(f, "IBM EGA 819"),
            InfoS::IbmEga43819 => write!(f, "IBM EGA43 819"),
            InfoS::IbmVga850 => write!(f, "IBM VGA 850"),
            InfoS::IbmVga50850 => write!(f, "IBM VGA50 850"),
            InfoS::IbmVga25g850 => write!(f, "IBM VGA25G 850"),
            InfoS::IbmEga850 => write!(f, "IBM EGA 850"),
            InfoS::IbmEga43850 => write!(f, "IBM EGA43 850"),
            InfoS::IbmVga852 => write!(f, "IBM VGA 852"),
            InfoS::IbmVga50852 => write!(f, "IBM VGA50 852"),
            InfoS::IbmVga25g852 => write!(f, "IBM VGA25G 852"),
            InfoS::IbmEga852 => write!(f, "IBM EGA 852"),
            InfoS::IbmEga43852 => write!(f, "IBM EGA43 852"),
            InfoS::IbmVga855 => write!(f, "IBM VGA 855"),
            InfoS::IbmVga50855 => write!(f, "IBM VGA50 855"),
            InfoS::IbmVga25g855 => write!(f, "IBM VGA25G 855"),
            InfoS::IbmEga855 => write!(f, "IBM EGA 855"),
            InfoS::IbmEga43855 => write!(f, "IBM EGA43 855"),
            InfoS::IbmVga857 => write!(f, "IBM VGA 857"),
            InfoS::IbmVga50857 => write!(f, "IBM VGA50 857"),
            InfoS::IbmVga25g857 => write!(f, "IBM VGA25G 857"),
            InfoS::IbmEga857 => write!(f, "IBM EGA 857"),
            InfoS::IbmEga43857 => write!(f, "IBM EGA43 857"),
            InfoS::IbmVga858 => write!(f, "IBM VGA 858"),
            InfoS::IbmVga50858 => write!(f, "IBM VGA50 858"),
            InfoS::IbmVga25g858 => write!(f, "IBM VGA25G 858"),
            InfoS::IbmEga858 => write!(f, "IBM EGA 858"),
            InfoS::IbmEga43858 => write!(f, "IBM EGA43 858"),
            InfoS::IbmVga860 => write!(f, "IBM VGA 860"),
            InfoS::IbmVga50860 => write!(f, "IBM VGA50 860"),
            InfoS::IbmVga25g860 => write!(f, "IBM VGA25G 860"),
            InfoS::IbmEga860 => write!(f, "IBM EGA 860"),
            InfoS::IbmEga43860 => write!(f, "IBM EGA43 860"),
            InfoS::IbmVga861 => write!(f, "IBM VGA 861"),
            InfoS::IbmVga50861 => write!(f, "IBM VGA50 861"),
            InfoS::IbmVga25g861 => write!(f, "IBM VGA25G 861"),
            InfoS::IbmEga861 => write!(f, "IBM EGA 861"),
            InfoS::IbmEga43861 => write!(f, "IBM EGA43 861"),
            InfoS::IbmVga862 => write!(f, "IBM VGA 862"),
            InfoS::IbmVga50862 => write!(f, "IBM VGA50 862"),
            InfoS::IbmVga25g862 => write!(f, "IBM VGA25G 862"),
            InfoS::IbmEga862 => write!(f, "IBM EGA 862"),
            InfoS::IbmEga43862 => write!(f, "IBM EGA43 862"),
            InfoS::IbmVga863 => write!(f, "IBM VGA 863"),
            InfoS::IbmVga50863 => write!(f, "IBM VGA50 863"),
            InfoS::IbmVga25g863 => write!(f, "IBM VGA25G 863"),
            InfoS::IbmEga863 => write!(f, "IBM EGA 863"),
            InfoS::IbmEga43863 => write!(f, "IBM EGA43 863"),
            InfoS::IbmVga864 => write!(f, "IBM VGA 864"),
            InfoS::IbmVga50864 => write!(f, "IBM VGA50 864"),
            InfoS::IbmVga25g864 => write!(f, "IBM VGA25G 864"),
            InfoS::IbmEga864 => write!(f, "IBM EGA 864"),
            InfoS::IbmEga43864 => write!(f, "IBM EGA43 864"),
            InfoS::IbmVga865 => write!(f, "IBM VGA 865"),
            InfoS::IbmVga50865 => write!(f, "IBM VGA50 865"),
            InfoS::IbmVga25g865 => write!(f, "IBM VGA25G 865"),
            InfoS::IbmEga865 => write!(f, "IBM EGA 865"),
            InfoS::IbmEga43865 => write!(f, "IBM EGA43 865"),
            InfoS::IbmVga866 => write!(f, "IBM VGA 866"),
            InfoS::IbmVga50866 => write!(f, "IBM VGA50 866"),
            InfoS::IbmVga25g866 => write!(f, "IBM VGA25G 866"),
            InfoS::IbmEga866 => write!(f, "IBM EGA 866"),
            InfoS::IbmEga43866 => write!(f, "IBM EGA43 866"),
            InfoS::IbmVga869 => write!(f, "IBM VGA 869"),
            InfoS::IbmVga50869 => write!(f, "IBM VGA50 869"),
            InfoS::IbmVga25g869 => write!(f, "IBM VGA25G 869"),
            InfoS::IbmEga869 => write!(f, "IBM EGA 869"),
            InfoS::IbmEga43869 => write!(f, "IBM EGA43 869"),
            InfoS::IbmVga872 => write!(f, "IBM VGA 872"),
            InfoS::IbmVga50872 => write!(f, "IBM VGA50 872"),
            InfoS::IbmVga25g872 => write!(f, "IBM VGA25G 872"),
            InfoS::IbmEga872 => write!(f, "IBM EGA 872"),
            InfoS::IbmEga43872 => write!(f, "IBM EGA43 872"),
            InfoS::IbmVgaKam => write!(f, "IBM VGA KAM"),
            InfoS::IbmVga50Kam => write!(f, "IBM VGA50 KAM"),
            InfoS::IbmVga25gKam => write!(f, "IBM VGA25G KAM"),
            InfoS::IbmEgaKam => write!(f, "IBM EGA KAM"),
            InfoS::IbmEga43Kam => write!(f, "IBM EGA43 KAM"),
            InfoS::IbmVgaMaz => write!(f, "IBM VGA MAZ"),
            InfoS::IbmVga50Maz => write!(f, "IBM VGA50 MAZ"),
            InfoS::IbmVga25gMaz => write!(f, "IBM VGA25G MAZ"),
            InfoS::IbmEgaMaz => write!(f, "IBM EGA MAZ"),
            InfoS::IbmEga43Maz => write!(f, "IBM EGA43 MAZ"),
            InfoS::IbmVgaMik => write!(f, "IBM VGA MIK"),
            InfoS::IbmVga50Mik => write!(f, "IBM VGA50 MIK"),
            InfoS::IbmVga25gMik => write!(f, "IBM VGA25G MIK"),
            InfoS::IbmEgaMik => write!(f, "IBM EGA MIK"),
            InfoS::IbmEga43Mik => write!(f, "IBM EGA43 MIK"),
            InfoS::AmigaTopaz1 => write!(f, "Amiga Topaz 1"),
            InfoS::AmigaTopaz1Plus => write!(f, "Amiga Topaz 1+"),
            InfoS::AmigaTopaz2 => write!(f, "Amiga Topaz 2"),
            InfoS::AmigaTopaz2Plus => write!(f, "Amiga Topaz 2+"),
            InfoS::AmigaP0tNoodle => write!(f, "Amiga P0T-NOoDLE"),
            InfoS::AmigaMicroKnight => write!(f, "Amiga MicroKnight"),
            InfoS::AmigaMicroKnightPlus => write!(f, "Amiga MicroKnight+"),
            InfoS::AmigaMosoul => write!(f, "Amiga mOsOul"),
            InfoS::C64PetsciiUnshifted => write!(f, "C64 PETSCII unshifted"),
            InfoS::C64PetsciiShifted => write!(f, "C64 PETSCII shifted"),
            InfoS::AtariAtascii => write!(f, "Atari ATASCII"),
            InfoS::Custom(s) => write!(f, "{s}"),
        }
    }
}

impl TryFrom<InfoS> for Font {
    type Error = FontError;

    fn try_from(info_s: InfoS) -> Result<Self, Self::Error> {
        match info_s {
            InfoS::IbmVga => Font::try_from(raw::CP437_F16.as_ref()),
            InfoS::IbmVga50 => Font::try_from(raw::CP437_F08.as_ref()),
            InfoS::IbmVga25g => Font::try_from(raw::CP437_F19.as_ref()),
            InfoS::IbmEga => Font::try_from(raw::CP437_F14.as_ref()),
            InfoS::IbmEga43 => Font::try_from(raw::CP437_F08.as_ref()),
            InfoS::IbmVga437 => Font::try_from(raw::CP437_F16.as_ref()),
            InfoS::IbmVga50437 => Font::try_from(raw::CP437_F08.as_ref()),
            InfoS::IbmVga25g437 => Font::try_from(raw::CP437_F19.as_ref()),
            InfoS::IbmEga437 => Font::try_from(raw::CP437_F14.as_ref()),
            InfoS::IbmEga43437 => Font::try_from(raw::CP437_F08.as_ref()),
            // InfoS::IbmVga720 => Font::try_from(raw::CP720_F16.as_ref()),
            // InfoS::IbmVga50720 => Font::try_from(raw::CP720_F08.as_ref()),
            // InfoS::IbmVga25g720 => Font::try_from(raw::CP720_F19.as_ref()),
            // InfoS::IbmEga720 => Font::try_from(raw::CP720_F14.as_ref()),
            // InfoS::IbmEga43720 => Font::try_from(raw::CP720_F08.as_ref()),
            InfoS::IbmVga737 => Font::try_from(raw::CP737_F16.as_ref()),
            InfoS::IbmVga50737 => Font::try_from(raw::CP737_F08.as_ref()),
            // InfoS::IbmVga25g737 => Font::try_from(raw::CP737_F19.as_ref()),
            InfoS::IbmEga737 => Font::try_from(raw::CP737_F14.as_ref()),
            InfoS::IbmEga43737 => Font::try_from(raw::CP737_F08.as_ref()),
            InfoS::IbmVga775 => Font::try_from(raw::CP775_F16.as_ref()),
            InfoS::IbmVga50775 => Font::try_from(raw::CP775_F08.as_ref()),
            // InfoS::IbmVga25g775 => Font::try_from(raw::CP775_F19.as_ref()),
            InfoS::IbmEga775 => Font::try_from(raw::CP775_F14.as_ref()),
            InfoS::IbmEga43775 => Font::try_from(raw::CP775_F08.as_ref()),
            // InfoS::IbmVga819 => Font::try_from(raw::CP819_F16.as_ref()),
            // InfoS::IbmVga50819 => Font::try_from(raw::CP819_F08.as_ref()),
            // InfoS::IbmVga25g819 => Font::try_from(raw::CP819_F19.as_ref()),
            // InfoS::IbmEga819 => Font::try_from(raw::CP819_F14.as_ref()),
            // InfoS::IbmEga43819 => Font::try_from(raw::CP819_F08.as_ref()),
            InfoS::IbmVga850 => Font::try_from(raw::CP850_F16.as_ref()),
            InfoS::IbmVga50850 => Font::try_from(raw::CP850_F08.as_ref()),
            InfoS::IbmVga25g850 => Font::try_from(raw::CP850_F19.as_ref()),
            InfoS::IbmEga850 => Font::try_from(raw::CP850_F14.as_ref()),
            InfoS::IbmEga43850 => Font::try_from(raw::CP850_F08.as_ref()),
            InfoS::IbmVga852 => Font::try_from(raw::CP852_F16.as_ref()),
            InfoS::IbmVga50852 => Font::try_from(raw::CP852_F08.as_ref()),
            InfoS::IbmVga25g852 => Font::try_from(raw::CP852_F19.as_ref()),
            InfoS::IbmEga852 => Font::try_from(raw::CP852_F14.as_ref()),
            InfoS::IbmEga43852 => Font::try_from(raw::CP852_F08.as_ref()),
            InfoS::IbmVga855 => Font::try_from(raw::CP855_F16.as_ref()),
            InfoS::IbmVga50855 => Font::try_from(raw::CP855_F08.as_ref()),
            // InfoS::IbmVga25g855 => Font::try_from(raw::CP855_F19.as_ref()),
            InfoS::IbmEga855 => Font::try_from(raw::CP855_F14.as_ref()),
            InfoS::IbmEga43855 => Font::try_from(raw::CP855_F08.as_ref()),
            InfoS::IbmVga857 => Font::try_from(raw::CP857_F16.as_ref()),
            InfoS::IbmVga50857 => Font::try_from(raw::CP857_F08.as_ref()),
            // InfoS::IbmVga25g857 => Font::try_from(raw::CP857_F19.as_ref()),
            InfoS::IbmEga857 => Font::try_from(raw::CP857_F14.as_ref()),
            InfoS::IbmEga43857 => Font::try_from(raw::CP857_F08.as_ref()),
            // InfoS::IbmVga858 => Font::try_from(raw::CP858_F16.as_ref()),
            // InfoS::IbmVga50858 => Font::try_from(raw::CP858_F08.as_ref()),
            // InfoS::IbmVga25g858 => Font::try_from(raw::CP858_F19.as_ref()),
            // InfoS::IbmEga858 => Font::try_from(raw::CP858_F14.as_ref()),
            // InfoS::IbmEga43858 => Font::try_from(raw::CP858_F08.as_ref()),
            InfoS::IbmVga860 => Font::try_from(raw::CP860_F16.as_ref()),
            InfoS::IbmVga50860 => Font::try_from(raw::CP860_F08.as_ref()),
            InfoS::IbmVga25g860 => Font::try_from(raw::CP860_F19.as_ref()),
            InfoS::IbmEga860 => Font::try_from(raw::CP860_F14.as_ref()),
            InfoS::IbmEga43860 => Font::try_from(raw::CP860_F08.as_ref()),
            InfoS::IbmVga861 => Font::try_from(raw::CP861_F16.as_ref()),
            InfoS::IbmVga50861 => Font::try_from(raw::CP861_F08.as_ref()),
            InfoS::IbmVga25g861 => Font::try_from(raw::CP861_F19.as_ref()),
            InfoS::IbmEga861 => Font::try_from(raw::CP861_F14.as_ref()),
            InfoS::IbmEga43861 => Font::try_from(raw::CP861_F08.as_ref()),
            InfoS::IbmVga862 => Font::try_from(raw::CP862_F16.as_ref()),
            InfoS::IbmVga50862 => Font::try_from(raw::CP862_F08.as_ref()),
            // InfoS::IbmVga25g862 => Font::try_from(raw::CP862_F19.as_ref()),
            InfoS::IbmEga862 => Font::try_from(raw::CP862_F14.as_ref()),
            InfoS::IbmEga43862 => Font::try_from(raw::CP862_F08.as_ref()),
            InfoS::IbmVga863 => Font::try_from(raw::CP863_F16.as_ref()),
            InfoS::IbmVga50863 => Font::try_from(raw::CP863_F08.as_ref()),
            InfoS::IbmVga25g863 => Font::try_from(raw::CP863_F19.as_ref()),
            InfoS::IbmEga863 => Font::try_from(raw::CP863_F14.as_ref()),
            InfoS::IbmEga43863 => Font::try_from(raw::CP863_F08.as_ref()),
            InfoS::IbmVga864 => Font::try_from(raw::CP864_F16.as_ref()),
            InfoS::IbmVga50864 => Font::try_from(raw::CP864_F08.as_ref()),
            // InfoS::IbmVga25g864 => Font::try_from(raw::CP864_F19.as_ref()),
            InfoS::IbmEga864 => Font::try_from(raw::CP864_F14.as_ref()),
            InfoS::IbmEga43864 => Font::try_from(raw::CP864_F08.as_ref()),
            InfoS::IbmVga865 => Font::try_from(raw::CP865_F16.as_ref()),
            InfoS::IbmVga50865 => Font::try_from(raw::CP865_F08.as_ref()),
            InfoS::IbmVga25g865 => Font::try_from(raw::CP865_F19.as_ref()),
            InfoS::IbmEga865 => Font::try_from(raw::CP865_F14.as_ref()),
            InfoS::IbmEga43865 => Font::try_from(raw::CP865_F08.as_ref()),
            InfoS::IbmVga866 => Font::try_from(raw::CP866_F16.as_ref()),
            InfoS::IbmVga50866 => Font::try_from(raw::CP866_F08.as_ref()),
            // InfoS::IbmVga25g866 => Font::try_from(raw::CP866_F19.as_ref()),
            InfoS::IbmEga866 => Font::try_from(raw::CP866_F14.as_ref()),
            InfoS::IbmEga43866 => Font::try_from(raw::CP866_F08.as_ref()),
            InfoS::IbmVga869 => Font::try_from(raw::CP869_F16.as_ref()),
            InfoS::IbmVga50869 => Font::try_from(raw::CP869_F08.as_ref()),
            // InfoS::IbmVga25g869 => Font::try_from(raw::CP869_F19.as_ref()),
            InfoS::IbmEga869 => Font::try_from(raw::CP869_F14.as_ref()),
            InfoS::IbmEga43869 => Font::try_from(raw::CP869_F08.as_ref()),
            // InfoS::IbmVga872 => Font::try_from(raw::CP872_F16.as_ref()),
            // InfoS::IbmVga50872 => Font::try_from(raw::CP872_F08.as_ref()),
            // InfoS::IbmVga25g872 => Font::try_from(raw::CP872_F19.as_ref()),
            // InfoS::IbmEga872 => Font::try_from(raw::CP872_F14.as_ref()),
            // InfoS::IbmEga43872 => Font::try_from(raw::CP872_F08.as_ref()),
            // InfoS::IbmVgaKam => Font::try_from(raw::CPKAM_F16.as_ref()),
            // InfoS::IbmVga50Kam => Font::try_from(raw::CPKAM_F08.as_ref()),
            // InfoS::IbmVga25gKam => Font::try_from(raw::CPKAM_F19.as_ref()),
            // InfoS::IbmEgaKam => Font::try_from(raw::CPKAM_F14.as_ref()),
            // InfoS::IbmEga43Kam => Font::try_from(raw::CPKAM_F08.as_ref()),
            // InfoS::IbmVgaMaz => Font::try_from(raw::CPMAZ_F16.as_ref()),
            // InfoS::IbmVga50Maz => Font::try_from(raw::CPMAZ_F08.as_ref()),
            // InfoS::IbmVga25gMaz => Font::try_from(raw::CPMAZ_F19.as_ref()),
            // InfoS::IbmEgaMaz => Font::try_from(raw::CPMAZ_F14.as_ref()),
            // InfoS::IbmEga43Maz => Font::try_from(raw::CPMAZ_F08.as_ref()),
            // InfoS::IbmVgaMik => Font::try_from(raw::CPMIK_F16.as_ref()),
            // InfoS::IbmVga50Mik => Font::try_from(raw::CPMIK_F08.as_ref()),
            // InfoS::IbmVga25gMik => Font::try_from(raw::CPMIK_F19.as_ref()),
            // InfoS::IbmEgaMik => Font::try_from(raw::CPMIK_F14.as_ref()),
            // InfoS::IbmEga43Mik => Font::try_from(raw::CPMIK_F08.as_ref()),
            InfoS::AmigaTopaz1 => Font::try_from(raw::TOPAZ_A500_F16.as_ref()),
            InfoS::AmigaTopaz1Plus => Font::try_from(raw::TOPAZ_PLUS_A500_F16.as_ref()),
            InfoS::AmigaTopaz2 => Font::try_from(raw::TOPAZ_A1200_F16.as_ref()),
            InfoS::AmigaTopaz2Plus => Font::try_from(raw::TOPAZ_PLUS_A1200_F16.as_ref()),
            InfoS::AmigaP0tNoodle => Font::try_from(raw::P0T_NOODLE_F16.as_ref()),
            InfoS::AmigaMicroKnight => Font::try_from(raw::MICRO_KNIGHT_F16.as_ref()),
            InfoS::AmigaMicroKnightPlus => Font::try_from(raw::MICRO_KNIGHT_PLUS_F16.as_ref()),
            InfoS::AmigaMosoul => Font::try_from(raw::MO_SOUL_F16.as_ref()),
            InfoS::C64PetsciiUnshifted => Font::try_from(raw::PETSCII_UNSHIFTED_F08.as_ref()),
            InfoS::C64PetsciiShifted => Font::try_from(raw::PETSCII_UNSHIFTED_F08.as_ref()),
            InfoS::AtariAtascii => Font::try_from(raw::ATASCII_F08.as_ref()),
            _ => Err(FontError::CouldNotLoadFont),
        }
    }
}

impl Serialize for InfoS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

struct StringVisitor;

impl<'de> de::Visitor<'de> for StringVisitor {
    type Value = InfoS;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match InfoS::try_from(value) {
            Ok(info_s) => Ok(info_s),
            Err(err) => Err(E::custom(err)),
        }
    }
}

impl<'de> Deserialize<'de> for InfoS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StringVisitor)
    }
}