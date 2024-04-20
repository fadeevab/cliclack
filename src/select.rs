use std::fmt::Display;
use std::io;

use console::Key;
use regex::Regex;

use crate::{
    prompt::cursor::StringCursor,
    prompt::interaction::{Event, PromptInteraction, State},
    theme::THEME,
};

#[derive(Clone)]
pub struct RadioButton<T> {
    pub value: T,
    pub label: String,
    pub hint: String,
}

/// A prompt that asks for one selection from a list of options.
pub struct Select<T> {
    prompt: String,
    items: Vec<RadioButton<T>>,
    cursor: usize,
    initial_value: Option<T>,
    enable_filter_mode: bool,
    switch_filter_mode: bool,
    filter: StringCursor,
    filtered_items: Vec<RadioButton<T>>,
}

impl<T> Select<T>
    where
        T: Clone + Eq,
{
    /// Creates a new selection prompt.
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            items: Vec::new(),
            cursor: 0,
            initial_value: None,
            enable_filter_mode: false,
            switch_filter_mode: false,
            filter: StringCursor::default(),
            filtered_items: Vec::new(),
        }
    }

    /// Adds an item to the selection prompt.
    pub fn item(mut self, value: T, label: impl Display, hint: impl Display) -> Self {
        self.items.push(RadioButton {
            value,
            label: label.to_string(),
            hint: hint.to_string(),
        });
        self
    }

    /// Adds multiple items to the list of options.
    pub fn items(mut self, items: &[(T, impl Display, impl Display)]) -> Self {
        for (value, label, hint) in items {
            self = self.item(value.clone(), label, hint);
        }
        self
    }

    /// Sets the initially selected item by value.
    pub fn initial_value(mut self, value: T) -> Self {
        self.initial_value = Some(value);
        self
    }

    /// Starts the prompt interaction.
    pub fn interact(&mut self) -> io::Result<T> {
        if self.items.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No items added to the list",
            ));
        }
        if let Some(initial_value) = &self.initial_value {
            self.cursor = self
                .items
                .iter()
                .position(|item| item.value == *initial_value)
                .unwrap_or(self.cursor);
        }
        <Self as PromptInteraction<T>>::interact(self)
    }

    /// Enable the filter mode, press / to activate
    pub fn enable_filter(mut self) -> Self {
        self.enable_filter_mode = true;
        self
    }
}

impl<T: Clone> PromptInteraction<T> for Select<T> {
    fn on(&mut self, event: &Event) -> State<T> {
        let Event::Key(key) = event;

        match key {
            Key::ArrowUp => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            Key::ArrowDown => {
                if self.cursor < self.items.len() - 1 {
                    self.cursor += 1;
                }
            }
            Key::ArrowRight => {
                if !self.switch_filter_mode && self.cursor < self.items.len() - 1 {
                    self.cursor += 1;
                }
            }
            Key::ArrowLeft => {
                if !self.switch_filter_mode && self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            Key::Char(char) => {
                if self.enable_filter_mode && *char == '/' {
                    self.filter.clear();
                    self.switch_filter_mode = !self.switch_filter_mode;
                    if self.switch_filter_mode { self.filtered_items = self.items.clone() }
                }
            }
            Key::Enter => {
                return if self.switch_filter_mode {
                    State::Submit(self.filtered_items.get(self.cursor).unwrap_or(&self.items[0]).value.clone())
                } else {
                    State::Submit(self.items[self.cursor].value.clone())
                };
            }
            _ => {}
        }

        State::Active
    }

    fn render(&mut self, state: &State<T>) -> String {
        let theme = THEME.lock().unwrap();

        let header_display = theme.format_header(&state.into(), &self.prompt);
        let items_display: String;
        let footer_display = theme.format_footer(&state.into());

        if self.switch_filter_mode {
            let filter_regex = Regex::new(&format!("(?i){}", self.filter))
                .unwrap_or_else(|_| Regex::new(r"^\b$").unwrap());

            self.filtered_items = self
                .items
                .iter()
                .filter(|item| filter_regex.is_match(&item.label))
                .cloned()
                .collect();

            if !self.filtered_items.is_empty() && self.cursor > self.filtered_items.len() - 1 { self.cursor = 0 }

            let filter_display = theme.format_input(&state.into(), &self.filter);

            items_display = self
                .filtered_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    theme.format_select_item(&state.into(), self.cursor == i, &item.label, &item.hint)
                })
                .collect();

            header_display + &filter_display + &items_display + &footer_display
        } else {
            items_display = self
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    theme.format_select_item(&state.into(), self.cursor == i, &item.label, &item.hint)
                })
                .collect();
            header_display + &items_display + &footer_display
        }
    }

    fn input(&mut self) -> Option<&mut StringCursor> {
        Some(&mut self.filter)
    }
}

#[cfg(test)]
mod test {
    use crate::Select;

    #[test]
    fn empty_list() {
        let mut select = Select::<&str>::new("Select an item").initial_value("");
        let result = select.interact();
        assert_eq!(
            "No items added to the list",
            result.unwrap_err().to_string()
        );
    }
}
