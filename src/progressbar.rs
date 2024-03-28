use std::{fmt::Display, time::Duration};

use console::Term;

use crate::{theme::THEME, ThemeState};

/// A spinner + progressbar that renders progress indication using current/total
/// semantics. If you're looking for a download bar (or a bar that deals with
/// bytes and formatting of bytes/KB/MB/GB, etc.), see [`DownloadBar`](crate::DownloadBar).
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
pub struct ProgressBar {
    progress_bar: indicatif::ProgressBar,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            progress_bar: indicatif::ProgressBar::new(100),
        }
        .as_progressbar()
    }
}

impl ProgressBar {
    /// Starts the progressbar.
    pub fn start(&mut self, length: u64, message: impl Display) {
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
        Term::stderr().move_cursor_up(1)?;
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

    /// Retrieves the current position of the progressbar.
    /// Note that this is _not_ the same as the current progress, which is
    /// `position / length`.
    pub fn get_position(&self) -> u64 {
        self.progress_bar.position()
    }

    /// Sets the position of the progressbar.
    pub fn set_position(&mut self, position: u64) {
        self.progress_bar.set_position(position);
    }

    /// Retrieves the length of the progressbar. This is the total number of
    /// steps, bytes, etc. and is used to calculate the progress, which is
    /// `position / length`.
    pub fn get_length(&self) -> u64 {
        self.progress_bar.length().unwrap()
    }

    /// Sets the length of the progressbar. This is the total number of steps,
    /// bytes, etc. and is used to calculate the progress, which is
    /// `position / length`.
    pub fn set_length(&mut self, length: u64) {
        self.progress_bar.set_length(length);
    }

    /// Formats the progressbar as a progressbar, using steps as the unit (i.e.
    /// 1/25, 2/25, etc.).
    pub fn as_progressbar(self) -> Self {
        let theme = THEME.lock().unwrap();
        self.progress_bar
            .enable_steady_tick(Duration::from_millis(100));
        self.progress_bar
            .set_style(theme.format_progressbar_start());
        self
    }

    /// Formats the progressbar as a download bar, using bytes as the unit (i.e.
    /// 1.2MB/5.0MB, etc.).
    pub fn as_downloadbar(self) -> Self {
        let theme = THEME.lock().unwrap();
        self.progress_bar
            .enable_steady_tick(Duration::from_millis(100));
        self.progress_bar
            .set_style(theme.format_downloadbar_start());
        self
    }
}
