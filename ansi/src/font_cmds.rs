// use ansiart::codepage437::{Font, InfoS};
use ansiart::{codepage437::Font, sauce::InfoS};
use clap::{AppSettings, Parser};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum FontCommand {
    /// Converts a bitmask font file to a PNG file.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    ToPng {
        #[clap(required = true, parse(from_os_str), value_name = "FON file")]
        fon: PathBuf,
        #[clap(required = true, parse(from_os_str), value_name = "PNG file")]
        png: PathBuf,
    },
    /// Converts a PNG file (16x16 arrangement) to a FON file.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    ToFon {
        #[clap(required = true, parse(from_os_str), value_name = "FON file")]
        png: PathBuf,
        #[clap(required = true, parse(from_os_str), value_name = "PNG file")]
        fon: PathBuf,
    },
    /// Converts a SAUCE font definition, e.g. "IBM VGA" to a PNG file.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    SauceToPng {
        #[clap(required = true, value_name = "String")]
        info_s: String,
        #[clap(required = true, parse(from_os_str), value_name = "PNG file")]
        png: PathBuf,
    },
    /// Converts a SAUCE font definition, e.g. "IBM VGA" to a FON file.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    SauceToFon {
        #[clap(required = true, value_name = "String")]
        info_s: String,
        #[clap(required = true, parse(from_os_str), value_name = "FON file")]
        fon: PathBuf,
    },
}

pub fn font_cmds(font_cmd: FontCommand) {
    match font_cmd {
        FontCommand::ToPng { fon, png } => match Font::read(fon) {
            Err(err) => eprintln!("{err}"),
            Ok(font) => {
                if let Err(err) = font.write_image(png) {
                    eprintln!("{err}");
                }
            }
        },
        FontCommand::ToFon { png, fon } => match Font::read_image(png) {
            Err(err) => eprintln!("{err}"),
            Ok(font) => {
                if let Err(err) = font.write(fon) {
                    eprintln!("{err}");
                }
            }
        },
        FontCommand::SauceToPng { info_s, png } => match InfoS::from_str(&info_s) {
            Err(err) => eprintln!("{err}"),
            Ok(info_s) => match Font::try_from(info_s) {
                Err(err) => eprintln!("{err}"),
                Ok(font) => {
                    if let Err(err) = font.write_image(png) {
                        eprintln!("{err}");
                    }
                }
            },
        },
        FontCommand::SauceToFon { info_s, fon } => match InfoS::from_str(&info_s) {
            Err(err) => eprintln!("{err}"),
            Ok(info_s) => match Font::try_from(info_s) {
                Err(err) => eprintln!("{err}"),
                Ok(font) => {
                    if let Err(err) = font.write(fon) {
                        eprintln!("{err}");
                    }
                }
            },
        },
    }
}
