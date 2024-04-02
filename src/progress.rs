use std::{
    fmt::Display,
    sync::{Arc, RwLock, RwLockWriteGuard},
};

use indicatif::ProgressStyle;

use crate::{theme::THEME, ThemeState};

#[derive(Default)]
pub(crate) struct ProgressBarOptions {
    pub template: String,
    pub message: Option<String>,
    pub grouped: bool,
    pub last: bool,
}

/// A spinner + progress bar that renders progress indication.
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
#[derive(Clone)]
pub struct ProgressBar {
    pub(crate) bar: indicatif::ProgressBar,
    pub(crate) options: Arc<RwLock<ProgressBarOptions>>,
}

impl ProgressBar {
    /// Creates a new progress bar with a given length.
    pub fn new(len: u64) -> Self {
        let this = Self {
            bar: indicatif::ProgressBar::new(len),
            options: Default::default(),
        };

        this.options().template = THEME.lock().unwrap().default_progress_template();

        this
    }

    /// Sets the template string for the progress bar according to
    /// [`indicatif::ProgressStyle`](https://docs.rs/indicatif/latest/indicatif/#templates).
    pub fn with_template(self, template: &str) -> Self {
        self.options().template = template.to_string();
        self
    }

    /// Returns a reference to the underlying progress bar to give access to its API.
    pub fn bar(&self) -> &indicatif::ProgressBar {
        &self.bar
    }

    /// Starts the progress bar.
    pub fn start(&self, message: impl Display) {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Active;
        let options = self.options();

        self.bar.set_style(
            ProgressStyle::with_template(&theme.format_progress_start(
                &options.template,
                options.grouped,
                options.last,
                state,
            ))
            .unwrap()
            .tick_chars(&theme.spinner_chars())
            .progress_chars(&theme.progress_chars()),
        );

        self.bar.set_message(message.to_string());
    }

    /// Stops the progress bar.
    pub fn stop(&self, message: impl Display) {
        let state = if !self.options().grouped {
            ThemeState::Submit
        } else {
            ThemeState::Active
        };

        self.println(message.to_string(), &state);
        self.preserve_finish_and_clear(message);
    }

    /// Makes the spinner stop with an error.
    pub fn error(&self, message: impl Display) {
        let state = if !self.options().grouped {
            ThemeState::Error("".into())
        } else {
            ThemeState::Active
        };

        self.println(message.to_string(), &state);
        self.preserve_finish_and_clear(message);
    }

    /// Cancel the spinner (stop with cancelling style).
    pub fn cancel(&self, message: impl Display) {
        let state = if !self.options().grouped {
            ThemeState::Cancel
        } else {
            ThemeState::Active
        };

        self.println(message.to_string(), &state);
        self.preserve_finish_and_clear(message);
    }

    /// Accesses the options for writing (a convenience function).
    #[inline]
    pub(crate) fn options(&self) -> RwLockWriteGuard<'_, ProgressBarOptions> {
        self.options.write().unwrap()
    }

    /// Redraws the progress bar with a new message.
    ///
    /// The method is semi-open for multi-progress bar purposes.
    pub(crate) fn println(&self, message: impl Display, state: &ThemeState) {
        let theme = THEME.lock().unwrap();
        let options = self.options();

        let msg = theme.format_progress_with_state(
            &message.to_string(),
            options.grouped,
            options.last,
            state,
        );

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.bar.println(msg);
    }

    /// Preserves the message for a multi-progress bar and clears the bar.
    fn preserve_finish_and_clear(&self, message: impl Display) {
        self.options().message = Some(message.to_string());
        self.bar.finish_and_clear();
    }
}
