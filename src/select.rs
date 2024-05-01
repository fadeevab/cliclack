use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::cursor::StringCursor,
    prompt::interaction::{Event, PromptInteraction, State},
    theme::THEME,
};

#[derive(Clone)]
struct RadioButton<T> {
    value: T,
    label: String,
    hint: String,
}

/// A prompt that asks for one selection from a list of options.
pub struct Select<T> {
    prompt: String,
    items: Vec<RadioButton<T>>,
    cursor: usize,
    initial_value: Option<T>,
    filter_mode: Option<FilterMode<T>>,
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
            filter_mode: None,
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

    /// Enable the filter mode
    pub fn filter_mode(mut self) -> Self {
        self.filter_mode = Some(FilterMode::new(self.items.clone()));
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

        if let Some(filter) = &mut self.filter_mode {
            if let Some(state) = filter.on(key, &self.items) {
                return state;
            }
        }

        let (cursor, items) = if let Some(filter) = &mut self.filter_mode {
            (&mut filter.cursor, &filter.items)
        } else {
            (&mut self.cursor, &self.items)
        };

        match key {
            Key::ArrowUp | Key::ArrowLeft => {
                if *cursor > 0 {
                    *cursor -= 1;
                }
            }
            Key::ArrowDown | Key::ArrowRight => {
                if !items.is_empty() && *cursor < items.len() - 1 {
                    *cursor += 1;
                }
            }
            Key::Enter => {
                return State::Submit(items[*cursor].value.clone());
            }
            _ => {}
        }

        State::Active
    }

    fn render(&mut self, state: &State<T>) -> String {
        let theme = THEME.lock().unwrap();

        let header_display = theme.format_header(&state.into(), &self.prompt);
        let footer_display = theme.format_footer(&state.into());

        let filter_display = if let Some(filter) = &self.filter_mode {
            match state {
                State::Submit(_) | State::Cancel => "".to_string(),
                _ => theme.format_input(&state.into(), &filter.input),
            }
        } else {
            "".to_string()
        };

        let (items, cursor) = if let Some(filter) = &self.filter_mode {
            (&filter.items, filter.cursor)
        } else {
            (&self.items, self.cursor)
        };

        let items_display: String = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                theme.format_select_item(&state.into(), cursor == i, &item.label, &item.hint)
            })
            .collect();

        header_display + &filter_display + &items_display + &footer_display
    }

    /// Handles the input cursor automatically in the filter mode.
    fn input(&mut self) -> Option<&mut StringCursor> {
        self.filter_mode.as_mut().map(|filter| &mut filter.input)
    }
}

struct FilterMode<T> {
    input: StringCursor,
    items: Vec<RadioButton<T>>,
    cursor: usize,
}

impl<T: Clone> FilterMode<T> {
    fn new(items: Vec<RadioButton<T>>) -> Self {
        Self {
            input: StringCursor::default(),
            items,
            cursor: 0,
        }
    }

    fn on(&mut self, key: &Key, all_items: &[RadioButton<T>]) -> Option<State<T>> {
        match key {
            // Need further processing of simple up and down actions.
            Key::ArrowDown | Key::ArrowUp => None,
            // Need moving up and down the list if no input provided.
            Key::ArrowLeft | Key::ArrowRight if self.input.is_empty() => None,
            // Need submitting of the selected item.
            Key::Enter if !self.items.is_empty() => None,
            // Otherwise, no items found.
            Key::Enter => Some(State::Error("No items".into())),
            // Refresh the filtered items for the rest of the keys.
            _ if !self.input.is_empty() => {
                let input_lower = self.input.to_string();
                let filter_words: Vec<_> = input_lower.split_whitespace().collect();

                let mut filtered_and_scored_items: Vec<_> = all_items
                    .iter()
                    .map(|item| {
                        let similarity = strsim::jaro_winkler(
                            &item.label.to_lowercase(),
                            &self.input.to_string().to_lowercase(),
                        );
                        let bonus = filter_words
                            .iter()
                            .all(|word| item.label.to_lowercase().contains(&word.to_lowercase()))
                            as usize as f64;
                        (similarity + bonus, item)
                    })
                    .filter(|(similarity, _)| *similarity > 0.6)
                    .collect();

                filtered_and_scored_items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

                self.items = filtered_and_scored_items
                    .into_iter()
                    .map(|(_, item)| item.clone())
                    .collect();

                if self.items.is_empty() || self.cursor > self.items.len() - 1 {
                    self.cursor = 0;
                }

                Some(State::Active)
            }
            // Reset the items to the original list.
            _ => {
                self.items = all_items.to_vec();
                Some(State::Active)
            }
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
