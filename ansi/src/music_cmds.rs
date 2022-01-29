use ansiart::ansiplay::{
    rodio::{OutputStream, Sink},
    Music, Player,
};
use ansiart::AnsiParser;
use clap::{AppSettings, Parser};
use std::{
    io::{self, Write},
    path::PathBuf,
};

#[derive(Debug, Parser)]
pub enum MusicCommand {
    /// Interprets a string an plays the sequence.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Parse {
        /// Displaying the notes whilst playing the ANSI Music
        #[clap(short = 's', long)]
        show: bool,
        #[clap(value_name = "input")]
        string: String,
    },
    /// Plays ANSI Music.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Play {
        /// Displaying the notes whilst playing the ANSI Music
        #[clap(short = 's', long)]
        show: bool,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Shows all parsed ANSI Music sequences.
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Show {
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
}

fn play_music(player: &mut Player, music: Music, show: bool, sink: &Sink) {
    for (index, entity) in music.into_iter().enumerate() {
        if show {
            if index > 0 {
                print!(" ");
            }
            print!("{entity}");
            io::stdout().flush().ok();
        }
        player.play_entity(entity, sink);
    }
}

pub fn music_cmds(music_cmd: MusicCommand) {
    match music_cmd {
        MusicCommand::Parse { show, string } => {
            match OutputStream::try_default() {
                Err(_err) => {
                    eprintln!("An error occured whilst attempting to create an audio stream");
                }
                Ok((_stream, stream_handle)) => match Sink::try_new(&stream_handle) {
                    Err(_) => {
                        eprintln!("An error occured whilst attempting to create an audio stream")
                    }
                    Ok(ref sink) => {
                        let mut player = Player::new();
                        let music = Music::from(string.as_str());
                        play_music(&mut player, music, show, sink);
                    }
                },
            };
        }
        MusicCommand::Play { files, show } => {
            match OutputStream::try_default() {
                Err(_err) => {
                    eprintln!("An error occured whilst attempting to create an audio stream");
                }
                Ok((_stream, stream_handle)) => match Sink::try_new(&stream_handle) {
                    Err(_) => {
                        eprintln!("An error occured whilst attempting to create an audio stream")
                    }
                    Ok(ref sink) => {
                        for path in files {
                            let mut player = Player::new();
                            match AnsiParser::read(&path) {
                                Err(_err) => eprintln!(
                                    "An error occured whilst attempting to read {}",
                                    path.to_string_lossy()
                                ),
                                Ok(parser) => {
                                    for sequence in parser {
                                        if let ansiart::Sequence::Music(music) = sequence {
                                            play_music(&mut player, music, show, sink);
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            };
        }
        MusicCommand::Show { files } => {
            for path in files {
                match AnsiParser::read(&path) {
                    Err(_err) => eprintln!(
                        "An error occured whilst attempting to read {}",
                        path.to_string_lossy()
                    ),
                    Ok(parser) => {
                        for sequence in parser {
                            if let ansiart::Sequence::Music(music) = sequence {
                                println!("{music}");
                            }
                        }
                    }
                }
            }
        }
    }
}
