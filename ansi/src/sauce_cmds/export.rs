use ansiart::sauce::Sauce;
use std::{fs, path::PathBuf};

pub fn export(path: PathBuf, files: Vec<PathBuf>) {
    let sauce = files
        .iter()
        .filter_map(|file| match Sauce::read(file) {
            Ok(Some(sauce)) => Some(sauce),
            _ => None,
        })
        .collect::<Vec<Sauce>>();
    match path.extension() {
        None => eprintln!("Could not determine output format based on lack of extension, use 'csv' or 'json'"),
        Some(ext) => match ext.to_str() {
            None => eprintln!("Error decoding export filename string."),
            Some(ext) => match ext.to_ascii_lowercase().as_str() {
                "json" => {
                    match fs::File::create(&path) {
                        Ok(writer) => {
                            if serde_json::to_writer_pretty(writer, &sauce).is_err() {
                                eprintln!("An error occured whilst attempting to write {}", path.to_string_lossy());
                            }
                        },
                        Err(err) => eprintln!("{err}"),
                    }
                }
                "csv" => {
                    match fs::File::create(&path) {
                        Ok(file) => {
                            let mut wtr = csv::Writer::from_writer(file);
                            for sauce in sauce {
                                match sauce.path {
                                    Some(ref path) => {
                                        if wtr.serialize(&sauce).is_err() {
                                            eprintln!("An error occured whilst attempting to serialize from {}", path.to_string_lossy())
                                        }
                                    },
                                    None => unreachable!(),
                                }
                            }
                            if wtr.flush().is_err() {
                                eprintln!("An error occured whilst attempting to write {}", path.to_string_lossy());
                            };
                        },
                        Err(_err) => eprintln!("An error occured whilst attempting to write {}", path.to_string_lossy()),
                    };
                }
                _ => eprintln!("Could not determine output format based on extension {ext}, use 'csv' or 'json'")
            },
        },
    }
}
