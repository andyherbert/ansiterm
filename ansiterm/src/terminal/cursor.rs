use super::Blink;

pub struct Cursor {
    pub column: usize,
    pub row: usize,
    pub height: usize,
    pub blink: Blink,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            column: 0,
            row: 0,
            height: 2,
            blink: Blink::new(7),
        }
    }
}

impl Cursor {
    pub fn new(height: usize) -> Self {
        Self {
            height,
            ..Default::default()
        }
    }

    pub fn reset_blink(&mut self) {
        self.blink.reset();
    }
}

impl Clone for Cursor {
    fn clone(&self) -> Self {
        Self {
            column: self.column,
            row: self.row,
            ..Default::default()
        }
    }
}
