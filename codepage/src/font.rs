use super::CP437_F16;

pub struct Font {
    pub bytes: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Default for Font {
    fn default() -> Self {
        Font::new(CP437_F16)
    }
}

impl Font {
    pub fn new(bytes: &[u8]) -> Self {
        if bytes.len() % 256 != 0 {
            unreachable!("Illegal font length");
        }
        let width = 8;
        let height = bytes.len() / 256;
        if !(8..=32).contains(&height) {
            unreachable!("Illegal font height");
        }
        Self {
            bytes: bytes.to_vec(),
            width,
            height,
        }
    }

    pub fn to_vec(&self, byte: u8, fg_rgba: &[u8; 4], bg_rgba: &[u8; 4]) -> Vec<[u8; 4]> {
        let mut font_bytes = Vec::with_capacity(self.width * self.height * 4);
        let offset = byte as usize * self.height;
        for byte in &self.bytes[offset..offset + self.height] {
            for bit_position in (0..self.width).rev() {
                match byte & (1 << bit_position) {
                    0 => font_bytes.push(bg_rgba.to_owned()),
                    _ => font_bytes.push(fg_rgba.to_owned()),
                }
            }
        }
        font_bytes
    }
}
