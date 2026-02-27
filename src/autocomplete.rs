use std::{cell::RefCell, rc::Rc};

/// Provides autocomplete suggestions for an input string.
///
/// Designed to work with different data sources without requiring cloning,
/// allowing results to be borrowed, shared (Rc), or owned.
pub trait Autocomplete {
    /// The type of autocomplete suggestions.
    type Result<'a>
    where
        Self: 'a;

    /// Returns the candidate suggestions for the given `input`.
    fn suggestions(&self, input: &str) -> Vec<Self::Result<'_>>;
}

/// Turns a vector of shared displayable elements into a fuzzy autocomplete source.
///
/// Labels are taken from `item.as_ref()`. Suggestions keep
/// shared ownership by returning `Rc<RefCell<T>>` values.
impl<T> Autocomplete for Vec<Rc<RefCell<T>>>
where
    T: AsRef<str>,
{
    type Result<'a>
        = Rc<RefCell<T>>
    where
        Self: 'a;

    fn suggestions(&self, input: &str) -> Vec<Rc<RefCell<T>>> {
        let labeled_items = self
            .iter()
            .map(|i| (i.borrow().as_ref().to_lowercase(), Rc::clone(i)));
        fuzzy(labeled_items, &input.to_lowercase())
    }
}

/// Turns a vector of strings into a fuzzy autocomplete source.
///
/// Each string is matched by its own contents and suggestions are returned
/// as borrowed `&str` slices into the original vector.
impl Autocomplete for Vec<String> {
    type Result<'a>
        = &'a str
    where
        Self: 'a;

    fn suggestions(&self, input: &str) -> Vec<&str> {
        let labeled_items = self.iter().map(|s| (s.to_lowercase(), s.as_str()));
        fuzzy(labeled_items, &input.to_lowercase())
    }
}

/// Turns a handler function into a dynamic autocomplete source.
///
/// Useful for dynamic suggestion sources that compute and return matches
/// directly for the given input.
impl<F, T> Autocomplete for F
where
    F: for<'a> Fn(&'a str) -> Vec<T>,
{
    type Result<'a>
        = T
    where
        Self: 'a;

    fn suggestions(&self, input: &str) -> Vec<Self::Result<'_>> {
        (self)(input)
    }
}

/// Ranks items by fuzzy similarity between their labels and the input.
///
/// Empty or whitespace input keeps all items in original order.
///
/// Otherwise:
/// - Uses Jaro-Winkler similarity between label and input.
/// - Adds a bonus if all whitespace-separated input words appear in the label.
/// - Filters weak matches (score <= 0.6) and sorts by score descending.
///
/// Labels are used only for scoring. The output contains the associated items.
fn fuzzy<I>(items: impl Iterator<Item = (String, I)>, input: &str) -> Vec<I> {
    if input.trim().is_empty() {
        return items.map(|(_, item)| item).collect();
    }

    let filter_words: Vec<_> = input.split_whitespace().collect();

    let mut scored: Vec<_> = items
        .map(|(label, item)| {
            let similarity = strsim::jaro_winkler(&label, input);
            let bonus = filter_words.iter().all(|word| label.contains(*word)) as usize as f64;
            (similarity + bonus, item)
        })
        .filter(|(score, _)| *score > 0.6)
        .collect();

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    scored.into_iter().map(|(_, item)| item).collect()
}
