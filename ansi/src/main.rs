use clap::{AppSettings, Parser};
use std::path::PathBuf;
mod font_cmds;
mod music_cmds;
mod sauce_cmds;
mod term_cmds;
use font_cmds::FontCommand;
use img2bin::parse_img;
use music_cmds::MusicCommand;
use sauce_cmds::SauceCommand;
use term_cmds::TermCommand;

#[derive(Parser)]
#[clap(author, version, about)]
enum Commands {
    /// Commands to manipulate SAUCE records
    #[clap(subcommand)]
    Sauce(SauceCommand),
    /// Commands to manipulate FON files
    #[clap(subcommand)]
    Font(FontCommand),
    /// Commands to manipulate ANSI Music
    #[clap(subcommand)]
    Music(MusicCommand),
    /// Commands to display ANSI Art in a simulated terminal
    #[clap(subcommand)]
    Term(TermCommand),
    /// Attempts to convert an image to a BinaryText File.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Img2bin {
        /// Displaying the notes whilst playing the ANSI Music
        #[clap(short = 'n', long)]
        nine_px: bool,
        #[clap(required = true, parse(from_os_str))]
        img: PathBuf,
        #[clap(required = true, parse(from_os_str))]
        bin: PathBuf,
    },
}

fn main() {
    match Commands::parse() {
        Commands::Sauce(sauce_cmd) => sauce_cmds::sauce_cmds(sauce_cmd),
        Commands::Font(font_cmd) => font_cmds::font_cmds(font_cmd),
        Commands::Music(music_cmd) => music_cmds::music_cmds(music_cmd),
        Commands::Term(term_cmd) => term_cmds::term_cmds(term_cmd),
        Commands::Img2bin { nine_px, img, bin } => {
            if parse_img(img, bin, if nine_px { 9 } else { 8 }).is_err() {
                eprintln!("An error occured whilst attempting to convert the image");
            }
        }
    }
}
