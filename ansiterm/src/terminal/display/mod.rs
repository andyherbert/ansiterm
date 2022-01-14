mod buffer;
mod get_and_put_pixels;
use super::{Blink, Cursor};
use buffer::Buffer;
use codepage::Font;
use ega_palette::{EgaPalette, Rgba};
use get_and_put_pixels::GetAndPutRgba;

pub enum Colour {
    Indexed(usize),
    Rgba(Rgba),
}

pub struct TerminalDisplay {
    font: Font,
    pub width: usize,
    pub height: usize,
    blink_on: Buffer,
    blink_off: Buffer,
    blink: Blink,
    palette: EgaPalette,
}

impl TerminalDisplay {
    pub fn new(columns: usize, rows: usize, palette: EgaPalette) -> Self {
        let font = Font::default();
        let width = columns * font.width;
        let height = rows * font.height;
        Self {
            font,
            width,
            height,
            blink_on: Buffer::new(width, height),
            blink_off: Buffer::new(width, height),
            blink: Blink::new(12),
            palette,
        }
    }

    pub fn draw_cursor(&mut self, frame: &mut [u8], cursor: &Cursor) {
        let x = cursor.column * self.font.width;
        let y = cursor.row * self.font.height + (self.font.height - cursor.height);
        frame.get_rgba(x, y, self.font.width, cursor.height, self.width, |pixel| {
            pixel.copy_from_slice(&[255 - pixel[0], 255 - pixel[1], 255 - pixel[2], 255]);
        });
    }

    pub fn next_frame(&mut self, frame: &mut [u8]) {
        if self.blink.tic() {
            frame.copy_from_slice(self.blink_on.frame.as_slice());
        } else {
            frame.copy_from_slice(self.blink_off.frame.as_slice());
        }
    }

    pub fn scroll_up(&mut self) {
        self.blink_on.scroll_up(self.font.height);
        self.blink_off.scroll_up(self.font.height);
    }

    pub fn clear(&mut self) {
        self.blink_on.clear();
        self.blink_off.clear();
    }

    pub fn clear_at(&mut self, column: usize, row: usize) {
        let x = column * self.font.width;
        let y = row * self.font.height;
        self.blink_on
            .clear_rect(x, y, self.font.width, self.font.height);
        self.blink_off
            .clear_rect(x, y, self.font.width, self.font.height);
    }

    pub fn draw_glyph(
        &mut self,
        byte: u8,
        column: usize,
        row: usize,
        fg: Colour,
        bg: Colour,
        blink: bool,
    ) {
        let x = column * self.font.width;
        let y = row * self.font.height;
        let fg = match fg {
            Colour::Indexed(index) => self.palette[index].rgba,
            Colour::Rgba(rgba) => rgba,
        };
        let bg = match bg {
            Colour::Indexed(index) => self.palette[index].rgba,
            Colour::Rgba(rgba) => rgba,
        };
        let rgba = self.font.to_vec(byte, &fg, &bg);
        self.blink_on
            .frame
            .put_rgba(x, y, self.font.width, self.font.height, self.width, &rgba);
        if blink {
            self.blink_off
                .fill_rect(x, y, self.font.width, self.font.height, &bg)
        } else {
            self.blink_off.frame.put_rgba(
                x,
                y,
                self.font.width,
                self.font.height,
                self.width,
                &rgba,
            );
        }
    }
}
