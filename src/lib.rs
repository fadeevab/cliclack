mod confirm;
mod multiselect;
mod password;
mod prompt;
mod select;
mod spinner;
mod text;
mod theme;
mod validate;

use std::fmt::Display;
use std::io;

use confirm::Confirm;
use console::Term;
use multiselect::MultiSelect;
use password::Password;
use select::Select;
use spinner::Spinner;
use theme::{ClackTheme, Theme};

use crate::text::Text;

// Re-export the PromptInteraction trait
pub use crate::prompt::interaction::PromptInteraction;

fn term_write(line: String) -> io::Result<()> {
    Term::stderr().write_str(&line)
}

pub fn clear_screen() -> io::Result<()> {
    Term::stdout().clear_screen()?;
    Term::stderr().clear_screen()
}

pub fn intro(title: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_intro(&title.to_string()))
}

pub fn outro(message: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_outro(&message.to_string()))
}

pub fn cancel(message: impl Display) -> io::Result<()> {
    term_write(ClackTheme.format_cancel(&message.to_string()))
}

pub fn text(prompt: impl Display) -> Text {
    Text::new(prompt)
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

    pub fn message(text: impl Display) -> io::Result<()> {
        log(text, ClackTheme.message_symbol())
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
