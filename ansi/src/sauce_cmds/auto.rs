use ansiart::sauce::{DataType, FileType, Sauce};
use std::path::{Path, PathBuf};

fn update_or_create_sauce(path: &Path, fun: impl FnOnce(&mut Sauce)) {
    let mut sauce = match Sauce::read(path) {
        Err(err) => {
            eprintln!("{err}");
            return;
        }
        Ok(Some(sauce)) => sauce,
        Ok(None) => Sauce::new(),
    };
    fun(&mut sauce);
    match sauce.write(path) {
        Err(err) => eprintln!("{err}"),
        Ok(()) => println!("Updated: {}", path.to_string_lossy()),
    }
}

fn auto_image(path: &Path, filetype: FileType) {
    match image::io::Reader::open(path) {
        Err(_err) => eprintln!(
            "An error occured whilst attempting to read {}",
            path.to_string_lossy()
        ),
        Ok(reader) => match reader.into_dimensions() {
            Err(_err) => eprintln!(
                "An error occured whilst attempting to parse {}",
                path.to_string_lossy()
            ),
            Ok((width, height)) => update_or_create_sauce(path, |sauce| {
                sauce.datatype = DataType::Bitmap;
                sauce.filetype = filetype;
                sauce.info_1 = width as usize;
                sauce.info_2 = height as usize;
            }),
        },
    }
}

pub fn auto(files: Vec<PathBuf>) {
    for ref path in files {
        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {
                match extension.to_uppercase().as_str() {
                    "GIF" => auto_image(path, FileType::Gif),
                    "JPG" | "JPEG " => auto_image(path, FileType::Jpg),
                    "PNG" => auto_image(path, FileType::Png),
                    "IT" => update_or_create_sauce(path, |sauce| {
                        sauce.datatype = DataType::Audio;
                        sauce.filetype = FileType::It;
                    }),
                    "MP4" => update_or_create_sauce(path, |sauce| {
                        sauce.datatype = DataType::Bitmap;
                        sauce.filetype = FileType::Mpg;
                    }),
                    "RIP" => update_or_create_sauce(path, |sauce| {
                        sauce.datatype = DataType::Character;
                        sauce.filetype = FileType::RipScript;
                    }),
                    "S3M" => update_or_create_sauce(path, |sauce| {
                        sauce.datatype = DataType::Audio;
                        sauce.filetype = FileType::S3m;
                    }),
                    _ => eprintln!("Skipped: {}", path.to_string_lossy()),
                }
            }
        }
    }
}
