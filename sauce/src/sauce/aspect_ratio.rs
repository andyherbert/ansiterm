use crate::SauceError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AspectRatio {
    Undefined,
    Modern,
    Legacy,
}

impl Default for AspectRatio {
    fn default() -> AspectRatio {
        AspectRatio::Modern
    }
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AspectRatio::Undefined => write!(f, "Undefined")?,
            AspectRatio::Modern => write!(f, "Modern")?,
            AspectRatio::Legacy => write!(f, "Legacy")?,
        }
        Ok(())
    }
}

impl FromStr for AspectRatio {
    type Err = SauceError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "Undefined" => Ok(AspectRatio::Undefined),
            "Modern" => Ok(AspectRatio::Modern),
            "Legacy" => Ok(AspectRatio::Legacy),
            _ => Err(SauceError::InvalidAspectRatioValue),
        }
    }
}

impl From<u8> for AspectRatio {
    fn from(value: u8) -> Self {
        match (value >> 3) & 3 {
            0 => AspectRatio::Undefined,
            1 => AspectRatio::Legacy,
            2 => AspectRatio::Modern,
            _ => unreachable!(),
        }
    }
}

impl From<&AspectRatio> for u8 {
    fn from(aspect_ratio: &AspectRatio) -> Self {
        match aspect_ratio {
            AspectRatio::Undefined => 0,
            AspectRatio::Modern => 2 << 3,
            AspectRatio::Legacy => 1 << 3,
        }
    }
}
