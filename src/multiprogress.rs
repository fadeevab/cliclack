use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use console::Term;

use crate::{progress::ProgressBar, theme::THEME, ThemeState};

const HEADER_HEIGHT: usize = 1;

/// Renders other progress bars and spinners under a common header in a single visual block.
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
            bar.redraw_active();
        }

        // Attention: deconstructing `pb` to avoid borrowing `pb.bar` twice.
        let ProgressBar { bar, options } = pb;
        let bar = self.multi.add(bar);
        {
            let mut options = options.write().unwrap();
            options.grouped = Some(self.clone());
            options.last = true;
        }

        let pb = ProgressBar { bar, options };
        self.bars.write().unwrap().push(pb.clone());
        pb
    }

    /// Stops the progress bar in a submitted state.
    pub fn stop(&self) {
        self.stop_with(&ThemeState::Submit)
    }

    /// Stops the progress bar with a default cancel message.
    pub fn cancel(&self) {
        self.stop_with(&ThemeState::Cancel)
    }

    /// Stops the progress bar with an error message.
    pub fn error(&self, error: impl Display) {
        self.stop_with(&ThemeState::Error(error.to_string()))
    }

    fn stop_with(&self, state: &ThemeState) {
        let term = Term::stderr();

        for pb in self.bars.read().unwrap().iter() {
            pb.bar.finish_and_clear();
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

        for pb in self.bars.read().unwrap().iter() {
            pb.redraw_finished(pb.bar.message(), state);
        }
    }
}
