use std::{cell::RefCell, rc::Rc};

use console::Key;

use crate::prompt::{cursor::StringCursor, interaction::State};

pub(crate) trait LabeledItem {
    fn label(&self) -> &str;
}

pub(crate) struct FilteredView<I: LabeledItem> {
    enabled: bool,
    input: StringCursor,
    items: Vec<Rc<RefCell<I>>>,
}

impl<I: LabeledItem> Default for FilteredView<I> {
    fn default() -> Self {
        Self {
            enabled: false,
            input: StringCursor::default(),
            items: vec![],
        }
    }
}

impl<I: LabeledItem + Clone> FilteredView<I> {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn set(&mut self, items: Vec<Rc<RefCell<I>>>) {
        self.items = items;
    }

    pub fn items(&self) -> &[Rc<RefCell<I>>] {
        &self.items
    }

    pub fn on<T>(&mut self, key: &Key, all_items: Vec<Rc<RefCell<I>>>) -> Option<State<T>> {
        if !self.enabled {
            // Pass over the control.
            return None;
        }

        match key {
            // Need further processing of simple up and down actions.
            Key::ArrowDown | Key::ArrowUp => None,
            // Need moving up and down the list if no input provided.
            Key::ArrowLeft | Key::ArrowRight if self.input.is_empty() => None,
            // Need submitting of the selected item.
            Key::Enter if !self.items.is_empty() => None,
            // Otherwise, no items found.
            Key::Enter => Some(State::Error("No items".into())),
            // Ignore spaces passing through.
            Key::Char(' ') => {
                self.input.delete_left();
                None
            }
            // Refresh the filtered items for the rest of the keys.
            _ if !self.input.is_empty() => {
                let input_lower = self.input.to_string();
                let filter_words: Vec<_> = input_lower.split_whitespace().collect();

                let mut filtered_and_scored_items: Vec<_> = all_items
                    .into_iter()
                    .map(|item| {
                        let label = item.borrow().label().to_lowercase();
                        let input = self.input.to_string().to_lowercase();
                        let similarity = strsim::jaro_winkler(&label, &input);
                        let bonus = filter_words
                            .iter()
                            .all(|word| label.contains(&word.to_lowercase()))
                            as usize as f64;
                        (similarity + bonus, item)
                    })
                    .filter(|(similarity, _)| *similarity > 0.6)
                    .collect();

                filtered_and_scored_items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

                self.items = filtered_and_scored_items
                    .into_iter()
                    .map(|(_, item)| item)
                    .collect();

                Some(State::Active)
            }
            // Reset the items to the original list.
            _ => {
                self.items = all_items.to_vec();
                Some(State::Active)
            }
        }
    }

    pub fn input(&mut self) -> Option<&mut StringCursor> {
        if !self.enabled {
            return None;
        }

        Some(&mut self.input)
    }
}
