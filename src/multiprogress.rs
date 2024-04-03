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
            bar.options_write().last = false;
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
        self.stop_with(&ThemeState::Submit)
    }

    pub fn cancel(&self) {
        self.stop_with(&ThemeState::Cancel)
    }

    pub fn error(&self, error: impl Display) {
        self.stop_with(&ThemeState::Error(error.to_string()))
    }

    fn stop_with(&self, state: &ThemeState) {
        let term = Term::stderr();

        for bar in self.bars.read().unwrap().iter() {
            // Corner case: stop the progress if it's not finished properly.
            if !bar.bar.is_finished() {
                let height = bar.bar().message().lines().count();
                bar.bar().finish_and_clear(); // It moves the cursor up...
                term.move_cursor_down(height + 1).ok(); // ...so move it down.
            }

            let height = if !bar.bar().message().is_empty() {
                if bar.options().last {
                    1 + FOOTER_HEIGHT
                } else {
                    1
                }
            } else {
                0
            };

            term.clear_last_lines(height).ok();
        }

        // Clear the header.
        term.clear_last_lines(HEADER_HEIGHT).ok();

        term.write_str(
            &THEME
                .lock()
                .unwrap()
                .format_header(state, (self.prompt.clone() + "\n ").trim_end()),
        )
        .ok();

        for bar in self.bars.read().unwrap().iter() {
            if bar.bar().message().is_empty() {
                continue;
            }
            bar.println(bar.bar().message(), state);
        }
    }
}
