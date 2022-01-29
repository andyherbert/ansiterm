use ansiart::sauce::Sauce;
use std::{fs, path::PathBuf};

fn update_sauce(sauce: &Sauce) {
    if let Some(ref sauce_path) = sauce.path {
        match sauce.write(sauce_path) {
            Err(err) => eprintln!("{err}"),
            Ok(()) => println!("Updated: {}", sauce_path.to_string_lossy()),
        }
    }
}

fn import_json(path: PathBuf) {
    match fs::File::open(&path) {
        Err(_err) => {
            eprintln!(
                "An error occured whilst attempting to read: {}",
                path.to_string_lossy()
            )
        }
        Ok(rdr) => match serde_json::from_reader::<fs::File, Vec<Sauce>>(rdr) {
            Err(_err) => {
                eprintln!(
                    "An error occured whilst attempting to interpret: {}",
                    path.to_string_lossy()
                )
            }
            Ok(sauces) => sauces.iter().for_each(update_sauce),
        },
    }
}

fn import_csv(path: PathBuf) {
    match csv::Reader::from_path(&path) {
        Err(_err) => eprintln!(
            "An error occured whilst attempting to read: {}",
            path.to_string_lossy()
        ),
        Ok(mut rdr) => {
            for result in rdr.deserialize::<Sauce>() {
                match result {
                    Err(_err) => {
                        eprintln!(
                            "An error occured whilst attempting to interpret: {}",
                            path.to_string_lossy()
                        )
                    }
                    Ok(sauce) => update_sauce(&sauce),
                }
            }
        }
    }
}

pub fn import(path: PathBuf) {
    match path.extension() {
        None => eprintln!("Could not determine input format based on lack of extension, use 'csv' or 'json'"),
        Some(ext) => match ext.to_str() {
            None => eprintln!("Error decoding import filename string."),
            Some(ext) => match ext.to_ascii_lowercase().as_str() {
                "json" => import_json(path),
                "csv" => import_csv(path),
                _ => eprintln!("Could not determine input format based on extension {ext}, use 'csv' or 'json'")
            },
        },
    }
}
