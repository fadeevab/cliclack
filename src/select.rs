use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::interaction::{Event, PromptInteraction, State}
    ,
    theme::THEME,
};
use crate::prompt::cursor::StringCursor;

pub struct RadioButton<T> {
    pub value: T,
    pub label: String,
    pub hint: String,
}

/// A prompt that asks for one selection from a list of options.
#[derive(Default)]
pub struct Select<T> {
    prompt: String,
    items: Vec<RadioButton<T>>,
    cursor: usize,
    initial_value: Option<T>,
    enable_filter: bool,
    filter: StringCursor,
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
            enable_filter: false,
            filter: StringCursor::default(),
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
}

impl<T: Clone> PromptInteraction<T> for Select<T> {
    fn on(&mut self, event: &Event) -> State<T> {
        let Event::Key(key) = event;

        match key {
            Key::ArrowLeft | Key::ArrowUp => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            Key::ArrowRight | Key::ArrowDown => {
                if self.cursor < self.items.len() - 1 {
                    self.cursor += 1;
                }
            }

            Key::Char(char) => {
                if *char == '/' {
                    self.filter.delete_word_to_the_left();
                    self.enable_filter = !self.enable_filter;
                }
            }
            Key::Enter => return State::Submit(self.items[self.cursor].value.clone()),

            _ => {}
        }

        State::Active
    }


    fn input(&mut self) -> Option<&mut StringCursor> {
        Some(&mut self.filter)
    }

    fn render(&mut self, state: &State<T>) -> String {
        let theme = THEME.lock().unwrap();

        let line1 = theme.format_header(&state.into(), &self.prompt);
        let line4 = theme.format_footer(&state.into());
        let line2: String;
        if self.enable_filter {
            line2 = self
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    theme.format_select_item(&state.into(), self.cursor == i, &item.label, &item.hint)
                })
                .filter(|item| {
                    item.contains(&self.filter.to_string())
                })
                .collect();

            if !self.filter.is_empty() {
                if let Some(index) = self.items.iter().position(|item| item.label.contains(&self.filter.to_string())) {
                    self.cursor = index;
                }
            }
            let line3 = theme.format_input(&state.into(), &self.filter);
            line1 + &line2 + &line3 + &line4
        } else {
            line2 = self
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    theme.format_select_item(&state.into(), self.cursor == i, &item.label, &item.hint)
                })
                .collect();
            line1 + &line2 + &line4
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
