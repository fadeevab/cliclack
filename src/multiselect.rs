use std::cell::RefCell;
use std::io;
use std::{fmt::Display, rc::Rc};

use console::Key;

use crate::prompt::term::TermSize;
use crate::{
    filter::{FilteredView, LabeledItem},
    prompt::{
        cursor::StringCursor,
        interaction::{Event, PromptInteraction, State},
    },
    theme::THEME,
};

#[derive(Clone)]
struct Checkbox<T> {
    value: T,
    label: String,
    hint: String,
    selected: bool,
}

impl<T> LabeledItem for Checkbox<T> {
    fn label(&self) -> &str {
        &self.label
    }
}

/// A prompt that asks for one or more selections from a list of options.
pub struct MultiSelect<T> {
    prompt: String,
    items: Vec<Rc<RefCell<Checkbox<T>>>>,
    cursor: usize,
    initial_values: Option<Vec<T>>,
    required: bool,
    filter: FilteredView<Checkbox<T>>,
    term: TermSize,
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
            filter: FilteredView::default(),
            term: TermSize::default(),
        }
    }

    /// Adds an item to the list of options.
    pub fn item(mut self, value: T, label: impl Display, hint: impl Display) -> Self {
        self.items.push(Rc::new(RefCell::new(Checkbox {
            value,
            label: label.to_string(),
            hint: hint.to_string(),
            selected: false,
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

    /// Enables the filter mode ("fuzzy search").
    ///
    /// The filter mode allows to filter the items by typing.
    pub fn filter_mode(mut self) -> Self {
        let term_size = self.term.get_max_rows();
        self.term
            .set_max_rows(term_size.checked_sub(1).unwrap_or(term_size));
        self.filter.enable();
        self
    }

    /// Set the max number of rows of items that are able to be displayed at once
    pub fn set_max_rows(mut self, size: usize) -> Self {
        self.term.set_max_rows(size);
        self
    }

    /// Starts the prompt interaction.
    pub fn interact(&mut self) -> io::Result<Vec<T>> {
        if self.items.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No items added to the list",
            ));
        }
        if let Some(initial_values) = &self.initial_values {
            for item in self.items.iter_mut() {
                if initial_values.contains(&item.borrow().value) {
                    item.borrow_mut().selected = true;
                }
            }
        }
        self.filter.set(self.items.to_vec());
        <Self as PromptInteraction<Vec<T>>>::interact(self)
    }
}

impl<T: Clone> PromptInteraction<Vec<T>> for MultiSelect<T> {
    fn on(&mut self, event: &Event) -> State<Vec<T>> {
        let Event::Key(key) = event;

        if let Some(state) = self.filter.on(key, self.items.clone()) {
            if self.filter.items().is_empty() || self.cursor > self.filter.items().len() - 1 {
                self.cursor = 0;
            }
            return state;
        }

        match key {
            Key::ArrowLeft | Key::ArrowUp | Key::Char('k') | Key::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }

                if self.cursor < self.term.get_pos() {
                    self.term.set_pos(self.cursor);
                }
            }
            Key::ArrowRight | Key::ArrowDown | Key::Char('j') | Key::Char('l') => {
                if !self.filter.items().is_empty() && self.cursor < self.filter.items().len() - 1 {
                    self.cursor += 1;
                }

                if self.cursor >= self.term.get_pos() + self.term.get_max_rows() {
                    self.term.set_pos(self.cursor - self.term.get_max_rows() + 1);
                }
            }
            Key::Char(' ') => {
                let mut item = self.filter.items()[self.cursor].borrow_mut();
                item.selected = !item.selected;
            }
            Key::Enter => {
                let selected_items = self
                    .items
                    .iter()
                    .map(|item| item.borrow())
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

        // Render the static header.
        let header = theme.format_header(&state.into(), &self.prompt);

        // Render the filter input until the user submits or cancels.
        let filter_line = if let Some(input) = self.filter.input() {
            match state {
                State::Submit(_) | State::Cancel => "".to_string(),
                _ => theme.format_input(&state.into(), input),
            }
        } else {
            "".to_string()
        };

        // When the user is submitting or canceling, the original items
        // are shown to display the final selection.
        // Otherwise, show the filtered items.
        let items_to_render = match state {
            State::Submit(_) | State::Cancel => &self.items,
            _ => self.filter.items(),
        };

        let mut items_render = String::new();
        for (i, item) in items_to_render
            .iter()
            .map(|i| i.borrow())
            .enumerate()
            .skip(self.term.get_pos())
            .take(self.term.get_max_rows())
        {
            items_render.push_str(&theme.format_multiselect_item(
                &state.into(),
                item.selected,
                i == self.cursor,
                &item.label,
                &item.hint,
            ));
        }

        let not_rendered_items = self.items.iter().filter(|i| i.borrow().selected).count()
            - self
                .filter
                .items()
                .iter()
                .filter(|i| i.borrow().selected)
                .count();

        // Render the footer with a hint about the number of selected items.
        let footer = if not_rendered_items > 0 {
            theme.format_footer_with_message(
                &state.into(),
                &format!(
                    "{not_rendered_items} selected item{s} not displayed",
                    s = if not_rendered_items > 1 { "s" } else { "" }
                ),
            )
        } else {
            theme.format_footer(&state.into())
        };

        header + &filter_line + &items_render + &footer
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
        let mut select = MultiSelect::<&str>::new("Select an item");
        let result = select.interact();
        assert_eq!(
            "No items added to the list",
            result.unwrap_err().to_string()
        );
    }
}
