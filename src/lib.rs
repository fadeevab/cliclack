mod password;
mod prompt;
mod text;
mod theme;
mod validate;

use std::{collections::HashMap, fmt::Display, io};

use console::Term;
use password::Password;
use theme::{ClackTheme, Theme};

use crate::text::Text;

// Re-export the PromptInteraction trait
pub use crate::prompt::interaction::PromptInteraction;

type ItemFn = fn(&HashMap<String, String>) -> io::Result<String>;

pub struct GroupItem {
    name: String,
    action: Box<ItemFn>,
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

pub fn password<S: Display>(prompt: S) -> Password {
    Password::new(prompt)
}

pub fn item(name: impl Display, action: ItemFn) -> GroupItem {
    GroupItem {
        name: name.to_string(),
        action: Box::new(action),
    }
}

pub fn group(items: Vec<GroupItem>) -> io::Result<HashMap<String, String>> {
    let mut result = HashMap::new();

    for GroupItem { name, action } in items {
        let ret: String = action(&result)?;
        result.insert(name, ret);
    }

    Ok(result)
}
