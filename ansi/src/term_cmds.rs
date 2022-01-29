use ansiart::AnsiParser;
use ansiterm::{terminal, TerminalEvent};
use clap::{AppSettings, Parser};
use std::{fs, path::PathBuf};
use stdin_receiver::StdInReceiver;

#[derive(Debug, Parser)]
pub enum TermCommand {
    /// Shows ANSI Art in a simulated terminal
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Show {
        /// Throttle input with at a specific baud rate
        #[clap(short = 'b', default_value = "14400", value_name = "baud rate")]
        baud_rate: usize,
        /// Terminal width in columns
        #[clap(short = 'c', default_value = "80", value_name = "columns")]
        columns: usize,
        /// Terminal height in rows
        #[clap(short = 'r', default_value = "25", value_name = "rows")]
        rows: usize,
        /// Ice colors
        #[clap(short = 'i')]
        ice_colors: bool,
        /// Scale
        #[clap(short = 's', default_value = "2", value_name = "scale")]
        scale: usize,
        #[clap(required = true, parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    /// Shows ANSI Art in a simulated terminal
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    StdIn {
        /// Throttle input with at a specific baud rate
        #[clap(short = 'b', default_value = "14400", value_name = "baud rate")]
        baud_rate: usize,
        /// Terminal width in columns
        #[clap(short = 'c', default_value = "80", value_name = "columns")]
        columns: usize,
        /// Terminal height in rows
        #[clap(short = 'r', default_value = "25", value_name = "rows")]
        rows: usize,
        /// Ice colors
        #[clap(short = 'i')]
        ice_colors: bool,
        /// Scale
        #[clap(short = 's', default_value = "2", value_name = "scale")]
        scale: usize,
    },
}

pub fn term_cmds(term_cmd: TermCommand) -> ! {
    match term_cmd {
        TermCommand::Show {
            baud_rate,
            columns,
            rows,
            ice_colors,
            scale,
            files,
        } => {
            let mut parser = AnsiParser::with_baud(baud_rate);
            for path in files {
                match fs::read(&path) {
                    Ok(bytes) => parser.input(bytes),
                    Err(_) => {
                        eprint!(
                            "An error  occured whilst attempting to read {}",
                            path.to_string_lossy()
                        )
                    }
                }
            }
            terminal(parser, columns, rows, scale, ice_colors, |_, _, _| {});
        }
        TermCommand::StdIn {
            baud_rate,
            columns,
            rows,
            ice_colors,
            scale,
        } => {
            let parser = AnsiParser::with_baud(baud_rate);
            let mut std_reciever = Some(StdInReceiver::default());
            terminal(
                parser,
                columns,
                rows,
                scale,
                ice_colors,
                move |parser, event, _| match event {
                    TerminalEvent::RedrawRequested => {
                        if let Some(ref std_receiver) = std_reciever {
                            if let Some(bytes) = std_receiver.recv().expect("Thread error") {
                                parser.input(bytes)
                            }
                        }
                    }
                    TerminalEvent::CloseRequested => {
                        if let Some(std_receiver) = std_reciever.take() {
                            std_receiver.join().expect("Thread error");
                        }
                    }
                    _ => {}
                },
            );
        }
    }
}
