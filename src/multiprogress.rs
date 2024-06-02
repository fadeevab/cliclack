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
    /// Creates a new multi-progress bar with a given prompt.
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

    /// Adds a progress bar and returns an internalized reference to it.
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
            options.grouped = true;
            options.last = true;
        }

        let pb = ProgressBar { bar, options };
        self.bars.write().unwrap().push(pb.clone());
        pb
    }

    /// Stops the multi-progress bar with a submitted (successful) state.
    pub fn stop(&self) {
        self.stop_with(&ThemeState::Submit)
    }

    /// Stops the multi-progress bar with a default cancel message.
    pub fn cancel(&self) {
        self.stop_with(&ThemeState::Cancel)
    }

    /// Stops the multi-progress bar with an error message.
    pub fn error(&self, error: impl Display) {
        self.stop_with(&ThemeState::Error(error.to_string()))
    }

    fn stop_with(&self, state: &ThemeState) {
        let mut inner_height = 0;

        // Redraw all progress bars.
        for pb in self.bars.read().unwrap().iter() {
            // Ignore cleared (hidden and stopped) progress bars.
            if pb.bar.message().is_empty() && pb.options().stopped {
                continue;
            }

            // Workaround: `bar.println` must be called before `bar.finish_and_clear`
            // to avoid lines "jumping" while terminal resizing.
            inner_height += pb.redraw_finished(pb.bar.message(), state);
            pb.bar.finish_and_clear();
        }

        let term = Term::stderr();

        // Move up to the header, clear and print the new header, then move down.
        term.move_cursor_up(inner_height).ok();
        term.clear_last_lines(HEADER_HEIGHT).ok();
        term.write_str(
            &THEME
                .lock()
                .unwrap()
                .format_header(state, (self.prompt.clone() + "\n ").trim_end()),
        )
        .ok();
        term.move_cursor_down(inner_height).ok();
    }
}
