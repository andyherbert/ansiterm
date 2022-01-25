mod blink;
mod cursor;
mod display;
// mod palette;
use blink::Blink;
use cursor::Cursor;
use display::{Colour, TerminalDisplay};
use ega_palette::{EgaPalette, Rgba};

pub struct Terminal {
    display: TerminalDisplay,
    cursor: Cursor,
    stored_cursor: Option<Cursor>,
    columns: usize,
    rows: usize,
    wrap: bool,
    fg: usize,
    bg: usize,
    blink: bool,
    bold: bool,
    pablo_true_colour_bg: Option<Rgba>,
    pablo_true_colour_fg: Option<Rgba>,
    ice_colours: bool,
}

impl Terminal {
    pub fn new(columns: usize, rows: usize, ice_colours: bool) -> Self {
        Self {
            display: TerminalDisplay::new(columns, rows, EgaPalette::ansi()),
            cursor: Cursor::new(2),
            stored_cursor: None,
            columns,
            rows,
            wrap: false,
            fg: 7,
            bg: 0,
            blink: false,
            bold: false,
            pablo_true_colour_bg: None,
            pablo_true_colour_fg: None,
            ice_colours,
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.display.width as u32, self.display.height as u32)
    }

    pub fn select_graphics_rendition(&mut self, values: Vec<usize>) {
        for value in values {
            match value {
                0 => {
                    self.fg = 7;
                    self.bg = 0;
                    self.bold = false;
                    self.blink = false;
                    self.pablo_true_colour_bg = None;
                    self.pablo_true_colour_fg = None;
                }
                1 => {
                    self.bold = true;
                    self.pablo_true_colour_fg = None;
                }
                5 => self.blink = true,
                30..=37 => {
                    self.fg = value - 30;
                    self.pablo_true_colour_fg = None;
                }
                40..=47 => {
                    self.bg = value - 40;
                    self.pablo_true_colour_bg = None;
                }
                _ => {}
            }
        }
    }

    pub fn set_screen_mode(&mut self, value: usize) {
        if value == 7 {
            self.wrap = true;
        }
    }

    pub fn reset_screen_mode(&mut self, value: usize) {
        if value == 7 {
            self.wrap = false;
        }
    }

    pub fn move_cursor_to_column(&mut self, column: usize) {
        self.cursor.column = column.min(self.columns - 1);
        self.cursor.reset_blink();
    }

    pub fn move_cursor_to_row(&mut self, row: usize) {
        self.cursor.row = row.min(self.rows - 1);
        self.cursor.reset_blink();
    }

    pub fn move_cursor_to(&mut self, column: usize, row: usize) {
        self.cursor.column = column.min(self.columns - 1);
        self.cursor.row = row.min(self.rows - 1);
        self.cursor.reset_blink();
    }

    pub fn cursor_up(&mut self, amount: usize) {
        self.move_cursor_to_row((self.cursor.row as isize - amount as isize).max(0) as usize);
    }

    pub fn cursor_down(&mut self, amount: usize) {
        self.move_cursor_to_row(self.cursor.row + amount);
    }

    pub fn cursor_forward(&mut self, amount: usize) {
        self.move_cursor_to_column(self.cursor.column + amount);
    }

    pub fn cursor_back(&mut self, amount: usize) {
        self.move_cursor_to_column((self.cursor.column as isize - amount as isize).max(0) as usize);
    }

    pub fn tab(&mut self) {
        self.cursor_forward(8);
    }

    pub fn next_frame(&mut self, frame: &mut [u8]) {
        self.display.next_frame(frame);
        if self.cursor.blink.tic() && self.cursor.row < self.rows {
            self.display.draw_cursor(frame, &self.cursor)
        }
    }

    pub fn literal(&mut self, byte: u8) {
        if self.cursor.row == self.rows {
            self.display.scroll_up();
            self.cursor.row -= 1;
        }
        let fg = match self.pablo_true_colour_fg {
            Some(rgba) => Colour::Rgba(rgba),
            None if self.bold => Colour::Indexed(self.fg + 8),
            None => Colour::Indexed(self.fg),
        };
        let bg = match self.pablo_true_colour_bg {
            Some(rgba) => Colour::Rgba(rgba),
            None => Colour::Indexed(self.bg),
        };
        self.display.draw_glyph(
            byte,
            self.cursor.column,
            self.cursor.row,
            fg,
            bg,
            self.blink && !self.ice_colours,
        );
        if self.cursor.column == self.columns - 1 {
            self.cursor.column = 0;
            self.cursor.row += 1;
        } else {
            self.cursor.column += 1;
        }
    }

    pub fn line_feed(&mut self) {
        if self.cursor.row == self.rows {
            if self.wrap {
                self.cursor.row = 0;
            } else {
                self.display.scroll_up();
            }
        } else {
            self.cursor.row += 1;
        }
    }

    pub fn carriage_return(&mut self) {
        self.cursor.column = 0;
    }

    pub fn save_cursor_position(&mut self) {
        self.stored_cursor = Some(self.cursor.clone());
    }

    pub fn restore_cursor_position(&mut self) {
        if let Some(Cursor { column, row, .. }) = self.stored_cursor {
            self.move_cursor_to(column, row);
            self.stored_cursor = None;
        }
    }

    fn clear_to_end_of_line(&mut self) {
        if self.cursor.row < self.rows {
            for column in self.cursor.column..self.columns {
                self.display.clear_at(column, self.cursor.row);
            }
        }
    }

    fn clear_to_start_of_line(&mut self) {
        if self.cursor.row < self.rows {
            for column in 0..=self.cursor.column {
                self.display.clear_at(column, self.cursor.row);
            }
        }
    }

    fn clear_line(&mut self) {
        if self.cursor.row < self.rows {
            for column in 0..self.columns {
                self.display.clear_at(column, self.cursor.row);
            }
        }
    }

    pub fn erase_in_line(&mut self, value: usize) {
        match value {
            0 => self.clear_to_end_of_line(),
            1 => self.clear_to_start_of_line(),
            2 => self.clear_line(),
            _ => {}
        }
    }

    fn clear_to_end_of_display(&mut self) {
        self.clear_to_end_of_line();
        for row in self.cursor.row + 1..self.rows {
            for column in 0..self.columns {
                self.display.clear_at(column, row);
            }
        }
    }

    fn clear_to_start_of_display(&mut self) {
        self.clear_to_start_of_line();
        for row in 0..self.cursor.row {
            for column in 0..self.columns {
                self.display.clear_at(column, row);
            }
        }
    }

    fn clear_display(&mut self) {
        self.display.clear();
        self.move_cursor_to(0, 0);
    }

    pub fn erase_display(&mut self, value: usize) {
        match value {
            0 => self.clear_to_end_of_display(),
            1 => self.clear_to_start_of_display(),
            2 => self.clear_display(),
            _ => {}
        }
    }

    pub fn true_colour_bg(&mut self, red: u8, green: u8, blue: u8) {
        self.pablo_true_colour_bg = Some([red, green, blue, 255]);
    }

    pub fn true_colour_fg(&mut self, red: u8, green: u8, blue: u8) {
        self.pablo_true_colour_fg = Some([red, green, blue, 255]);
    }
}
