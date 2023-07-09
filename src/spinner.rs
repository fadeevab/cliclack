use std::{fmt::Display, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};

use crate::theme::{ClackTheme, Theme};

pub struct Spinner {
    spinner: ProgressBar,
}

impl Default for Spinner {
    fn default() -> Self {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(Duration::from_millis(100));
        Self { spinner }
    }
}

impl Spinner {
    pub fn start(&mut self, message: impl Display) {
        self.spinner.set_style(
            ProgressStyle::with_template(&ClackTheme.format_spinner_start())
                .unwrap()
                .tick_chars(&ClackTheme.spinner_chars()),
        );

        self.spinner.set_message(message.to_string());
    }

    pub fn stop(&mut self, message: impl Display) {
        self.spinner
            .set_style(ProgressStyle::with_template(&ClackTheme.format_spinner_stop()).unwrap());

        self.spinner.finish_with_message(message.to_string());
    }
}
