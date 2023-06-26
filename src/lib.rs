use console::{style, Emoji, Style, StyledObject, Term};
use dialoguer::{theme::ColorfulTheme, Input};
use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::Display, io};

const S_STEP_ACTIVE: Emoji = Emoji("◆", "*");
const S_STEP_CANCEL: Emoji = Emoji("■", "x");
const S_STEP_ERROR: Emoji = Emoji("▲", "x");
const S_STEP_SUBMIT: Emoji = Emoji("◇", "o");

const S_BAR_START: Emoji = Emoji("┌", "T");
const S_BAR: Emoji = Emoji("│", "|");
const S_BAR_END: Emoji = Emoji("└", "—");

const S_RADIO_ACTIVE: Emoji = Emoji("●", ">");
const S_RADIO_INACTIVE: Emoji = Emoji("○", " ");
const S_CHECKBOX_ACTIVE: Emoji = Emoji("◻", "[•]");
const S_CHECKBOX_SELECTED: Emoji = Emoji("◼", "[+]");
const S_CHECKBOX_INACTIVE: Emoji = Emoji("◻", "[ ]");
const S_PASSWORD_MASK: Emoji = Emoji("▪", "•");

const S_BAR_H: Emoji = Emoji("─", "-");
const S_CORNER_TOP_RIGHT: Emoji = Emoji("╮", "+");
const S_CONNECT_LEFT: Emoji = Emoji("├", "+");
const S_CORNER_BOTTOM_RIGHT: Emoji = Emoji("╯", "+");

const S_INFO: Emoji = Emoji("●", "•");
const S_SUCCESS: Emoji = Emoji("◆", "*");
const S_WARN: Emoji = Emoji("▲", "!");
const S_ERROR: Emoji = Emoji("■", "x");

enum State {
    Initial,
    Active,
    Cancel,
    Submit,
    Error,
}

fn symbol(state: State) -> StyledObject<String> {
    let symbol = match state {
        State::Initial | State::Active => style(S_STEP_ACTIVE).cyan(),
        State::Cancel => style(S_STEP_CANCEL).red(),
        State::Error => style(S_STEP_ERROR).yellow(),
        State::Submit => style(S_STEP_SUBMIT).green(),
    };

    style(format!("{}\n{}", style(S_BAR).bright().black(), symbol))
}

static THEME: Lazy<ColorfulTheme> = Lazy::new(|| ColorfulTheme {
    prompt_prefix: symbol(State::Initial),
    prompt_suffix: style(format!("\n{}", S_BAR_END)).cyan(),
    success_prefix: symbol(State::Submit),
    success_suffix: style(format!("\n{}", S_BAR)).bright().black(),
    values_style: Style::new().for_stderr().bright().black(),
    error_prefix: symbol(State::Error),
    ..ColorfulTheme::default()
});

fn term_write_line(line: &str) -> io::Result<()> {
    Term::stderr().write_line(line)
}

pub struct GroupItem<F>
where
    F: FnOnce(&HashMap<String, String>) -> dialoguer::Result<String>,
{
    name: String,
    action: F,
}

pub fn clear_screen() -> io::Result<()> {
    Term::stderr().clear_screen()
}

pub fn intro<S: Display>(title: S) -> io::Result<()> {
    term_write_line(&format!(
        "{}  {}",
        style(S_BAR_START).bright().black(),
        title
    ))
}

pub fn outro<S: Display>(message: S) -> io::Result<()> {
    term_write_line(&format!(
        "{}\n{}  {}",
        style(S_BAR).bright().black(),
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

pub fn text<S: Display>(prompt: S) -> Input<'static, String> {
    Input::with_theme(&*THEME).with_prompt(prompt.to_string())
}

pub fn item<S: ToString, F>(name: S, action: F) -> GroupItem<F>
where
    F: FnOnce(&HashMap<String, String>) -> dialoguer::Result<String>,
{
    GroupItem {
        name: name.to_string(),
        action,
    }
}

pub fn group<F>(items: Vec<GroupItem<F>>) -> Result<HashMap<String, String>, io::Error>
where
    F: FnOnce(&HashMap<String, String>) -> dialoguer::Result<String>,
{
    let mut result = HashMap::new();

    for GroupItem { name, action } in items {
        let ret: String = action(&result).map_err(|e| match e {
            dialoguer::Error::IO(ioerr) => ioerr,
        })?;
        result.insert(name, ret);
    }

    Ok(result)
}
