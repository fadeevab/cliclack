use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use console::Term;

use crate::{progress::ProgressBar, theme::THEME, ThemeState};

const HEADER_HEIGHT: usize = 2;

/// A spinner + progress bar that renders progress indication.
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
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

        // HEADER_HEIGHT: 2 lines.
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

        let height = self.bars.write().unwrap().len() + HEADER_HEIGHT;
        term.clear_last_lines(height).ok();

        let state = &ThemeState::Submit;

        term.write_str(
            &THEME
                .lock()
                .unwrap()
                .format_header(state, (self.prompt.clone() + "\n ").trim_end()),
        )
        .ok();

        for bar in self.bars.write().unwrap().iter() {
            let message = bar.options().message.as_ref().unwrap().clone();
            bar.println(message, state);
        }
    }
}
