use std::fmt::Display;

use indicatif::ProgressStyle;

use crate::{theme::THEME, ThemeState};

/// A spinner + progress bar that renders progress indication using current/total
/// semantics. If you're looking for a download bar (or a bar that deals with
/// bytes and formatting of bytes/KB/MB/GB, etc.), see [`DownloadBar`](crate::DownloadBar).
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
pub struct ProgressBar {
    bar: indicatif::ProgressBar,
    template: String,
}

impl ProgressBar {
    /// Creates a new progress bar with a given length.
    pub fn new(len: u64) -> Self {
        let theme = THEME.lock().unwrap();
        let bar = indicatif::ProgressBar::new(len);
        Self {
            bar,
            template: theme.default_progress_template(),
        }
    }

    /// Sets the template string for the progress bar according to
    /// [`indicatif::ProgressStyle`](https://docs.rs/indicatif/latest/indicatif/#templates).
    pub fn with_template(self, template: &str) -> Self {
        Self {
            template: template.into(),
            ..self
        }
    }

    /// Returns a reference to the underlying progress bar to give access to its API.
    pub fn bar(&self) -> &indicatif::ProgressBar {
        &self.bar
    }

    /// Starts the progress bar.
    pub fn start(&self, message: impl Display) {
        let theme = THEME.lock().unwrap();

        self.bar.set_style(
            ProgressStyle::with_template(&theme.format_progress_start(&self.template))
                .unwrap()
                .tick_chars(&theme.spinner_chars())
                .progress_chars(&theme.progress_chars()),
        );

        self.bar.set_message(message.to_string());
    }

    /// Stops the spinner.
    pub fn stop(&self, message: impl Display) {
        let theme = THEME.lock().unwrap();

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.bar
            .println(theme.format_spinner_stop(&message.to_string()));
        self.bar.finish_and_clear();
    }

    /// Makes the spinner stop with an error.
    pub fn error(&self, message: impl Display) {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Error("".into());

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.bar
            .println(theme.format_spinner_with_state(&message.to_string(), state));
        self.bar.finish_and_clear();
    }

    /// Cancel the spinner (stop with cancelling style).
    pub fn cancel(&self, message: impl Display) {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Cancel;

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.bar
            .println(theme.format_spinner_with_state(&message.to_string(), state));
        self.bar.finish_and_clear();
    }
}
