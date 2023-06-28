use console::{style, Emoji, Style};

use crate::{cursor::StringCursor, interaction::State};

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

fn color<R>(state: &State<R>) -> Style {
    match state {
        State::Active => Style::new().cyan(),
        State::Cancel => Style::new().red(),
        State::Submit(_) => Style::new().bright().black(),
        State::Error(_) => Style::new().yellow(),
    }
}

fn symbol<R>(state: &State<R>) -> String {
    let color = color(state);
    let green = Style::new().green();

    match state {
        State::Active => color.apply_to(S_STEP_ACTIVE),
        State::Cancel => color.apply_to(S_STEP_CANCEL),
        State::Submit(_) => green.apply_to(S_STEP_SUBMIT),
        State::Error(_) => color.apply_to(S_STEP_ERROR),
    }
    .to_string()
}

fn input_style<R>(state: &State<R>) -> Style {
    match state {
        State::Cancel => Style::new().dim().strikethrough(),
        State::Submit(_) => Style::new().dim(),
        _ => Style::new(),
    }
}

fn placeholder_style() -> Style {
    Style::new().dim()
}

fn format_header<R>(state: &State<R>, prompt: &str) -> String {
    format!("{state_symbol}  {prompt}\n", state_symbol = symbol(state))
}

fn format_input<R>(state: &State<R>, show_input: &str) -> String {
    format!("{bar}  {show_input}\n", bar = color(state).apply_to(S_BAR))
}

fn format_footer<R>(state: &State<R>) -> String {
    format!(
        "{}\n",
        color(state).apply_to(match state {
            State::Active => S_BAR_END.to_string(),
            State::Cancel => format!("{S_BAR_END}  Operation cancelled."),
            State::Submit(_) => S_BAR.to_string(),
            State::Error(err) => format!("{S_BAR_END}  {err}"),
        })
    )
}

fn format_cursor(cursor: &StringCursor, styling: &Style) -> String {
    let (left, cursor, right) = cursor.split();
    format!(
        "{left}{cursor}{right}",
        left = styling.apply_to(left),
        cursor = style(cursor).reverse(),
        right = styling.apply_to(right)
    )
}

pub trait Theme {
    fn render_text(
        &self,
        state: &State<String>,
        prompt: &str,
        input: &StringCursor,
        placeholder: &StringCursor,
    ) -> String {
        let (value, style) = if input.is_empty() {
            (placeholder, placeholder_style())
        } else {
            (input, input_style(state))
        };

        let show_input = &match state {
            State::Active | State::Error(_) => format_cursor(value, &style),
            _ => input_style(state).apply_to(input).to_string(),
        };

        let line1 = format_header(state, prompt);
        let line2 = format_input(state, show_input);
        let line3 = format_footer(state);

        line1 + &line2 + &line3
    }
}

pub struct ClackTheme;

impl Theme for ClackTheme {}
