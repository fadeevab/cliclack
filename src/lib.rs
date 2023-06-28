mod prompt;
mod text;
mod theme;

use std::{collections::HashMap, fmt::Display, io};

use console::Term;
use theme::{ClackTheme, Theme};

use crate::text::Text;

// Re-export the PromptInteraction trait
pub use crate::prompt::interaction::PromptInteraction;

pub struct GroupItem<F>
where
    F: FnOnce(&HashMap<String, String>) -> io::Result<String>,
{
    name: String,
    action: F,
}

fn term_write_line(line: String) -> io::Result<()> {
    Term::stderr().write_line(&line)
}

pub fn clear_screen() -> io::Result<()> {
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

pub fn item<S: ToString, F>(name: S, action: F) -> GroupItem<F>
where
    F: FnOnce(&HashMap<String, String>) -> io::Result<String>,
{
    GroupItem {
        name: name.to_string(),
        action,
    }
}

pub fn group<F>(items: Vec<GroupItem<F>>) -> io::Result<HashMap<String, String>>
where
    F: FnOnce(&HashMap<String, String>) -> io::Result<String>,
{
    let mut result = HashMap::new();

    for GroupItem { name, action } in items {
        let ret: String = action(&result)?;
        result.insert(name, ret);
    }

    Ok(result)
}
