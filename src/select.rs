use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::interaction::{Event, PromptInteraction, State},
    theme::THEME,
};

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
}

impl<T> Select<T>
where
    T: Clone + Eq,
{
    /// Creates a new selection prompt.
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: THEME.lock().unwrap()
                .format_multiline_text(&prompt.to_string()),
            items: Vec::new(),
            cursor: 0,
            initial_value: None,
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
            Key::Enter => return State::Submit(self.items[self.cursor].value.clone()),
            _ => {}
        }

        State::Active
    }

    fn render(&mut self, state: &State<T>) -> String {
        let theme = THEME.lock().unwrap();

        let line1 = theme.format_header(&state.into(), &self.prompt);

        let line2: String = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                theme.format_select_item(&state.into(), self.cursor == i, &item.label, &item.hint)
            })
            .collect();
        let line3 = theme.format_footer(&state.into());

        line1 + &line2 + &line3
    }
}
