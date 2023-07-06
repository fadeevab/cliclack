mod confirm;
mod multiselect;
mod password;
mod prompt;
mod select;
mod text;
mod theme;
mod validate;

use std::{fmt::Display, io};

use console::Term;
use multiselect::MultiSelect;
use password::Password;
use select::Select;
use theme::{ClackTheme, Theme};

use crate::text::Text;

// Re-export the PromptInteraction trait
pub use crate::prompt::interaction::PromptInteraction;

fn term_write_line(line: String) -> io::Result<()> {
    Term::stderr().write_line(&line)
}

pub fn clear_screen() -> io::Result<()> {
    Term::stdout().clear_screen()?;
    Term::stderr().clear_screen()
}

pub fn intro<S: Display>(title: S) -> io::Result<()> {
    term_write_line(ClackTheme.format_intro(&title.to_string()))
}

pub fn outro<S: Display>(message: S) -> io::Result<()> {
    term_write_line(ClackTheme.format_outro(&message.to_string()))
}

pub fn cancel<S: Display>(message: S) -> io::Result<()> {
    term_write_line(ClackTheme.format_cancel(&message.to_string()))
}

pub fn text<S: Display>(prompt: S) -> Text {
    Text::new(prompt)
}

pub fn password<S: Display>(prompt: S) -> Password {
    Password::new(prompt)
}

pub fn select<S: Display, T: Default + Clone + Eq>(prompt: S) -> Select<T> {
    Select::new(prompt)
}

pub fn multiselect<S: Display, T: Default + Clone + Eq>(prompt: S) -> MultiSelect<T> {
    MultiSelect::new(prompt)
}

pub fn confirm<S: Display>(prompt: S) -> confirm::Confirm {
    confirm::Confirm::new(prompt)
}
