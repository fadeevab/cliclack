use std::fmt::{Display, Formatter, Result};

use zeroize::ZeroizeOnDrop;

#[derive(Default, ZeroizeOnDrop, Clone)]
pub struct StringCursor {
    value: Vec<char>,
    cursor: usize,
}

/// Returns the indices of the first character of each word in the given string,
/// as well as the indices of the start and end of the string. The returned
/// indices are sorted in ascending order.
fn word_jump_indices(value: &[char]) -> Vec<usize> {
    let mut indices = vec![0];
    let mut in_word = false;

    for (i, ch) in value.iter().enumerate() {
        if ch.is_whitespace() {
            in_word = false;
        } else if !in_word {
            indices.push(i);
            in_word = true;
        }
    }

    indices.push(value.len());

    indices
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

    pub fn move_word_left(&mut self) {
        if self.cursor > 0 {
            let jumps = word_jump_indices(&self.value);
            let ix = jumps.binary_search(&self.cursor).unwrap_or_else(|i| i);
            self.cursor = jumps[std::cmp::max(ix - 1, 0)];
        }
    }

    pub fn move_word_right(&mut self) {
        if self.cursor < self.value.len() {
            let jumps = word_jump_indices(&self.value);
            let ix = jumps
                .binary_search(&self.cursor)
                .map_or_else(|i| i, |i| i + 1);
            self.cursor = jumps[std::cmp::min(ix, jumps.len() - 1)];
        }
    }

    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor = self.value.len();
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

    pub fn delete_word_left(&mut self) {
        if self.cursor > 0 {
            let jumps = word_jump_indices(&self.value);
            let ix = jumps.binary_search(&self.cursor).unwrap_or_else(|x| x);
            let start = jumps[std::cmp::max(ix - 1, 0)];
            let end = self.cursor;
            self.value.drain(start..end);
            self.cursor = start;
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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut char> {
        self.value.iter_mut()
    }
}

impl Display for StringCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", String::from_iter(&self.value))
    }
}
