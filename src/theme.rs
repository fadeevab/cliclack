use console::{style, Emoji, Style};

use crate::prompt::{cursor::StringCursor, interaction::State};

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

pub enum ThemeState {
    Active,
    Cancel,
    Submit,
    Error(String),
}

impl<R> From<&State<R>> for ThemeState {
    fn from(state: &State<R>) -> Self {
        match state {
            State::Active => Self::Active,
            State::Cancel => Self::Cancel,
            State::Submit(_) => Self::Submit,
            State::Error(e) => Self::Error(e.clone()),
        }
    }
}

fn state_color(state: &ThemeState) -> Style {
    match state {
        ThemeState::Active => Style::new().cyan(),
        ThemeState::Cancel => Style::new().red(),
        ThemeState::Submit => Style::new().bright().black(),
        ThemeState::Error(_) => Style::new().yellow(),
    }
}

fn state_symbol(state: &ThemeState) -> String {
    let color = state_color(state);
    let green = Style::new().green();

    match state {
        ThemeState::Active => color.apply_to(S_STEP_ACTIVE),
        ThemeState::Cancel => color.apply_to(S_STEP_CANCEL),
        ThemeState::Submit => green.apply_to(S_STEP_SUBMIT),
        ThemeState::Error(_) => color.apply_to(S_STEP_ERROR),
    }
    .to_string()
}

fn cursor_parts_with_style(cursor: &StringCursor, new_style: &Style) -> String {
    let (left, cursor, right) = cursor.split();
    format!(
        "{left}{cursor}{right}",
        left = new_style.apply_to(left),
        cursor = style(cursor).reverse(),
        right = new_style.apply_to(right)
    )
}

pub trait Theme {
    fn format_intro(&self, title: &str) -> String {
        let color = state_color(&ThemeState::Submit);
        format!(
            "{start_bar}  {title}\n{bar}",
            start_bar = color.apply_to(S_BAR_START),
            bar = color.apply_to(S_BAR),
        )
    }

    fn format_outro(&self, message: &str) -> String {
        let color = state_color(&ThemeState::Submit);
        format!("{bar}  {message}\n", bar = color.apply_to(S_BAR_END))
    }

    fn format_cancel(&self, message: &str) -> String {
        let color = state_color(&ThemeState::Submit);
        format!(
            "{bar}  {message}",
            bar = color.apply_to(S_BAR_END),
            message = style(message).red()
        )
    }

    fn format_header(&self, state: &ThemeState, prompt: &str) -> String {
        format!(
            "{state_symbol}  {prompt}\n",
            state_symbol = state_symbol(state)
        )
    }

    fn format_input(&self, state: &ThemeState, cursor: &StringCursor) -> String {
        let new_style = &match state {
            ThemeState::Cancel => Style::new().dim().strikethrough(),
            ThemeState::Submit => Style::new().dim(),
            _ => Style::new(),
        };

        let input = &match state {
            ThemeState::Active | ThemeState::Error(_) => cursor_parts_with_style(cursor, new_style),
            _ => new_style.apply_to(cursor).to_string(),
        };

        format!("{bar}  {input}\n", bar = state_color(state).apply_to(S_BAR))
    }

    fn format_placeholder(&self, state: &ThemeState, cursor: &StringCursor) -> String {
        let new_style = &Style::new().dim();

        let placeholder = &match state {
            ThemeState::Active | ThemeState::Error(_) => cursor_parts_with_style(cursor, new_style),
            _ => new_style.apply_to(cursor).to_string(),
        };

        format!(
            "{bar}  {placeholder}\n",
            bar = state_color(state).apply_to(S_BAR)
        )
    }

    fn format_footer(&self, state: &ThemeState) -> String {
        format!(
            "{}\n", // '\n' vanishes by style applying, thus exclude it from styling
            state_color(state).apply_to(match state {
                ThemeState::Active => format!("{S_BAR_END}"),
                ThemeState::Cancel => format!("{S_BAR_END}  Operation cancelled."),
                ThemeState::Submit => format!("{S_BAR}"),
                ThemeState::Error(err) => format!("{S_BAR_END}  {err}"),
            })
        )
    }
}

pub struct ClackTheme;

impl Theme for ClackTheme {}
