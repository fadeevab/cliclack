use std::cell::RefCell;
use std::io;
use std::{fmt::Display, rc::Rc};

use console::Key;

use crate::{
    filter::FilteredView,
    prompt::{
        cursor::StringCursor,
        interaction::{Event, PromptInteraction, State},
    },
    theme::THEME,
    view::ListView,
};

#[derive(Clone)]
struct RadioButton<T> {
    value: T,
    label: String,
    hint: String,
}

impl<T> AsRef<str> for RadioButton<T> {
    fn as_ref(&self) -> &str {
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
    view: ListView,
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
            view: ListView::default(),
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

    /// The maximum visible number of items.
    ///
    /// If the number of items is greater than the maximum number of rows,
    /// the list will be scrolled.
    ///
    /// By default, the selection prompt will display all items at once.
    pub fn max_rows(mut self, height: usize) -> Self {
        self.view.height = height;
        self
    }

    /// Starts the prompt interaction.
    pub fn interact(&mut self) -> io::Result<T> {
        self.prepare()?;
        <Self as PromptInteraction<T>>::interact(self)
    }

    fn prepare(&mut self) -> io::Result<()> {
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
                .position(|item| item.borrow().value == *initial_value)
                .unwrap_or(self.cursor);
            // Keep the initial selection within the visible rows.
            self.view.start = (self.cursor + 1).saturating_sub(self.view.height);
        }
        self.filter.set(self.items.to_vec());
        Ok(())
    }
}

impl<T: Clone> PromptInteraction<T> for Select<T> {
    fn on(&mut self, event: &Event) -> State<T> {
        let Event::Key(key) = event;

        if let Some(state) = self.filter.on(key, &self.items) {
            if self.filter.items().is_empty() || self.cursor > self.filter.items().len() - 1 {
                self.cursor = 0;
                self.view.start = 0;
            }
            return state;
        }

        match key {
            Key::ArrowUp | Key::ArrowLeft | Key::Char('k') | Key::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }

                if self.cursor < self.view.start {
                    self.view.start = self.cursor;
                }
            }
            Key::ArrowDown | Key::ArrowRight | Key::Char('j') | Key::Char('l') => {
                if !self.filter.items().is_empty() && self.cursor < self.filter.items().len() - 1 {
                    self.cursor += 1;
                }

                if self.cursor >= self.view.start + self.view.height {
                    self.view.start = self.cursor - self.view.height + 1;
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
        let theme = THEME.read().unwrap();

        let header_display = theme.format_header(&state.into(), &self.prompt);

        let has_above = self.view.start > 0;
        let has_below =
            self.view.start.saturating_add(self.view.height) < self.filter.items().len();
        let scroll_indicator = theme.format_scroll_indicator(&state.into(), has_above, has_below);
        let footer_display = theme.format_footer_with_message(&state.into(), &scroll_indicator);

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
            .skip(self.view.start)
            .take(self.view.height)
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

    fn select_with_items() -> Select<usize> {
        let mut select = Select::new("Choose a value");
        for index in 0..25 {
            select = select.item(index, format!("Value {index:02}"), "");
        }
        select
    }

    #[test]
    fn initial_selection_is_visible() {
        for (initial_value, height, expected_cursor, expected_start) in [
            (None, 7, 0, 0),
            (Some(0), 7, 0, 0),
            (Some(6), 7, 6, 0),
            (Some(7), 7, 7, 1),
            (Some(20), 7, 20, 14),
            (Some(24), 7, 24, 18),
            (Some(20), 1, 20, 20),
            (Some(20), usize::MAX, 20, 0),
            (Some(25), 7, 0, 0),
        ] {
            let mut select = select_with_items().max_rows(height);
            if let Some(value) = initial_value {
                select = select.initial_value(value);
            }
            select.prepare().unwrap();

            assert_eq!(select.cursor, expected_cursor);
            assert_eq!(select.view.start, expected_start);

            let active = select.render(&State::Active);
            for index in 0..25 {
                let visible = index >= expected_start && index - expected_start < height;
                assert_eq!(active.contains(&format!("Value {index:02}")), visible);
            }

            let state = select.on(&Event::Key(Key::Enter));
            assert!(matches!(&state, State::Submit(value) if *value == expected_cursor));
            let submitted = select.render(&state);
            assert!(submitted.contains(&format!("Value {expected_cursor:02}")));
            assert_eq!(submitted.matches("Value ").count(), 1);
        }
    }

    #[test]
    fn navigate_from_initial_selection() {
        for (key, expected_cursor, expected_start) in
            [(Key::ArrowUp, 19, 14), (Key::ArrowDown, 21, 15)]
        {
            let mut select = select_with_items().max_rows(7).initial_value(20);
            select.prepare().unwrap();

            assert!(matches!(select.on(&Event::Key(key)), State::Active));
            assert_eq!(select.cursor, expected_cursor);
            assert_eq!(select.view.start, expected_start);
            assert!(select
                .render(&State::Active)
                .contains(&format!("Value {expected_cursor:02}")));

            let state = select.on(&Event::Key(Key::Enter));
            assert!(matches!(&state, State::Submit(value) if *value == expected_cursor));
            assert!(select
                .render(&state)
                .contains(&format!("Value {expected_cursor:02}")));
        }
    }

    #[test]
    fn filtering_resets_initial_selection_and_view() {
        let mut select = select_with_items()
            .max_rows(7)
            .initial_value(20)
            .filter_mode();
        select.prepare().unwrap();

        // The interaction loop updates the input before dispatching the key.
        select.input().unwrap().insert('0');
        assert!(matches!(
            select.on(&Event::Key(Key::Char('0'))),
            State::Active
        ));
        assert!(!select.filter.items().is_empty());
        assert!(select.filter.items().len() < 20);
        assert_eq!(select.cursor, 0);
        assert_eq!(select.view.start, 0);
        let label = select.filter.items()[0].borrow().label.clone();
        assert!(select.render(&State::Active).contains(&label));
    }

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
