pub(crate) struct TermSize {
    window_max_rows: usize,
    window_pos: usize,
}

impl Default for TermSize {
    fn default() -> Self {
        let mut window_max_rows = usize::MAX;

        if let Some(termsize) = termsize::get() {
            window_max_rows = (termsize.rows as usize)
                .checked_sub(3)
                .unwrap_or(termsize.rows as usize);
        }

        Self {
            window_max_rows,
            window_pos: 0,
        }
    }
}

impl TermSize {
    pub fn get_max_rows(&self) -> usize {
        self.window_max_rows
    }

    pub fn set_max_rows(&mut self, rows: usize) {
        self.window_max_rows = rows;
    }

    pub fn get_pos(&self) -> usize {
        self.window_pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.window_pos = pos;
    }
}
