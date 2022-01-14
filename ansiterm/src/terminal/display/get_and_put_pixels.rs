use super::super::Rgba;

pub trait GetAndPutRgba {
    fn get_rgba<F: FnMut(&mut [u8])>(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        total_width: usize,
        each_pixel: F,
    );

    fn put_rgba(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        total_width: usize,
        rgba: &[Rgba],
    );

    fn fill_with_rgba(&mut self, rgba: &Rgba);
}

impl GetAndPutRgba for [u8] {
    fn get_rgba<F: FnMut(&mut [u8])>(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        total_width: usize,
        mut each_pixel: F,
    ) {
        let start = (y * total_width + x) * 4;
        let end = ((y + height) * total_width + x) * 4;
        for line_start in (start..end).step_by(total_width * 4) {
            for chunk in self[line_start..width * 4 + line_start].chunks_exact_mut(4) {
                each_pixel(chunk);
            }
        }
    }

    fn put_rgba(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        total_width: usize,
        rgba: &[Rgba],
    ) {
        let mut iter = rgba.iter();
        self.get_rgba(x, y, width, height, total_width, |dest| {
            if let Some(src) = iter.next() {
                dest.copy_from_slice(&src.to_owned());
            }
        });
    }

    fn fill_with_rgba(&mut self, src: &Rgba) {
        for dest in self.chunks_exact_mut(4) {
            dest.copy_from_slice(src);
        }
    }
}
