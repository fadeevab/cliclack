use std::cell::RefCell;
use std::{io, usize};
use std::{fmt::Display, rc::Rc};

use console::Key;
use termsize::Size;

use crate::{
    filter::{FilteredView, LabeledItem},
    prompt::{
        cursor::StringCursor,
        interaction::{Event, PromptInteraction, State},
    },
    theme::THEME,
};

#[derive(Clone)]
struct RadioButton<T> {
    value: T,
    label: String,
    hint: String,
}

impl<T> LabeledItem for RadioButton<T> {
    fn label(&self) -> &str {
        &self.label
    }
}

/// A prompt that asks for one selection from a list of options.
pub struct Select<T> {
    prompt: String,
    items: Vec<Rc<RefCell<RadioButton<T>>>>,
    cursor: usize,
    initial_value: Option<T>,
    filter: FilteredView<RadioButton<T>>,
    window_size: usize,
    window_pos: usize,
    term_size: Option<Size>,
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
            filter: FilteredView::default(),
            window_size: usize::MAX,
            window_pos: 0,
            term_size: termsize::get(),
        }
    }

    /// Adds an item to the selection prompt.
    pub fn item(mut self, value: T, label: impl Display, hint: impl Display) -> Self {
        self.items.push(Rc::new(RefCell::new(RadioButton {
            value,
            label: label.to_string(),
            hint: hint.to_string(),
        })));
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

    /// Enables the filter mode ("fuzzy search").
    ///
    /// The filter mode allows to filter the items by typing.
    pub fn filter_mode(mut self) -> Self {
        self.filter.enable();
        self
    }

    /// Sets the window size. This is the maximum number of items to display
    /// at once, triggering scrolling if necessary.
    pub fn window_size(mut self, size: usize) -> Self {
        self.window_size = size;
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

        // If the window size hasn't been specified manually, calculate it
        // based on the current size of the terminal.
        if let Some(size) = &self.term_size {
            // Determine the optimal maximum height of the window.
            let mut max_height = size.rows as usize - 3;
            if self.filter.is_enabled() {
                max_height -= 1;
            }

            // If the window size is not set or exceeds the maximum optimal height,
            // use the optimal height instead.
            if self.window_size == usize::MAX || self.window_size > max_height {
                self.window_size = max_height;
            }
        }

        if let Some(initial_value) = &self.initial_value {
            self.cursor = self
                .items
                .iter()
                .position(|item| item.borrow().value == *initial_value)
                .unwrap_or(self.cursor);
        }
        self.filter.set(self.items.to_vec());
        <Self as PromptInteraction<T>>::interact(self)
    }
}

impl<T: Clone> PromptInteraction<T> for Select<T> {
    fn on(&mut self, event: &Event) -> State<T> {
        let Event::Key(key) = event;

        if let Some(state) = self.filter.on(key, self.items.clone()) {
            if self.filter.items().is_empty() || self.cursor > self.filter.items().len() - 1 {
                self.cursor = 0;
            }
            return state;
        }

        match key {
            Key::ArrowUp | Key::ArrowLeft => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                if self.cursor < self.window_pos {
                    self.window_pos = self.cursor;
                }
            }
            Key::ArrowDown | Key::ArrowRight => {
                let filtered_item_count = self.filter.items().len();
                if !self.filter.items().is_empty() && self.cursor < filtered_item_count - 1 {
                    self.cursor += 1;
                }
                if self.cursor >= self.window_pos + self.window_size {
                    self.window_pos = self.cursor - self.window_size + 1;
                }
            }
            Key::Enter => {
                return State::Submit(self.filter.items()[self.cursor].borrow().value.clone());
            }
            _ => {}
        }

        State::Active
    }

    fn render(&mut self, state: &State<T>) -> String {
        let theme = THEME.lock().unwrap();

        let header_display = theme.format_header(&state.into(), &self.prompt);
        let footer_display = theme.format_footer(&state.into());

        let filter_display = if let Some(input) = &self.filter.input() {
            match state {
                State::Submit(_) | State::Cancel => "".to_string(),
                _ => theme.format_input(&state.into(), input),
            }
        } else {
            "".to_string()
        };

        let items_display: String = self
            .filter
            .items()
            .iter()
            .enumerate()
            .skip(self.window_pos)
            .take(self.window_size)
            .map(|(i, item)| {
                let item = item.borrow();
                theme.format_select_item(&state.into(), self.cursor == i, &item.label, &item.hint)
            })
            .collect();

        header_display + &filter_display + &items_display + &footer_display
    }

    /// Enable handling of the input in the filter mode.
    fn input(&mut self) -> Option<&mut StringCursor> {
        self.filter.input()
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
