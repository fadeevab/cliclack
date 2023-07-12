mod confirm;
mod input;
mod multiselect;
mod password;
mod prompt;
mod select;
mod spinner;
mod theme;
mod validate;

use std::fmt::Display;
use std::io;

use console::Term;

pub use confirm::Confirm;
pub use input::Input;
pub use multiselect::MultiSelect;
pub use password::Password;
pub use select::Select;
pub use spinner::Spinner;
pub use theme::{ClackTheme, Theme};

// Re-export the PromptInteraction trait
pub use crate::prompt::interaction::PromptInteraction;

fn term_write(line: String) -> io::Result<()> {
    Term::stderr().write_str(&line)
}

/// Clears the terminal.
pub fn clear_screen() -> io::Result<()> {
    Term::stdout().clear_screen()?;
    Term::stderr().clear_screen()
}

/// Prints the first message of the prompts sequence.
pub fn intro(title: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_intro(&title.to_string()))
}

/// Prints the last message of the prompts sequence.
pub fn outro(message: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_outro(&message.to_string()))
}

/// Prints the last message of the prompts sequence with a failure style.
pub fn outro_cancel(message: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_cancel(&message.to_string()))
}

pub fn input(prompt: impl Display) -> Input {
    Input::new(prompt)
}

pub fn password(prompt: impl Display) -> Password {
    Password::new(prompt)
}

pub fn select<T: Default + Clone + Eq>(prompt: impl Display) -> Select<T> {
    Select::new(prompt)
}

pub fn multiselect<T: Default + Clone + Eq>(prompt: impl Display) -> MultiSelect<T> {
    MultiSelect::new(prompt)
}

pub fn confirm(prompt: impl Display) -> Confirm {
    Confirm::new(prompt)
}

pub fn spinner() -> Spinner {
    Spinner::default()
}

pub fn note(prompt: impl Display, message: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_note(&prompt.to_string(), &message.to_string()))
}

pub mod log {
    use super::*;

    fn log(text: impl Display, symbol: impl Display) -> io::Result<()> {
        term_write(ClackTheme.format_log(&text.to_string(), &symbol.to_string()))
    }

    pub fn remark(text: impl Display) -> io::Result<()> {
        log(text, ClackTheme.remark_symbol())
    }

    pub fn info(text: impl Display) -> io::Result<()> {
        log(text, ClackTheme.info_symbol())
    }

    pub fn warning(message: impl Display) -> io::Result<()> {
        log(message, ClackTheme.warning_symbol())
    }

    pub fn error(message: impl Display) -> io::Result<()> {
        log(message, ClackTheme.error_symbol())
    }

    pub fn success(message: impl Display) -> io::Result<()> {
        log(message, ClackTheme.success_symbol())
    }

    pub fn step(message: impl Display) -> io::Result<()> {
        log(message, ClackTheme.step_symbol())
    }
}
