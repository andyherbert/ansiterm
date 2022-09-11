use crate::SauceError;
use serde::{de, Deserialize, Serialize};
pub use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LetterSpacing {
    None,
    EightPixels,
    NinePixels,
}

impl Default for LetterSpacing {
    fn default() -> LetterSpacing {
        LetterSpacing::EightPixels
    }
}

impl fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LetterSpacing::None => write!(f, "None"),
            LetterSpacing::EightPixels => write!(f, "8px"),
            LetterSpacing::NinePixels => write!(f, "9px"),
        }
    }
}

impl FromStr for LetterSpacing {
    type Err = SauceError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "None" => Ok(LetterSpacing::None),
            "8px" => Ok(LetterSpacing::EightPixels),
            "9px" => Ok(LetterSpacing::NinePixels),
            _ => Err(SauceError::InvalidLetterSpacingValue),
        }
    }
}

impl From<u8> for LetterSpacing {
    fn from(value: u8) -> Self {
        match (value >> 1) & 3 {
            0 => LetterSpacing::None,
            1 => LetterSpacing::EightPixels,
            2 => LetterSpacing::NinePixels,
            _ => unreachable!(),
        }
    }
}

impl From<&LetterSpacing> for u8 {
    fn from(letterspacing: &LetterSpacing) -> Self {
        match letterspacing {
            LetterSpacing::None => 0,
            LetterSpacing::EightPixels => 1 << 1,
            LetterSpacing::NinePixels => 2 << 1,
        }
    }
}

impl Serialize for LetterSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

struct StringVisitor;

impl<'de> de::Visitor<'de> for StringVisitor {
    type Value = LetterSpacing;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match LetterSpacing::from_str(value) {
            Ok(letterspacing) => Ok(letterspacing),
            Err(err) => Err(E::custom(err)),
        }
    }
}

impl<'de> Deserialize<'de> for LetterSpacing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::sauce::LetterSpacing;
    use serde_json::{from_str, to_string};
    #[test]
    fn de() {
        let letterspacing = LetterSpacing::EightPixels;
        let json = to_string(&letterspacing).unwrap();
        assert_eq!(json, "\"8px\"");
        let de_letterspacing = from_str::<LetterSpacing>(&json).unwrap();
        assert_eq!(letterspacing, de_letterspacing);
    }
}
