use std::{cell::RefCell, rc::Rc};

/// Provides a list of suggestions for an input.
///
/// Designed to work as a source of autocomplete suggestions, or self-filtering,
/// without requiring cloning, allowing results to be shared (Rc) or owned.
pub trait Suggest {
    /// The type of suggestions.
    type Result;

    /// Returns the candidate suggestions for the given `input`.
    fn suggest(&self, input: &str) -> Vec<Self::Result>;
}

/// Turns a vector of shared displayable elements into a fuzzy searchable source.
///
/// Labels are taken from `item.as_ref()`. Suggestions keep
/// shared ownership by returning `Rc<RefCell<T>>` values.
impl<T> Suggest for Vec<Rc<RefCell<T>>>
where
    T: AsRef<str>,
{
    type Result = Rc<RefCell<T>>;

    fn suggest(&self, input: &str) -> Vec<Rc<RefCell<T>>> {
        let labeled_items = self
            .iter()
            .map(|i| (i.borrow().as_ref().to_lowercase(), Rc::clone(i)));
        fuzzy(labeled_items, &input.to_lowercase())
    }
}

/// Turns a vector of strings into a fuzzy searchable source.
///
/// Each string is matched by its own contents and suggestions are returned
/// as owned `String` values.
impl Suggest for Vec<String> {
    type Result = String;

    fn suggest(&self, input: &str) -> Vec<String> {
        let labeled_items = self.iter().map(|s| (s.to_lowercase(), s.clone()));
        fuzzy(labeled_items, &input.to_lowercase())
    }
}

/// Turns a handler function into a dynamic source of suggestions.
///
/// Useful for dynamic suggestions computed directly for the given input.
impl<F, T> Suggest for F
where
    F: for<'a> Fn(&'a str) -> Vec<T>,
{
    type Result = T;

    fn suggest(&self, input: &str) -> Vec<Self::Result> {
        (self)(input)
    }
}

/// Ranks items by fuzzy similarity between their labels and the input.
///
/// Empty or whitespace input keeps all items in original order.
///
/// Otherwise:
/// - Uses Jaro-Winkler similarity algorithm ([`strsim::jaro_winkler`]).
/// - Adds a bonus if all whitespace-separated input words appear in the label.
/// - Filters weak matches (score <= 0.6) and sorts by score descending.
///
/// Labels are used only for scoring. The output contains the associated items.
fn fuzzy<T>(items: impl Iterator<Item = (String, T)>, input: &str) -> Vec<T> {
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
