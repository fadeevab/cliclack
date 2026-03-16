use console::Key;

use crate::{prompt::interaction::State, theme::THEME, Suggest};

pub(crate) struct Autocomplete {
    /// The list of suggestions to be rendered.
    source: Box<dyn Suggest<Result = String>>,
    /// The list of suggestions to be rendered.
    items: Vec<String>,
    /// The index of the currently selected suggestion.
    cursor: usize,
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
            cursor: 0,
        }
    }

    /// Returns the list of suggestions for the given input.
    pub fn items(&self) -> &Vec<String> {
        &self.items
    }

    /// Returns the currently highlighted suggestion.
    pub fn highlighted(&self) -> Option<&String> {
        self.items.get(self.cursor)
    }

    /// Tracks the state of the autocompletion popup.
    pub fn on(&mut self, key: &Key, query: &str) -> Option<String> {
        if self.items.is_empty() {
            self.items = self.source.suggest(query);
            return None;
        }

        // Temporarily cap the cursor, in case the suggestions list has shrunk.
        // It allows to keep the original cursor position unless arrows are pressed.
        let cursor = self.cursor.min(self.items.len().saturating_sub(1));

        match key {
            // Move the cursor up in a circular manner.
            Key::ArrowUp => self.cursor = (cursor.saturating_sub(1)) % self.items.len(),
            // Move the cursor down in a circular manner.
            Key::ArrowDown => self.cursor = (cursor + 1) % self.items.len(),
            // Submit the currently highlighted suggestion.
            Key::Tab => return Some(self.items[cursor].clone()),
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
        let cursor = self.cursor.min(self.items.len().saturating_sub(1));

        let theme = THEME.read().unwrap();

        let empty_line = [theme.format_autocomplete_item(&state.into(), false, "")].into_iter();
        let items = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| theme.format_autocomplete_item(&state.into(), cursor == i, item));

        empty_line.chain(items).collect()
    }
}
