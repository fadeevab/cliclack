use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use console::Term;

use crate::{progress::ProgressBar, theme::THEME, ThemeState};

const HEADER_HEIGHT: usize = 1;
const FOOTER_HEIGHT: usize = 1;

/// A spinner + progress bar that renders progress indication.
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
#[derive(Clone)]
pub struct MultiProgress {
    multi: indicatif::MultiProgress,
    bars: Arc<RwLock<Vec<ProgressBar>>>,
    prompt: String,
}

impl MultiProgress {
    /// Creates a new progress bar with a given length.
    pub fn new(prompt: impl Display) -> Self {
        let theme = THEME.lock().unwrap();
        let multi = indicatif::MultiProgress::new();

        let header =
            theme.format_header(&ThemeState::Active, (prompt.to_string() + "\n ").trim_end());

        multi.println(header).ok();

        Self {
            multi,
            bars: Default::default(),
            prompt: prompt.to_string(),
        }
    }

    /// Starts the progress bar.
    pub fn add(&self, pb: ProgressBar) -> ProgressBar {
        // Unset the last flag for all other progress bars: it affects rendering.
        for bar in self.bars.write().unwrap().iter_mut() {
            bar.options().last = false;
        }

        // Attention: deconstructing `pb` to avoid borrowing `pb.bar` twice.
        let ProgressBar { bar, options } = pb;
        let bar = self.multi.add(bar);
        {
            let mut options = options.write().unwrap();
            options.grouped = true;
            options.last = true;
        }

        let pb = ProgressBar { bar, options };
        self.bars.write().unwrap().push(pb.clone());
        pb
    }

    pub fn stop(&self) {
        let term = Term::stderr();

        let visible_count = self
            .bars
            .read()
            .unwrap()
            .iter()
            .filter(|bar| bar.options().message.is_some())
            .count();

        term.clear_last_lines(visible_count + HEADER_HEIGHT + FOOTER_HEIGHT)
            .ok();

        let state = &ThemeState::Submit;

        term.write_str(
            &THEME
                .lock()
                .unwrap()
                .format_header(state, (self.prompt.clone() + "\n ").trim_end()),
        )
        .ok();

        for bar in self.bars.read().unwrap().iter() {
            let message = bar.options().message.as_ref().cloned().unwrap_or_default();
            bar.println(message, state);
        }
    }
}
