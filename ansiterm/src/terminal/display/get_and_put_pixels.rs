use ansiart::ega_palette::Rgba;

pub trait GetAndPutRgba {
    fn put_rgba(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        total_width: usize,
        rgba: &Rgba,
    );
    fn put_inverse(&mut self, x: usize, y: usize, width: usize, height: usize, total_width: usize);
    fn fill_with_rgba(&mut self, rgba: &Rgba);
}

impl GetAndPutRgba for [u8] {
    fn put_rgba(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        total_width: usize,
        rgba: &Rgba,
    ) {
        let start = (y * total_width + x) * 4;
        let end = ((y + height) * total_width + x) * 4;
        for line_start in (start..end).step_by(total_width * 4) {
            if let Some(chunk) = self.get_mut(line_start..width * 4 + line_start) {
                for chunk in chunk.chunks_exact_mut(4) {
                    chunk.copy_from_slice(rgba);
                }
            }
        }
    }

    fn put_inverse(&mut self, x: usize, y: usize, width: usize, height: usize, total_width: usize) {
        let start = (y * total_width + x) * 4;
        let end = ((y + height) * total_width + x) * 4;
        for line_start in (start..end).step_by(total_width * 4) {
            if let Some(chunk) = self.get_mut(line_start..width * 4 + line_start) {
                for chunk in chunk.chunks_exact_mut(4) {
                    let rgba = [255 - chunk[0], 255 - chunk[1], 255 - chunk[2], 255];
                    chunk.copy_from_slice(&rgba);
                }
            }
        }
    }

    fn fill_with_rgba(&mut self, src: &Rgba) {
        for dest in self.chunks_exact_mut(4) {
            dest.copy_from_slice(src);
        }
    }
}
