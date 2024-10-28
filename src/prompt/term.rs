pub(crate) struct TermSize {
    window_size: usize,
    window_pos: usize,
}

impl Default for TermSize {
    fn default() -> Self {
        let mut window_size = usize::MAX;

        if let Some(termsize) = termsize::get() {
            window_size = termsize.rows as usize - 3;
        }

        Self {
            window_size,
            window_pos: 0,
        }
    }
}

impl TermSize {
    pub fn get_size(&self) -> usize {
        self.window_size
    }

    pub fn set_size(&mut self, size: usize) {
        self.window_size = size;
    }

    pub fn get_pos(&self) -> usize {
        self.window_pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.window_pos = pos;
    }
}
