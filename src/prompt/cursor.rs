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

/// Returns the indices of the start of each line in the given string.
fn line_jump_indices(value: &[char]) -> Vec<usize> {
    value.split(|c| *c == '\n').fold(vec![0], |mut acc, line| {
        acc.push(acc.last().unwrap() + line.len() + 1);
        acc
    })
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

    pub fn move_up(&mut self) {
        let jumps = line_jump_indices(&self.value);
        self.cursor = match jumps.binary_search(&self.cursor) {
            Ok(ix) if ix + 1 < jumps.len() => {
                // happened to be at the start of a line
                let target_line = ix.saturating_sub(1);
                jumps[target_line]
            }
            Ok(ix) | Err(ix) => {
                let ix = ix.saturating_sub(1); // current line
                let target_line = ix.saturating_sub(1);
                let offset = std::cmp::min(
                    self.cursor - jumps[ix],
                    (jumps[ix] - jumps[target_line]).saturating_sub(1),
                );
                jumps[target_line] + offset
            }
        }
    }

    pub fn move_down(&mut self) {
        let jumps = line_jump_indices(&self.value);
        self.cursor = match jumps.binary_search(&self.cursor) {
            Ok(ix) if ix + 1 < jumps.len() => {
                // happened to be at the start of a line
                let target_line = std::cmp::min(ix + 1, jumps.len().saturating_sub(2));
                jumps[target_line]
            }
            Ok(ix) => {
                // happened to be at the end of string
                jumps[ix].saturating_sub(1)
            }
            Err(ix) => {
                let ix = ix.saturating_sub(1); // current line
                let target_line = std::cmp::min(ix + 1, jumps.len().saturating_sub(2));
                let target_next = std::cmp::min(target_line + 1, jumps.len().saturating_sub(1));
                let offset = std::cmp::min(
                    self.cursor - jumps[ix],
                    (jumps[target_next] - jumps[target_line]).saturating_sub(1),
                );
                jumps[target_line] + offset
            }
        }
    }

    pub fn move_left_by_word(&mut self) {
        let jumps = word_jump_indices(&self.value);
        let ix = jumps.binary_search(&self.cursor).unwrap_or_else(|i| i);
        self.cursor = jumps[ix.saturating_sub(1)];
    }

    pub fn move_right_by_word(&mut self) {
        let jumps = word_jump_indices(&self.value);
        let ix = jumps
            .binary_search(&self.cursor)
            .map_or_else(|i| i, |i| i + 1);
        self.cursor = jumps[std::cmp::min(ix, jumps.len().saturating_sub(1))];
    }

    pub fn move_home(&mut self) {
        let jumps = line_jump_indices(&self.value);
        self.cursor = match jumps.binary_search(&self.cursor) {
            Ok(ix) if ix + 1 < jumps.len() => self.cursor, // happened to be at the start of a line
            Ok(ix) | Err(ix) => jumps[ix.saturating_sub(1)],
        }
    }

    pub fn move_end(&mut self) {
        let jumps = line_jump_indices(&self.value);
        self.cursor = match jumps.binary_search(&self.cursor) {
            Ok(ix) if ix + 1 < jumps.len() => jumps[ix + 1].saturating_sub(1), // happened to be at the start of a line
            Ok(ix) | Err(ix) => jumps[ix].saturating_sub(1),
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

    pub fn delete_word_to_the_left(&mut self) {
        if self.cursor > 0 {
            let jumps = word_jump_indices(&self.value);
            let ix = jumps.binary_search(&self.cursor).unwrap_or_else(|x| x);
            let start = jumps[std::cmp::max(ix - 1, 0)];
            let end = self.cursor;
            self.value.drain(start..end);
            self.cursor = start;
        }
    }

    pub fn clear(&mut self) {
        self.cursor = 0;
        self.value.clear()
    }

    pub fn extend(&mut self, string: &str) {
        self.value.extend(string.chars());
    }

    pub fn split(&self) -> (String, String, String) {
        let left = String::from_iter(&self.value[..self.cursor]);
        let mut cursor = String::from(' ');
        let mut right = String::new();

        match self.current() {
            Some('\n') => right.push('\n'),
            Some(chr) => cursor = chr.to_string(),
            None => {}
        };

        if !self.value.is_empty() && self.cursor < self.value.len() - 1 {
            right.push_str(&String::from_iter(&self.value[self.cursor + 1..]));
        }

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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_cursor {
        ($cursor: expr, $char: expr) => {
            assert_eq!($cursor.current().unwrap_or(' '), $char);
        };
    }

    macro_rules! assert_content {
        ($cursor: expr, $content: expr) => {
            assert_eq!($cursor.value, $content.chars().collect::<Vec<_>>());
        };
    }

    #[test]
    fn test_string_cursor() {
        let mut cursor = StringCursor {
            value: "hello\nworld".chars().collect(),
            cursor: 0,
        };
        assert_cursor!(cursor, 'h');
        assert_content!(cursor, "hello\nworld");
        cursor.move_right();
        assert_cursor!(cursor, 'e');
        cursor.move_up();
        assert_cursor!(cursor, 'h');
        cursor.move_up();
        assert_cursor!(cursor, 'h');
        cursor.move_down();
        assert_cursor!(cursor, 'w');
        cursor.move_down();
        assert_cursor!(cursor, 'w');
        cursor.move_end();
        assert_cursor!(cursor, ' ');
        cursor.move_up();
        assert_cursor!(cursor, '\n');
        for c in "\nbeautiful".chars() {
            cursor.insert(c);
        }
        assert_content!(cursor, "hello\nbeautiful\nworld");
        cursor.move_up();
        assert_cursor!(cursor, '\n');
        cursor.move_down();
        assert_cursor!(cursor, 'i');
        cursor.move_end();
        assert_cursor!(cursor, '\n');
        cursor.move_end();
        assert_cursor!(cursor, '\n');
        cursor.move_down();
        cursor.move_left();
        assert_cursor!(cursor, 'd');
        cursor.move_home();
        assert_cursor!(cursor, 'w');
    }
}
