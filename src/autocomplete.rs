use console::Key;

use crate::{prompt::interaction::State, theme::THEME, Suggest};

pub(crate) struct Autocomplete {
    /// The list of suggestions to be rendered.
    source: Box<dyn Suggest<Result = String>>,
    /// The list of suggestions to be rendered.
    items: Vec<String>,
    /// The index of the currently selected suggestion (unselected by default).
    cursor: Option<usize>,
}

impl Autocomplete {
    /// Creates a new autocompletion popup with the given suggestions.
    pub fn new<S>(suggestions: S) -> Self
    where
        S: Suggest<Result = String> + 'static,
    {
        Self {
            source: Box::new(suggestions),
            items: Vec::new(),
            cursor: None,
        }
    }

    /// Tracks the state of the autocompletion popup.
    pub fn on(&mut self, key: &Key, query: &str) -> Option<State<String>> {
        if self.items.is_empty() {
            self.items = self.source.suggest(query);
            return None;
        }

        let len = self.items.len();

        // Temporarily cap the cursor, in case the suggestions list has shrunk.
        // It allows to keep the original cursor position unless arrows are pressed.
        let cursor = self.cursor.unwrap_or(0).min(len - 1);

        match key {
            // If the cursor is not set, it will be set on the first arrow key press.
            Key::ArrowUp | Key::ArrowDown if self.cursor.is_none() => {
                self.cursor = Some(cursor);
                return None;
            }
            // Move the cursor up in a circular manner.
            Key::ArrowUp => self.cursor = Some(cursor.saturating_sub(1) % len),
            // Move the cursor down in a circular manner.
            Key::ArrowDown => self.cursor = Some((cursor + 1) % len),
            // Submit the currently highlighted suggestion if cursor is set.
            Key::Tab | Key::Enter => {
                return self
                    .cursor
                    .map(|_| State::Submit(self.items[cursor].clone()))
            }
            // Hide the autocompletion drop-down, and tell the prompt to not close the entire app.
            Key::Escape => {
                self.items.clear();
                self.cursor = None;
                return Some(State::Cancel);
            }
            // Other keys refresh the suggestions, capping the cursor.
            _ => {
                self.items = self.source.suggest(query);
            }
        }
        None
    }

    /// Renders autocomplete popup suggestions under the input line.
    pub fn render<T>(&self, state: &State<T>) -> String {
        match state {
            State::Submit(_) | State::Cancel => return String::new(),
            _ => {}
        }

        if self.items.is_empty() {
            return String::new();
        }

        // Temporarily cap the cursor, in case the suggestions list has shrunk.
        // It allows to keep the original cursor position unless arrows are pressed.
        let cursor = self.cursor.map(|line| line.min(self.items.len() - 1));

        let theme = THEME.read().unwrap();

        let empty_line = [/*theme.format_autocomplete_item(&state.into(), false, "")*/].into_iter();
        let items = self.items.iter().enumerate().map(|(i, item)| {
            theme.format_autocomplete_item(&state.into(), cursor == Some(i), item)
        });

        empty_line.chain(items).collect()
    }
}
