use std::{
    fmt::Display,
    sync::{Arc, RwLock, RwLockWriteGuard},
    time::Duration,
};

use indicatif::ProgressStyle;

use crate::{theme::THEME, ThemeState};

#[derive(Default)]
pub(crate) struct ProgressBarOptions {
    pub template: String,
    pub grouped: bool,
    pub last: bool,
}

/// A progress bar renders progress indication. Supports spinner and download templates.
/// Can be used for both single and multi-progress bars
/// (see [`MultiProgress`](crate::multiprogress::MultiProgress)).
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

        this.options_write().template = THEME.lock().unwrap().default_progress_template();

        this
    }

    /// Sets a default spinner visual template for the progress bar.
    pub fn with_spinner_template(self) -> Self {
        self.options_write().template = THEME.lock().unwrap().default_spinner_template();
        self
    }

    /// Sets a default visual template for downloading.
    pub fn with_download_template(self) -> Self {
        self.options_write().template = THEME.lock().unwrap().default_download_template();
        self
    }

    /// Sets a custom template string for the progress bar according to
    /// [`indicatif::ProgressStyle`](https://docs.rs/indicatif/latest/indicatif/#templates).
    pub fn with_template(self, template: &str) -> Self {
        self.options_write().template = template.to_string();
        self
    }

    /// Returns a reference to the underlying progress bar to give access to its API.
    pub fn bar(&self) -> &indicatif::ProgressBar {
        &self.bar
    }

    /// Starts the progress bar.
    pub fn start(&self, message: impl Display) {
        let theme = THEME.lock().unwrap();
        let options = self.options();

        self.bar.set_style(
            ProgressStyle::with_template(&theme.format_progress_start(
                &options.template,
                options.grouped,
                options.last,
            ))
            .unwrap()
            .tick_chars(&theme.spinner_chars())
            .progress_chars(&theme.progress_chars()),
        );

        self.bar
            .set_message(theme.format_progress_message(&message.to_string()));
        self.bar.enable_steady_tick(Duration::from_millis(100));
    }

    /// Stops the progress bar.
    pub fn stop(&self, message: impl Display) {
        if !self.first_stop(message.to_string()) {
            return;
        }

        let state = if !self.options().grouped {
            ThemeState::Submit
        } else {
            ThemeState::Active
        };

        self.print_finish_and_clear(message, &state);
    }

    /// Cancel the spinner (stop with cancelling style).
    pub fn cancel(&self, message: impl Display) {
        if !self.first_stop(message.to_string()) {
            return;
        }

        let state = if !self.options().grouped {
            ThemeState::Cancel
        } else {
            ThemeState::Active
        };

        self.print_finish_and_clear(message, &state);
    }

    /// Makes the spinner stop with an error.
    pub fn error(&self, message: impl Display) {
        if !self.first_stop(message.to_string()) {
            return;
        }

        let state = if !self.options().grouped {
            ThemeState::Error("".into())
        } else {
            ThemeState::Active
        };

        self.print_finish_and_clear(message, &state);
    }

    /// Accesses the options for writing (a convenience function).
    #[inline]
    pub(crate) fn options_write(&self) -> RwLockWriteGuard<'_, ProgressBarOptions> {
        self.options.write().unwrap()
    }

    /// Accesses the options for reading (a convenience function).
    #[inline]
    pub(crate) fn options(&self) -> RwLockWriteGuard<'_, ProgressBarOptions> {
        self.options.write().unwrap()
    }

    /// Redraws the progress bar with a new message.
    ///
    /// The method is semi-open for multi-progress bar purposes.
    pub(crate) fn println(&self, message: impl Display, state: &ThemeState) {
        let theme = THEME.lock().unwrap();
        let options = self.options.read().unwrap();

        let msg = theme.format_progress_with_state(
            &message.to_string(),
            options.grouped,
            options.last,
            state,
        );

        // Workaround: the next line doesn't "jump" around while resizing the terminal.
        self.bar.println(msg);
    }

    /// Redraws the progress bar without changing the message.
    pub(crate) fn redraw_as_started(&self) {
        let theme = THEME.lock().unwrap();
        let options = self.options();

        self.bar.set_style(
            ProgressStyle::with_template(&theme.format_progress_start(
                &options.template,
                options.grouped,
                options.last,
            ))
            .unwrap()
            .tick_chars(&theme.spinner_chars())
            .progress_chars(&theme.progress_chars()),
        );
    }

    /// Redraws the progress bar without changing the message.
    fn redraw_as_stopped(&self) {
        let theme = THEME.lock().unwrap();
        let options = self.options();

        self.bar.set_style(
            ProgressStyle::with_template(&theme.format_progress_with_state(
                &self.bar.message(),
                options.grouped,
                options.last,
                &ThemeState::Active,
            ))
            .unwrap(),
        );
    }

    fn first_stop(&self, message: String) -> bool {
        if self.bar.is_finished() {
            return false;
        }

        // Corner case: preserve the message for the last stop.
        self.bar.set_message(message);

        if self.options().grouped && self.options().last {
            // Corner case: if the last progress bar is stopped first,
            // we need to redraw it and keep talking to indicatif::MultiBar
            // that it's still active to avoid a jumping effect.
            self.redraw_as_stopped();
            return false;
        }

        true
    }

    fn print_finish_and_clear(&self, message: impl Display, state: &ThemeState) {
        self.println(message.to_string(), state);
        self.bar.finish_and_clear();
    }
}
