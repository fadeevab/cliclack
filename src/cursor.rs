use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub struct StringCursor {
    value: Vec<char>,
    cursor: usize,
}

impl StringCursor {
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn current(&self) -> Option<char> {
        self.value.get(self.cursor).copied()
    }

    pub fn insert(&mut self, chr: char) {
        self.value.insert(self.cursor, chr);
        self.cursor += 1;
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor < self.value.len() {
            self.cursor += 1;
        }
    }

    pub fn delete_left(&mut self) {
        if self.value.is_empty() {
            return;
        }

        if self.cursor > 0 {
            self.value.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    pub fn delete_right(&mut self) {
        if self.value.is_empty() {
            return;
        }

        if self.cursor < self.value.len() {
            self.value.remove(self.cursor);
        }
    }

    pub fn extend(&mut self, string: &str) {
        self.value.extend(string.chars());
    }

    pub fn split(&self) -> (String, String, String) {
        let left = String::from_iter(&self.value[..self.cursor]);

        let cursor = String::from_iter(&[self.current().unwrap_or(' ')]);

        let right = if !self.value.is_empty() && self.cursor < self.value.len() - 1 {
            String::from_iter(&self.value[self.cursor + 1..])
        } else {
            String::new()
        };

        (left, cursor, right)
    }
}

impl Display for StringCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", String::from_iter(&self.value))
    }
}
