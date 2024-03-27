use std::{fmt::Display, time::Duration};

use indicatif::ProgressStyle;

use crate::{theme::THEME, ThemeState};

/// A spinner + downloadbar that renders progress indication based on a 
/// total number of bytes.
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
pub struct DownloadBar {
    download_bar: indicatif::ProgressBar,
}

impl Default for DownloadBar {
    fn default() -> Self {
        let download_bar = indicatif::ProgressBar::new(100);
        download_bar.enable_steady_tick(Duration::from_millis(100));
        Self { download_bar }
    }
}

impl DownloadBar {
    /// Starts the downloadbar.
    pub fn start(&mut self, total_bytes: u64, message: impl Display) {
        let theme = THEME.lock().unwrap();

        self.download_bar.set_style(
            ProgressStyle::with_template(&theme.format_downloadbar_start())
                .unwrap()
                .tick_chars(&theme.spinner_chars())
                .progress_chars("#>-"),
        );
        self.download_bar.set_length(total_bytes);
        self.download_bar.set_message(message.to_string());
    }

    pub fn get_progress(&self) -> u64 {
        self.download_bar.position()
    }

    pub fn set_position(&self, position: u64) {
        self.download_bar.set_position(position);
    }

    pub fn increment(&mut self, delta: u64) {
        self.download_bar.inc(delta);
    }

    /// Stops the downloadbar.
    pub fn stop(&mut self, message: impl Display) {
        let theme = THEME.lock().unwrap();

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.download_bar
            .println(theme.format_downloadbar_stop(&message.to_string()));
        self.download_bar.finish_and_clear();
    }

    /// Makes the downloarbar stop with an error.
    pub fn error(&mut self, message: impl Display) -> std::io::Result<()> {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Error("".into());

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.download_bar
            .println(theme.format_downloadbar_with_state(&message.to_string(), state)?);
        self.download_bar.finish_and_clear();
        Ok(())
    }

    /// Cancel the downloadbar (stop with cancelling style).
    pub fn cancel(&mut self, message: impl Display) -> std::io::Result<()> {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Cancel;

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.download_bar
            .println(theme.format_downloadbar_with_state(&message.to_string(), state)?);
        self.download_bar.finish_and_clear();
        Ok(())
    }
}
