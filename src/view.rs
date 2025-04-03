/// Represents a visible page of items in a selection list.
#[derive(Clone)]
pub struct ListView {
    /// The height of the page (number of visible items).
    pub height: usize,
    /// The starting index of the page.
    pub start: usize,
}

impl Default for ListView {
    fn default() -> Self {
        Self {
            height: usize::MAX,
            start: 0,
        }
    }
}
