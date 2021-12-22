use super::palette::BLACK;
use super::GetAndPutRgba;

pub struct Buffer {
    pub frame: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let frame = vec![0; width * height * 4];
        let mut buffer = Self {
            frame,
            width,
            height,
        };
        buffer.clear();
        buffer
    }

    pub fn clear(&mut self) {
        self.frame.fill_with_rgba(&BLACK);
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, src: &[u8; 4]) {
        self.frame
            .get_rgba(x, y, width, height, self.width, |dest| {
                dest.copy_from_slice(src);
            });
    }

    pub fn clear_rect(&mut self, x: usize, y: usize, width: usize, height: usize) {
        self.fill_rect(x, y, width, height, &BLACK);
    }

    pub fn scroll_up(&mut self, y: usize) {
        let start = y * self.width * 4;
        let buffer = self.frame[start..].to_vec();
        self.frame[0..buffer.len()].copy_from_slice(&buffer);
        self.clear_rect(0, self.height - y, self.width, y);
    }
}
