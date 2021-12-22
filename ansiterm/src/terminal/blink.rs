pub struct Blink {
    state: bool,
    count: usize,
    delay: usize,
}

impl Blink {
    pub fn new(delay: usize) -> Self {
        Self {
            state: true,
            count: delay,
            delay,
        }
    }

    pub fn tic(&mut self) -> bool {
        if self.count - 1 == 0 {
            self.count = self.delay;
            self.state = !self.state;
        } else {
            self.count -= 1;
        }
        self.state
    }

    pub fn reset(&mut self) {
        self.count = self.delay;
        self.state = true;
    }
}
