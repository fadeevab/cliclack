use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::interaction::{Event, PromptInteraction, State},
    theme::{ClackTheme, Theme},
};

#[derive(Default)]
pub struct SelectItem<T: Default> {
    value: T,
    label: String,
    hint: String,
}

#[derive(Default)]
pub struct Select<T: Default> {
    prompt: String,
    items: Vec<SelectItem<T>>,
    cursor: usize,
}

impl<T: Default + Clone> Select<T> {
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            ..Default::default()
        }
    }

    pub fn item(mut self, value: T, label: impl Display, hint: impl Display) -> Self {
        self.items.push(SelectItem {
            value,
            label: label.to_string(),
            hint: hint.to_string(),
        });
        self
    }

    pub fn interact(&mut self) -> io::Result<T> {
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
