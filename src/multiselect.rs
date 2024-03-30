use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::interaction::{Event, PromptInteraction, State},
    theme::THEME,
};

pub struct Checkbox<T> {
    pub value: T,
    pub label: String,
    pub hint: String,
    pub selected: bool,
}

/// A prompt that asks for one or more selections from a list of options.
pub struct MultiSelect<T> {
    prompt: String,
    items: Vec<Checkbox<T>>,
    cursor: usize,
    initial_values: Option<Vec<T>>,
    required: bool,
}

impl<T> MultiSelect<T>
where
    T: Clone + Eq,
{
    /// Creates a new [`MultiSelect`] prompt.
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            items: vec![],
            cursor: 0,
            initial_values: None,
            required: true,
        }
    }

    /// Adds an item to the list of options.
    pub fn item(mut self, value: T, label: impl Display, hint: impl Display) -> Self {
        self.items.push(Checkbox {
            value,
            label: label.to_string(),
            hint: hint.to_string(),
            selected: false,
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

    /// Sets the initially selected values.
    pub fn initial_values(mut self, value: Vec<T>) -> Self {
        self.initial_values = Some(value);
        self
    }

    /// Sets whether the input is required. Default: `true` (at least
    /// 1 selected item).
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Starts the prompt interaction.
    pub fn interact(&mut self) -> io::Result<Vec<T>> {
        if let Some(initial_values) = &self.initial_values {
            for item in self.items.iter_mut() {
                if initial_values.contains(&item.value) {
                    item.selected = true;
                }
            }
        }
        <Self as PromptInteraction<Vec<T>>>::interact(self)
    }
}

impl<T: Clone> PromptInteraction<Vec<T>> for MultiSelect<T> {
    fn on(&mut self, event: &Event) -> State<Vec<T>> {
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
            Key::Char(' ') => {
                self.items[self.cursor].selected = !self.items[self.cursor].selected;
            }
            Key::Enter => {
                let selected_items = self
                    .items
                    .iter()
                    .filter(|item| item.selected)
                    .map(|item| item.value.clone())
                    .collect::<Vec<_>>();

                if selected_items.is_empty() && self.required {
                    return State::Error("Input required".to_string());
                }

                return State::Submit(selected_items);
            }
            _ => {}
        }

        State::Active
    }

    fn render(&mut self, state: &State<Vec<T>>) -> String {
        let theme = THEME.lock().unwrap();

        let line1 = theme.format_header(&state.into(), &self.prompt);

        let mut line2 = String::new();
        for (i, item) in self.items.iter().enumerate() {
            line2.push_str(&theme.format_multiselect_item(
                &state.into(),
                item.selected,
                i == self.cursor,
                &item.label,
                &item.hint,
            ));
        }
        let line3 = theme.format_footer(&state.into());

        line1 + &line2 + &line3
    }
}
