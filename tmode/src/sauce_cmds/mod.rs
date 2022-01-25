mod auto;
mod export;
mod import;
use chrono::{Datelike, Local};
use clap::{AppSettings, Parser};
use codepage437::CP437String;
use sauce::{AspectRatio, Comments, InfoS, LetterSpacing, Sauce};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum SauceCommand {
    /// Changes the aspect-ratio setting to 'Legacy'.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    AspectLegacy {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Changes the aspect-ratio setting to 'Modern'.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    AspectModern {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Changes the aspect-ratio setting to no-preference.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    AspectNone {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Adds an author to SAUCE records.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Author {
        #[clap(required = true, value_name = "author's name")]
        author: String,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Automatically insert a SAUCE record for non-textmode files.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Auto {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Adds comments to SAUCE records.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Comments {
        #[clap(required = true, value_name = "comments")]
        comments: String,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Sets today's date.x
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Date {
        #[clap(required = true, value_name = "comments")]
        date: String,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Sets today's date.x
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    DateNow {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Exports multiple SAUCE records to a CSV or JSON file.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Export {
        #[clap(required = true, value_name = "csv or json file")]
        export_path: PathBuf,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },

    /// Adds a font name to SAUCE records.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Font {
        #[clap(required = true, value_name = "font name")]
        font: String,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Adds a group to SAUCE records.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Group {
        #[clap(required = true, value_name = "group's name")]
        group: String,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Enables iCE colors for supported filetypes.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    IceOn {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Disables iCE colors for supported filetypes.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    IceOff {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Imports a CSV file to update multiple SAUCE records.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Import {
        #[clap(required = true, value_name = "csv or json file")]
        import_path: PathBuf,
    },
    /// Sets a numeric value to the information group 1.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Info1 {
        /// Sets a numeric value to information 1.
        #[clap(required = true, value_name = "value")]
        value: u16,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Sets a numeric value to the information group 2.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Info2 {
        /// Sets a numeric value to information 1.
        #[clap(required = true, value_name = "value")]
        value: u16,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Sets a numeric value to the information group 3.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Info3 {
        /// Sets a numeric value to information 1.
        #[clap(required = true, value_name = "value")]
        value: u16,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Sets a numeric value to the information group 4.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Info4 {
        /// Sets a numeric value to information 1.
        #[clap(required = true, value_name = "value")]
        value: u16,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Enables 9px letter spacing for supported filetypes.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    LetterSpacingOn {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Enables 8px letter spacing for supported filetypes.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    LetterSpacingOff {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Sets no preference on letter spacing setting for supported filetypes.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    LetterSpacingNone {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Removes SAUCE records.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Remove {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Displays SAUCE information.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Show {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Adds a title to the SAUCE records.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Title {
        #[clap(required = true, value_name = "title")]
        title: String,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
}

fn for_every_sauce(files: Vec<PathBuf>, fun: impl Fn(&mut Sauce)) {
    for ref path in files {
        match Sauce::read(path) {
            Err(err) => eprintln!("{err}"),
            Ok(None) => eprintln!("No SAUCE set for {}", path.to_string_lossy()),
            Ok(Some(mut sauce)) => {
                fun(&mut sauce);
                match sauce.write(path) {
                    Ok(()) => println!("Updated: {}", path.to_string_lossy()),
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn main(sauce_cmd: SauceCommand) {
    match sauce_cmd {
        SauceCommand::AspectLegacy { files } => {
            for_every_sauce(files, |sauce| sauce.aspect_ratio = AspectRatio::Legacy);
        }
        SauceCommand::AspectModern { files } => {
            for_every_sauce(files, |sauce| sauce.aspect_ratio = AspectRatio::Modern);
        }
        SauceCommand::AspectNone { files } => {
            for_every_sauce(files, |sauce| sauce.aspect_ratio = AspectRatio::None);
        }
        SauceCommand::Author { author, files } => {
            if author.len() > 20 {
                eprintln!("Too many characters in 'author', use 20 characters or less");
            } else {
                match CP437String::from_str(&author) {
                    Err(err) => eprintln!("{err}"),
                    Ok(author) => for_every_sauce(files, |sauce| sauce.author = author.clone()),
                }
            }
        }
        SauceCommand::Auto { files } => auto::auto(files),
        SauceCommand::Comments { comments, files } => {
            if comments.len() > 255 * 64 {
                eprintln!(
                    "Too many characters in 'comments', use {} characters or less",
                    255 * 64
                );
            } else {
                let strings = comments
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(64)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>();
                match Comments::try_from(&strings) {
                    Err(err) => eprintln!("{err}"),
                    Ok(comments) => {
                        for_every_sauce(files, |sauce| sauce.comments = comments.clone());
                    }
                }
            }
        }
        SauceCommand::Date { date, files } => {
            if date.len() > 8 {
                eprintln!("Too many characters in 'Date', use 8 characters or less");
            } else {
                match CP437String::from_str(&date) {
                    Err(err) => eprintln!("{err}"),
                    Ok(date) => for_every_sauce(files, |sauce| sauce.date = date.clone()),
                }
            }
        }
        SauceCommand::DateNow { files } => {
            let time = Local::now();
            let (_bce, year) = time.year_ce();
            let month = time.month();
            let day = time.day();
            let string = format!("{year:04}{month:02}{day:02}");
            let date = CP437String::from_str(&string).expect("legal string");
            for_every_sauce(files, |sauce| sauce.date = date.clone())
        }
        SauceCommand::Export { export_path, files } => export::export(export_path, files),
        SauceCommand::Font { font, files } => {
            if font.len() > 22 {
                eprintln!("Too many characters in 'TInfoS', use 22 characters or less");
            } else {
                match CP437String::from_str(&font) {
                    Err(err) => eprintln!("{err}"),
                    Ok(font) => for_every_sauce(files, |sauce| sauce.info_s = InfoS::from(&font)),
                }
            }
        }
        SauceCommand::Group { group, files } => {
            if group.len() > 20 {
                eprintln!("Too many characters in 'group', use 20 characters or less");
            } else {
                match CP437String::from_str(&group) {
                    Err(err) => eprintln!("{err}"),
                    Ok(group) => for_every_sauce(files, |sauce| sauce.group = group.clone()),
                }
            }
        }
        SauceCommand::IceOn { files } => for_every_sauce(files, |sauce| sauce.ice_colors = true),
        SauceCommand::IceOff { files } => for_every_sauce(files, |sauce| sauce.ice_colors = false),
        SauceCommand::Import { import_path } => import::import(import_path),
        SauceCommand::Info1 { value, files } => {
            for_every_sauce(files, |sauce| sauce.info_1 = value as usize)
        }
        SauceCommand::Info2 { value, files } => {
            for_every_sauce(files, |sauce| sauce.info_2 = value as usize)
        }
        SauceCommand::Info3 { value, files } => {
            for_every_sauce(files, |sauce| sauce.info_3 = value as usize)
        }
        SauceCommand::Info4 { value, files } => {
            for_every_sauce(files, |sauce| sauce.info_4 = value as usize)
        }
        SauceCommand::LetterSpacingOn { files } => for_every_sauce(files, |sauce| {
            sauce.letter_spacing = LetterSpacing::NinePixels
        }),
        SauceCommand::LetterSpacingOff { files } => for_every_sauce(files, |sauce| {
            sauce.letter_spacing = LetterSpacing::EightPixels
        }),
        SauceCommand::LetterSpacingNone { files } => {
            for_every_sauce(files, |sauce| sauce.letter_spacing = LetterSpacing::None);
        }
        SauceCommand::Remove { files } => {
            for ref path in files {
                match Sauce::remove(path) {
                    Err(err) => eprintln!("{err}"),
                    Ok(()) => println!("Removed: {}", path.to_string_lossy()),
                }
            }
        }
        SauceCommand::Show { files } => for_every_sauce(files, |sauce| print!("{sauce}")),
        SauceCommand::Title { title, files } => {
            if title.len() > 35 {
                eprintln!("Too many characters in 'title', use 35 characters or less");
            } else {
                match CP437String::from_str(&title) {
                    Err(err) => eprintln!("{err}"),
                    Ok(title) => for_every_sauce(files, |sauce| sauce.title = title.clone()),
                }
            }
        }
    }
}
