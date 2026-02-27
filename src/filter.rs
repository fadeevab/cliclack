use std::{cell::RefCell, rc::Rc};

use console::Key;

use crate::{
    autocomplete::Autocomplete,
    prompt::{cursor::StringCursor, interaction::State},
};

/// The list of items gathered (filtered) by interactive input using
/// `FilteredView::on` event in a selection prompt.
///
/// The filter keeps and tracks the list of items to be rendered, however,
/// the items list can be shrunk due to enabled filtering.
pub(crate) struct FilteredView<I: AsRef<str>> {
    /// Enables the filtered view.
    enabled: bool,
    /// Collects the input from the user.
    input: StringCursor,
    /// Represents a view of the filtered items.
    items: Vec<Rc<RefCell<I>>>,
}

impl<I> Default for FilteredView<I>
where
    I: AsRef<str>,
{
    fn default() -> Self {
        Self {
            enabled: false,
            input: StringCursor::default(),
            items: vec![],
        }
    }
}

impl<I> FilteredView<I>
where
    I: AsRef<str>,
{
    /// Sets the items to be filtered.
    ///
    /// The reason of having this method is having an ability to support
    /// a builder pattern of the selection prompt, where the items are added one by one,
    /// and the filter can be enabled and initialized at different moment of time.
    pub fn set(&mut self, items: Vec<Rc<RefCell<I>>>) {
        self.items = items;
    }

    /// Sets a predefined set of items for the view.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Returns the items in the view.
    pub fn items(&self) -> &[Rc<RefCell<I>>] {
        &self.items
    }

    /// Collects the input and filters the items from the list of all items.
    ///
    /// Uses the Jaro-Winkler similarity algorithm to score the items
    /// ([`strsim::jaro_winkler`]).
    pub fn on<T>(&mut self, key: &Key, all_items: Vec<Rc<RefCell<I>>>) -> Option<State<T>> {
        if !self.enabled {
            // Pass over the control.
            return None;
        }

        match key {
            // Need further processing of simple "up" and "down" actions.
            Key::ArrowDown | Key::ArrowUp => None,
            // Need moving up and down if no input provided.
            Key::ArrowLeft | Key::ArrowRight if self.input.is_empty() => None,
            // Need to submit the selected item.
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
                self.items = all_items.suggestions(&self.input.to_string());
                Some(State::Active)
            }
            // Reset the items to the original list.
            _ => {
                self.items = all_items.to_vec();
                Some(State::Active)
            }
        }
    }

    /// Returns the input cursor if the filter is enabled.
    /// It makes the outer code to handle the input.
    pub fn input(&mut self) -> Option<&mut StringCursor> {
        if !self.enabled {
            return None;
        }

        Some(&mut self.input)
    }
}
