use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::interaction::{Event, PromptInteraction, State},
    theme::{ClackTheme, Theme},
};

#[derive(Default)]
pub struct RadioButton<T: Default> {
    pub value: T,
    pub label: String,
    pub hint: String,
}

/// A prompt that asks for one selection from a list of options.
#[derive(Default)]
pub struct Select<T: Default> {
    prompt: String,
    items: Vec<RadioButton<T>>,
    cursor: usize,
    initial_value: Option<T>,
}

impl<T> Select<T>
where
    T: Default + Clone + Eq,
{
    /// Creates a new selection prompt.
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            ..Default::default()
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

    /// Sets the initially selected item by value.
    pub fn initial_value(mut self, value: T) -> Self {
        self.initial_value = Some(value);
        self
    }

    /// Starts the prompt interaction.
    pub fn interact(&mut self) -> io::Result<T> {
        for (i, item) in self.items.iter().enumerate() {
            if let Some(initial_value) = &self.initial_value {
                if initial_value == &item.value {
                    self.cursor = i;
                    break;
                }
            }
        }
        <Self as PromptInteraction<T>>::interact(self)
    }
}

impl<T: Default + Clone> PromptInteraction<T> for Select<T> {
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
        let line1 = ClackTheme.format_header(&state.into(), &self.prompt);

        let mut line2 = String::new();
        for (i, item) in self.items.iter().enumerate() {
            line2.push_str(&ClackTheme.format_select_item(
                &state.into(),
                self.cursor == i,
                &item.label,
                &item.hint,
            ));
        }
        let line3 = ClackTheme.format_footer(&state.into());

        line1 + &line2 + &line3
    }
}
