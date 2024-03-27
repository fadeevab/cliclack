use std::{fmt::Display, time::Duration};

use indicatif::ProgressStyle;

use crate::{theme::THEME, ThemeState};

/// A spinner that renders progress indication.
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
pub struct ProgressBar {
    progress_bar: indicatif::ProgressBar,
}

impl Default for ProgressBar {
    fn default() -> Self {
        let progress_bar = indicatif::ProgressBar::new(100);
        progress_bar.enable_steady_tick(Duration::from_millis(100));
        Self { progress_bar }
    }
}

impl ProgressBar {
    /// Starts the progressbar.
    pub fn start(&mut self, length: u64, message: impl Display) {
        let theme = THEME.lock().unwrap();

        self.progress_bar.set_style(
            ProgressStyle::with_template(&theme.format_progressbar_start())
                .unwrap()
                .tick_chars(&theme.spinner_chars())
                .progress_chars("#>-"),
        );

        self.progress_bar.set_length(length);
        self.progress_bar.set_message(message.to_string());
    }

    pub fn increment(&mut self, delta: u64) {
        self.progress_bar.inc(delta);
    }

    /// Stops the progressbar.
    pub fn stop(&mut self, message: impl Display) {
        let theme = THEME.lock().unwrap();

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.progress_bar
            .println(theme.format_progressbar_stop(&message.to_string()));
        self.progress_bar.finish_and_clear();
    }

    /// Makes the progressbar stop with an error.
    pub fn error(&mut self, message: impl Display) -> std::io::Result<()> {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Error("".into());

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.progress_bar
            .println(theme.format_progressbar_with_state(&message.to_string(), state)?);
        self.progress_bar.finish_and_clear();
        Ok(())
    }

    /// Cancel the progressbar (stop with cancelling style).
    pub fn cancel(&mut self, message: impl Display) -> std::io::Result<()> {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Cancel;

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.progress_bar
            .println(theme.format_progressbar_with_state(&message.to_string(), state)?);
        self.progress_bar.finish_and_clear();
        Ok(())
    }
}
