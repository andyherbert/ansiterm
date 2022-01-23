/// Method for drawing font data received from [crate::Font::to_bytes] to an RGBA buffer
pub trait DrawFont {
    fn draw_font(
        &mut self,
        x: usize,
        y: usize,
        font_width: usize,
        total_width: usize,
        font_bytes: &[u8],
    );
}

impl DrawFont for Vec<u8> {
    fn draw_font(
        &mut self,
        x: usize,
        y: usize,
        font_width: usize,
        total_width: usize,
        font_bytes: &[u8],
    ) {
        let mut line_start = (y * total_width + x) * 4;
        for font_bytes in font_bytes.chunks_exact(font_width * 4) {
            match self.get_mut(line_start..line_start + font_width * 4) {
                Some(bytes) => bytes.copy_from_slice(font_bytes),
                None => break,
            }
            line_start += total_width * 4;
        }
    }
}
