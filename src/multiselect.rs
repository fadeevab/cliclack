use std::cell::RefCell;
use std::io;
use std::{fmt::Display, rc::Rc};

use console::Key;

use crate::view::ListView;
use crate::{
    filter::FilteredView,
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

impl<T> AsRef<str> for Checkbox<T> {
    fn as_ref(&self) -> &str {
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
    page: ListView,
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
            page: ListView::default(),
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
    ///
    /// The first matching item in list order receives focus and is scrolled into view.
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
        self.page.height = height;
        self
    }

    /// Starts the prompt interaction.
    pub fn interact(&mut self) -> io::Result<Vec<T>> {
        self.prepare()?;
        <Self as PromptInteraction<Vec<T>>>::interact(self)
    }

    fn prepare(&mut self) -> io::Result<()> {
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
            self.cursor = self
                .items
                .iter()
                .position(|item| initial_values.contains(&item.borrow().value))
                .unwrap_or(self.cursor);
            // Keep the first initial selection within the visible rows.
            self.page.start = (self.cursor + 1).saturating_sub(self.page.height);
        }
        self.filter.set(self.items.to_vec());
        Ok(())
    }
}

impl<T: Clone> PromptInteraction<Vec<T>> for MultiSelect<T> {
    fn on(&mut self, event: &Event) -> State<Vec<T>> {
        let Event::Key(key) = event;

        if let Some(state) = self.filter.on(key, &self.items) {
            if self.filter.items().is_empty() || self.cursor > self.filter.items().len() - 1 {
                self.cursor = 0;
                self.page.start = 0;
            }
            return state;
        }

        match key {
            Key::ArrowLeft | Key::ArrowUp | Key::Char('k') | Key::Char('h') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }

                if self.cursor < self.page.start {
                    self.page.start = self.cursor;
                }
            }
            Key::ArrowRight | Key::ArrowDown | Key::Char('j') | Key::Char('l') => {
                if !self.filter.items().is_empty() && self.cursor < self.filter.items().len() - 1 {
                    self.cursor += 1;
                }

                if self.cursor >= self.page.start + self.page.height {
                    self.page.start = self.cursor - self.page.height + 1;
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
        let theme = THEME.read().unwrap();

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
            .skip(self.page.start)
            .take(self.page.height)
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
            - items_to_render
                .iter()
                .skip(self.page.start)
                .take(self.page.height)
                .filter(|i| i.borrow().selected)
                .count();

        let selection_message = if not_rendered_items > 0 {
            format!(
                "{not_rendered_items} selected item{s} not displayed",
                s = if not_rendered_items > 1 { "s" } else { "" }
            )
        } else {
            String::new()
        };

        let has_above = self.page.start > 0;
        let has_below = self.page.start.saturating_add(self.page.height) < items_to_render.len();
        let scroll_indicator = theme.format_scroll_indicator(&state.into(), has_above, has_below);

        let footer_message = [scroll_indicator, selection_message]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("  ");

        // Render the footer with a hint about the scroll position and the
        // number of selected items.
        let footer = theme.format_footer_with_message(&state.into(), &footer_message);

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

    fn select_with_items() -> MultiSelect<usize> {
        let mut select = MultiSelect::new("Choose values");
        for index in 0..25 {
            select = select.item(index, format!("Value {index:02}"), "");
        }
        select
    }

    #[test]
    fn first_initial_selection_is_visible() {
        for (initial_values, height, expected_cursor, expected_start) in [
            (None, 7, 0, 0),
            (Some(vec![]), 7, 0, 0),
            (Some(vec![0]), 7, 0, 0),
            (Some(vec![6]), 7, 6, 0),
            (Some(vec![7]), 7, 7, 1),
            (Some(vec![20]), 7, 20, 14),
            (Some(vec![24]), 7, 24, 18),
            (Some(vec![20]), 1, 20, 20),
            (Some(vec![20]), usize::MAX, 20, 0),
            (Some(vec![25]), 7, 0, 0),
            (Some(vec![25, 24, 20, 20]), 7, 20, 14),
            (Some(vec![24, 2, 20]), 7, 2, 0),
            (Some((0..25).rev().collect()), 7, 0, 0),
        ] {
            let values = initial_values.as_deref().unwrap_or_default();
            let expected_values: Vec<_> = (0..25).filter(|value| values.contains(value)).collect();
            let mut select = select_with_items().max_rows(height).required(false);
            if let Some(values) = initial_values {
                select = select.initial_values(values);
            }
            select.prepare().unwrap();

            assert_eq!(select.cursor, expected_cursor);
            assert_eq!(select.page.start, expected_start);
            assert_eq!(select.page.height, height);

            let active = select.render(&State::Active);
            for index in 0..25 {
                let visible = index >= expected_start && index - expected_start < height;
                assert_eq!(active.contains(&format!("Value {index:02}")), visible);
            }

            let state = select.on(&Event::Key(Key::Enter));
            assert!(matches!(&state, State::Submit(values) if *values == expected_values));
            if !expected_values.is_empty() {
                assert!(select
                    .render(&state)
                    .contains(&format!("Value {expected_cursor:02}")));
            }
        }
    }

    #[test]
    fn navigate_and_toggle_from_first_initial_selection() {
        for (key, expected_cursor, expected_start, expected_values) in [
            (None, 20, 14, vec![24]),
            (Some(Key::ArrowUp), 19, 14, vec![19, 20, 24]),
            (Some(Key::ArrowDown), 21, 15, vec![20, 21, 24]),
        ] {
            let mut select = select_with_items().max_rows(7).initial_values(vec![24, 20]);
            select.prepare().unwrap();

            if let Some(key) = key {
                assert!(matches!(select.on(&Event::Key(key)), State::Active));
            }
            assert_eq!(select.cursor, expected_cursor);
            assert_eq!(select.page.start, expected_start);
            assert!(select
                .render(&State::Active)
                .contains(&format!("Value {expected_cursor:02}")));

            assert!(matches!(
                select.on(&Event::Key(Key::Char(' '))),
                State::Active
            ));
            let state = select.on(&Event::Key(Key::Enter));
            assert!(matches!(&state, State::Submit(values) if *values == expected_values));
        }
    }

    #[test]
    fn hidden_selection_message_counts_items_outside_the_visible_page() {
        for (start, height, filtered_indices, expected_hidden) in [
            (0, 7, None, 2),
            (14, 7, None, 2),
            (18, 7, None, 1),
            (0, usize::MAX, None, 0),
            (0, 7, Some(vec![2]), 2),
            (0, 7, Some(vec![2, 20, 24]), 0),
            (0, 1, Some(vec![2, 20]), 2),
            (1, 1, Some(vec![2, 20]), 2),
            (0, 7, Some(vec![]), 3),
        ] {
            let mut select = select_with_items()
                .max_rows(height)
                .initial_values(vec![2, 20, 24]);
            select.prepare().unwrap();
            select.page.start = start;
            if let Some(indices) = filtered_indices {
                select.filter.set(
                    indices
                        .iter()
                        .map(|&index| Rc::clone(&select.items[index]))
                        .collect(),
                );
            }

            let active = select.render(&State::Active);
            if expected_hidden == 0 {
                assert!(!active.contains("selected item"));
            } else {
                let noun = if expected_hidden == 1 {
                    "item"
                } else {
                    "items"
                };
                assert!(
                    active.contains(&format!("{expected_hidden} selected {noun} not displayed"))
                );
            }
        }
    }

    #[test]
    fn filtering_resets_focus_and_view_without_changing_selections() {
        let mut select = select_with_items()
            .max_rows(7)
            .initial_values(vec![24, 20])
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
        assert_eq!(select.page.start, 0);
        let label = select.filter.items()[0].borrow().label.clone();
        let active = select.render(&State::Active);
        assert!(active.contains(&label));
        assert!(active.contains("2 selected items not displayed"));

        let state = select.on(&Event::Key(Key::Enter));
        assert!(matches!(&state, State::Submit(values) if *values == vec![20, 24]));
    }

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
