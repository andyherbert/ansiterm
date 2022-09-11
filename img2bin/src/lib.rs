use ansiart::{
    codepage437::{ascii, Font},
    ega_palette::EgaPalette,
    sauce::{DataType, FileType, Sauce},
};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Rgba};
use std::{error::Error, fmt::Display, fs::File, io::Write, mem::swap, ops::Not, path::Path};

const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);

#[derive(Clone, Default, Debug)]
pub struct ParsedBlock {
    pub bitmask: Vec<bool>,
    pub fg: Option<Rgba<u8>>,
    pub bg: Option<Rgba<u8>>,
}

#[derive(Debug)]
struct Img2binError;

impl Display for Img2binError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Img2bin Error")
    }
}

impl Error for Img2binError {}

#[derive(Debug)]
pub struct Block {
    code: usize,
    fg: usize,
    bg: usize,
}

impl Not for &ParsedBlock {
    type Output = ParsedBlock;

    fn not(self) -> Self::Output {
        ParsedBlock {
            bitmask: self.bitmask.iter().map(|value| !value).collect(),
            fg: self.bg,
            bg: self.fg,
        }
    }
}

impl PartialEq<Vec<bool>> for ParsedBlock {
    fn eq(&self, other: &Vec<bool>) -> bool {
        &self.bitmask == other
    }
}

fn parse_block(img: &DynamicImage, x: u32, y: u32) -> Result<ParsedBlock, Img2binError> {
    let mut block = ParsedBlock::default();
    for y in y..y + 16 {
        for x in x..x + 8 {
            let pixel = img.get_pixel(x, y);
            match block.fg {
                Some(rgba) if rgba == pixel => {
                    block.bitmask.push(true);
                }
                Some(_) => match block.bg {
                    Some(rgba) if rgba == pixel => block.bitmask.push(false),
                    Some(_) => return Err(Img2binError),
                    None => {
                        block.bg = Some(pixel);
                        block.bitmask.push(false);
                    }
                },
                None => {
                    block.fg = Some(pixel);
                    block.bitmask.push(true);
                }
            }
        }
    }
    Ok(block)
}

fn parse_blocks(img: DynamicImage, font_width: usize) -> Result<Vec<ParsedBlock>, Img2binError> {
    let mut blocks = vec![];
    let (width, height) = img.dimensions();
    let columns = width / font_width as u32;
    let rows = height / 16;
    for row in 0..rows {
        let y = row * 16;
        for column in 0..columns {
            let x = column * font_width as u32;
            blocks.push(parse_block(&img, x, y)?);
        }
    }
    Ok(blocks)
}

fn get_match(
    block: ParsedBlock,
    palette: &EgaPalette,
    font_bitmasks: &[Vec<bool>],
) -> Result<Block, Img2binError> {
    let negative = !&block;
    for (code, font_bitmask) in font_bitmasks.iter().enumerate() {
        if &block == font_bitmask {
            return Ok(Block {
                code,
                fg: palette.closest(&block.fg.expect("fg").0),
                bg: palette.closest(&block.bg.unwrap_or(BLACK).0),
            });
        }
        if let Some(bg) = block.bg {
            if &negative == font_bitmask {
                return Ok(Block {
                    code,
                    fg: palette.closest(&bg.0),
                    bg: palette.closest(&block.fg.expect("fg").0),
                });
            }
        }
    }
    Err(Img2binError)
}

fn get_matches(img: DynamicImage, font_width: usize) -> Result<Vec<Block>, Img2binError> {
    let mut matches = vec![];
    let font_bitmasks = Font::default().to_bitmasks();
    let palette = EgaPalette::cga();
    for block in parse_blocks(img, font_width)? {
        matches.push(get_match(block, &palette, &font_bitmasks)?);
    }
    Ok(matches)
}

pub fn parse_img(
    path: impl AsRef<Path>,
    out: impl AsRef<Path>,
    font_width: usize,
) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?;
    let (width, _) = img.dimensions();
    let mut bytes = vec![];
    let matches = get_matches(img, font_width)?;
    for Block {
        mut code,
        mut fg,
        mut bg,
    } in matches
    {
        if code == 0 {
            if fg == 0 {
                code = ascii::SPACE as usize;
                bg = 0;
            } else {
                code = 219;
                fg = bg;
            }
        }
        if fg < 8 {
            match code {
                176 => {
                    swap(&mut fg, &mut bg);
                    code = 178;
                }
                177 => {
                    swap(&mut fg, &mut bg);
                    code = 177;
                }
                178 => {
                    swap(&mut fg, &mut bg);
                    code = 176;
                }
                220 => {
                    swap(&mut fg, &mut bg);
                    code = 223;
                }
                221 => {
                    swap(&mut fg, &mut bg);
                    code = 222;
                }
                222 => {
                    swap(&mut fg, &mut bg);
                    code = 221;
                }
                223 => {
                    swap(&mut fg, &mut bg);
                    code = 220;
                }
                _ => {}
            }
        }
        bytes.push(code as u8);
        bytes.push((fg + (bg << 4)) as u8);
    }
    let mut file = File::create(&out)?;
    file.write_all(&bytes)?;
    let sauce = Sauce {
        datatype: DataType::BinaryText,
        filetype: FileType::Variable((width / font_width as u32 / 2) as u8),
        filesize: bytes.len(),
        ..Default::default()
    };
    sauce.write(&out)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::parse_img;

    #[test]
    fn it_works() {
        parse_img(
            "/Users/andyh/Desktop/2018-m-10-m-0420-K-Thulu-Blocktober-Ce.png",
            "/Users/andyh/Desktop/2018-m-10-m-0420-K-Thulu-Blocktober-Ce.bin",
            8,
        )
        .unwrap();
    }
}
