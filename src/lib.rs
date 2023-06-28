mod cursor;
mod interaction;
mod text;
mod theme;

use crate::text::Text;
use console::{style, Emoji, Term};
use std::{collections::HashMap, fmt::Display, io};

fn term_write_line(line: &str) -> io::Result<()> {
    Term::stderr().write_line(line)
}

pub use interaction::PromptInteraction;

pub struct GroupItem<F>
where
    F: FnOnce(&HashMap<String, String>) -> io::Result<String>,
{
    name: String,
    action: F,
}

pub fn clear_screen() -> io::Result<()> {
    Term::stderr().clear_screen()
}

const S_BAR_START: Emoji = Emoji("┌", "T");
const S_BAR: Emoji = Emoji("│", "|");
const S_BAR_END: Emoji = Emoji("└", "—");

pub fn intro<S: Display>(title: S) -> io::Result<()> {
    term_write_line(&format!(
        "{}  {}\n{}",
        style(S_BAR_START).bright().black(),
        title,
        style(S_BAR).bright().black(),
    ))
}

pub fn outro<S: Display>(message: S) -> io::Result<()> {
    term_write_line(&format!(
        "{}  {}",
        style(S_BAR_END).bright().black(),
        message
    ))
}

pub fn cancel<S: Display>(message: S) -> io::Result<()> {
    term_write_line(&format!(
        "{}  {}",
        style(S_BAR_END).bright().black(),
        style(message).red()
    ))
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
