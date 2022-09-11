//! # EGA Pallette
//! Represents EGA colors, a palette containing these colors, and methods of extracting RGB data from each color.

#[cfg(test)]
mod tests;

/// Represents red, green, blue, and alpha values
pub type Rgba = [u8; 4];
/// Represents red, green, and blue values
pub type Rgb = [u8; 3];
/// Represents Xyz values
pub type Xyz = [f64; 3];
/// Represents Lab values
pub type Lab = [f64; 3];

// "D65" is a standard 6500K Daylight light source.
// https://en.wikipedia.org/wiki/Illuminant_D65
const D65: Xyz = [95.047, 100.0, 108.883];

/// The order of colors in the [CGA palette](https://en.wikipedia.org/wiki/Color_Graphics_Adapter)
pub static CGA_ORDER: [u8; 16] = [0, 1, 2, 3, 4, 5, 20, 7, 56, 57, 58, 59, 60, 61, 62, 63];
/// The order of colors used for [ANSI Escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
pub static ANSI_ORDER: [u8; 16] = [0, 4, 2, 20, 1, 5, 3, 7, 56, 60, 58, 62, 57, 61, 59, 63];
/// RGBA values for black
pub static BLACK_RGBA: Rgba = [0, 0, 0, 255];
/// RGBA values for white
pub static WHITE_RGBA: Rgba = [255, 255, 255, 255];

// Coverts 6 bit binary representation to 8 bit RGB values
// The binary representation is in the form "rgbRGB" where the lowercase letters are the low-intensity bits, and uppercase letters are high-intensity bits
fn convert_ega_to_rgb(value: u8) -> (u8, u8, u8) {
    let blue = (((value & 0b001000) >> 3) + ((value & 0b000001) << 1)) * 0x55;
    let green = (((value & 0b010000) >> 4) + (value & 0b000010)) * 0x55;
    let red = (((value & 0b100000) >> 5) + ((value & 0b000100) >> 1)) * 0x55;
    (red as u8, green as u8, blue as u8)
}

// Returns RGBA information for the specified 6 bit EGA value
fn ega_to_rgba(value: u8) -> Rgba {
    let (red, green, blue) = convert_ega_to_rgb(value);
    [red, green, blue, 255]
}

// Returns RGB information for the specified 6 bit EGA value
fn ega_to_rgb(value: u8) -> Rgb {
    let (red, green, blue) = convert_ega_to_rgb(value);
    [red, green, blue]
}

// Returns XYZ information for the specified RGB value
// https://www.image-engineering.de/library/technotes/958-how-to-convert-between
fn rgb_to_xyz(rgb: Rgb) -> Xyz {
    let rgb = rgb.map(|value| {
        let v = value as f64 / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    });
    [
        ((0.4124564 * rgb[0]) + (0.3575761 * rgb[1]) + (0.1804375 * rgb[2])) * 100.0,
        ((0.2126729 * rgb[0]) + (0.7151522 * rgb[1]) + (0.0721750 * rgb[2])) * 100.0,
        ((0.0193339 * rgb[0]) + (0.1191920 * rgb[1]) + (0.9503041 * rgb[2])) * 100.0,
    ]
}

// Returns LAB information for the specified XYZ value
// http://www.easyrgb.com/en/math.php
fn xyz_to_lab(mut xyz: Xyz) -> Lab {
    for (value, reference) in xyz.iter_mut().zip(D65.iter()) {
        *value /= reference;
        if *value > 0.008856 {
            *value = (*value).powf(1.0 / 3.0);
        } else {
            *value = *value * 7.787 + 16.0 / 116.0;
        }
    }
    [
        116.0 * xyz[1] - 16.0,
        500.0 * (xyz[0] - xyz[1]),
        200.0 * (xyz[1] - xyz[2]),
    ]
}

/// Represents an EGA color
#[derive(Debug, Clone)]
pub struct EgaColor {
    /// The 6 bit EGA value
    pub value: u8,
    /// The RGBA values
    pub rgba: Rgba,
    /// The RGB value
    pub rgb: Rgb,
    /// The XYZ value
    pub xyz: Xyz,
    /// The LAB value
    pub lab: Lab,
}

impl EgaColor {
    /// Constructs a new EGA Color based on the supplied 6 bit value
    pub fn new(value: u8) -> EgaColor {
        let rgba = ega_to_rgba(value);
        let rgb = ega_to_rgb(value);
        let xyz = rgb_to_xyz(rgb);
        let lab = xyz_to_lab(xyz);
        EgaColor {
            value,
            rgba,
            rgb,
            xyz,
            lab,
        }
    }
}

/// Represents a collection of EGA Colors, not limited to 16
#[derive(Clone, Debug)]
pub struct EgaPalette {
    colors: Vec<EgaColor>,
}

/// Returns a reference to an element depending on the type of index.
impl std::ops::Index<usize> for EgaPalette {
    type Output = EgaColor;

    fn index(&self, index: usize) -> &Self::Output {
        &self.colors[index]
    }
}

impl EgaPalette {
    /// Creates a new palette depending on the values given in the supplied slice
    pub fn new(indexes: &[u8]) -> EgaPalette {
        let colors = indexes.iter().map(|index| EgaColor::new(*index)).collect();
        EgaPalette { colors }
    }

    /// Constructs a new palette with the color ordering expected for ANSI escape sequences
    pub fn ansi() -> EgaPalette {
        EgaPalette::new(&ANSI_ORDER)
    }

    /// Constructs a new palette with the color ordering expected for the default CGA palette
    pub fn cga() -> EgaPalette {
        EgaPalette::new(&CGA_ORDER)
    }

    /// Constructs a new palette with a complette complement of possible EGA values 0-63
    pub fn all() -> EgaPalette {
        let indexes: Vec<u8> = (0..64).into_iter().collect();
        EgaPalette::new(indexes.as_slice())
    }

    /// Returns the number of elements in the palette
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Returns `true` of the palette contains no colors
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Returns a reference to an element depending on the type of index.
    pub fn get(&self, index: usize) -> Option<&EgaColor> {
        self.colors.get(index)
    }

    /// Returns the index (or None) of a specific Rgba
    pub fn index_of(&self, find: &Rgba) -> Option<usize> {
        self.colors.iter().position(|col| &col.rgba == find)
    }

    /// Returns the closest colour of a specific Rgba (alpha is ignored)
    pub fn closest(&self, find: &Rgba) -> usize {
        let rgb = [find[0], find[1], find[2]];
        let xyz = rgb_to_xyz(rgb);
        let lab = xyz_to_lab(xyz);
        let mut closest: Option<(usize, f64)> = None;
        for (index, col) in self.colors.iter().enumerate() {
            let value = ((lab[0] - col.lab[0]).powf(2.0)
                + (lab[1] - col.lab[1]).powf(2.0)
                + (lab[2] - col.lab[2]).powf(2.0))
            .sqrt();
            match closest {
                Some((_, prev)) if value < prev => closest = Some((index, value)),
                None => closest = Some((index, value)),
                _ => continue,
            }
        }
        closest.expect("match").0
    }
}

impl Default for EgaPalette {
    /// Gives the same result as `EgaPalette::cga()`
    fn default() -> Self {
        EgaPalette::cga()
    }
}
