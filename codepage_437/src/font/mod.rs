mod draw_font;
mod font_error;
/// Raw binary data for various bitmap fonts
pub mod raw;
pub use draw_font::DrawFont;
use ega_palette::{Rgba, BLACK_RGBA, WHITE_RGBA};
pub use font_error::FontError;
use image::{DynamicImage, ImageBuffer, Pixel};

/// Structure to hold, read, write, and generate rgba data for fonts
#[derive(Clone, Debug, PartialEq)]
pub struct Font {
    pub bytes: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Default for Font {
    /// Equivalent to IBM Codepage 437 8x16
    fn default() -> Self {
        Font::try_from(raw::CP437_F16.as_ref()).expect("font")
    }
}

impl Font {
    /// Attemptes to read an image constructed from a 16x16 grid, with each glyph 8 pixels wide
    pub fn read_image(path: &str) -> Result<Font, FontError> {
        match image::io::Reader::open(path) {
            Ok(reader) => match reader.decode() {
                Ok(image) => Font::try_from(image),
                Err(_err) => Err(FontError::CannotReadImage),
            },
            Err(_err) => Err(FontError::CannotReadFile),
        }
    }

    /// Attemptes to write an image using a 16x16 grid arrangement
    pub fn write_image(&self, path: &str) -> Result<(), FontError> {
        let buffer = ImageBuffer::try_from(self)?;
        match buffer.save(path) {
            Ok(()) => Ok(()),
            Err(_err) => Err(FontError::CannotWriteImage),
        }
    }

    /// Attempts to write bitmask data to a file
    pub fn write(&self, path: &str) -> Result<(), FontError> {
        match std::fs::write(path, &self.bytes) {
            Ok(()) => Ok(()),
            Err(_err) => Err(FontError::CannotWriteFile),
        }
    }

    /// Generates RGBA data for a [Font] using fg and bg data
    pub fn to_bytes(&self, code: u8, fg_rgba: &Rgba, bg_rgba: &Rgba) -> Vec<u8> {
        let mut font_bytes = Vec::with_capacity(self.width * self.height);
        let offset = code as usize * self.height;
        for byte in &self.bytes[offset..offset + self.height] {
            for bit_position in (0..self.width).rev() {
                match byte & (1 << bit_position) {
                    0 => font_bytes.extend_from_slice(bg_rgba),
                    _ => font_bytes.extend_from_slice(fg_rgba),
                }
            }
        }
        font_bytes
    }
}

impl TryFrom<DynamicImage> for Font {
    type Error = FontError;

    fn try_from(image: DynamicImage) -> Result<Self, Self::Error> {
        let image = image.into_rgba8();
        let (image_width, image_height) = image.dimensions();
        if image_width != 128 {
            return Err(FontError::IllegalFontSize);
        }
        if image_height % 16 != 0 {
            return Err(FontError::IllegalFontSize);
        }
        let font_height = image_height / 16;
        let mut bitmask = Vec::with_capacity(16 * image_height as usize);
        for glyph_y in (0..image_height).step_by(font_height as usize) {
            for glyph_x in (0..image_width).step_by(8) {
                for y in glyph_y..glyph_y + font_height {
                    let mut byte: u8 = 0;
                    for x in glyph_x..glyph_x + 8 {
                        let (red, green, blue, _) = image[(x, y)].channels4();
                        byte <<= 1;
                        if red > 127 && green > 127 && blue > 127 {
                            byte += 1;
                        }
                    }
                    bitmask.push(byte);
                }
            }
        }
        Font::try_from(bitmask.as_slice())
    }
}

impl TryFrom<&Font> for ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    type Error = FontError;

    fn try_from(font: &Font) -> Result<Self, Self::Error> {
        let width = font.width * 16;
        let height = font.height * 16;
        let mut buffer = vec![0; width * height * 4];
        let mut code = 0;
        for y in (0..height).step_by(font.height) {
            for x in (0..width).step_by(font.width) {
                let rgba = font.to_bytes(code, &WHITE_RGBA, &BLACK_RGBA);
                buffer.draw_font(x, y, font.width, font.width * 16, &rgba);
                if code < 255 {
                    code += 1;
                }
            }
        }
        match ImageBuffer::from_vec(width as u32, height as u32, buffer) {
            Some(buffer) => Ok(buffer),
            None => Err(FontError::CannotWriteImage),
        }
    }
}

impl TryFrom<&[u8]> for Font {
    type Error = FontError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() % 256 != 0 {
            return Err(FontError::IllegalFontSize);
        }
        let height = bytes.len() / 256;
        if !(8..=32).contains(&height) {
            return Err(FontError::IllegalFontHeight);
        }
        Ok(Font {
            bytes: bytes.to_vec(),
            width: 8,
            height,
        })
    }
}
