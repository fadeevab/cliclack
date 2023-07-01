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

impl<T> From<&State<T>> for ThemeState {
    fn from(state: &State<T>) -> Self {
        match state {
            State::Active => Self::Active,
            State::Cancel => Self::Cancel,
            State::Submit(_) => Self::Submit,
            State::Error(e) => Self::Error(e.clone()),
        }
    }
}

pub trait Theme {
    fn state_color(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => Style::new().cyan(),
            ThemeState::Cancel => Style::new().red(),
            ThemeState::Submit => Style::new().bright().black(),
            ThemeState::Error(_) => Style::new().yellow(),
        }
    }

    fn state_symbol(&self, state: &ThemeState) -> String {
        let color = self.state_color(state);
        let green = Style::new().green();

        match state {
            ThemeState::Active => color.apply_to(S_STEP_ACTIVE),
            ThemeState::Cancel => color.apply_to(S_STEP_CANCEL),
            ThemeState::Submit => green.apply_to(S_STEP_SUBMIT),
            ThemeState::Error(_) => color.apply_to(S_STEP_ERROR),
        }
        .to_string()
    }

    fn radio_symbol(&self, state: &ThemeState, selected: bool) -> String {
        match state {
            ThemeState::Active if selected => style(S_RADIO_ACTIVE).green(),
            ThemeState::Active if !selected => style(S_RADIO_INACTIVE).dim(),
            _ => style(Emoji("", "")),
        }
        .to_string()
    }

    fn input_style(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Cancel => Style::new().dim().strikethrough(),
            ThemeState::Submit => Style::new().dim(),
            _ => Style::new(),
        }
    }

    fn placeholder_style(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Cancel => Style::new().hidden(),
            _ => Style::new().dim(),
        }
    }

    fn cursor_with_style(&self, cursor: &StringCursor, new_style: &Style) -> String {
        let (left, cursor, right) = cursor.split();
        format!(
            "{left}{cursor}{right}",
            left = new_style.apply_to(left),
            cursor = style(cursor).reverse(),
            right = new_style.apply_to(right)
        )
    }

    fn password_mask(&self) -> char {
        S_PASSWORD_MASK.to_string().chars().next().unwrap()
    }

    fn format_intro(&self, title: &str) -> String {
        let color = self.state_color(&ThemeState::Submit);
        format!(
            "{start_bar}  {title}\n{bar}",
            start_bar = color.apply_to(S_BAR_START),
            bar = color.apply_to(S_BAR),
        )
    }

    fn format_outro(&self, message: &str) -> String {
        let color = self.state_color(&ThemeState::Submit);
        format!("{bar}  {message}\n", bar = color.apply_to(S_BAR_END))
    }

    fn format_cancel(&self, message: &str) -> String {
        let color = self.state_color(&ThemeState::Submit);
        format!(
            "{bar}  {message}",
            bar = color.apply_to(S_BAR_END),
            message = style(message).red()
        )
    }

    fn format_header(&self, state: &ThemeState, prompt: &str) -> String {
        format!(
            "{state_symbol}  {prompt}\n",
            state_symbol = self.state_symbol(state)
        )
    }

    fn format_footer(&self, state: &ThemeState) -> String {
        format!(
            "{}\n", // '\n' vanishes by style applying, thus exclude it from styling
            self.state_color(state).apply_to(match state {
                ThemeState::Active => format!("{S_BAR_END}"),
                ThemeState::Cancel => format!("{S_BAR_END}  Operation cancelled."),
                ThemeState::Submit => format!("{S_BAR}"),
                ThemeState::Error(err) => format!("{S_BAR_END}  {err}"),
            })
        )
    }

    fn format_input(&self, state: &ThemeState, cursor: &StringCursor) -> String {
        let new_style = &self.input_style(state);

        let input = &match state {
            ThemeState::Active | ThemeState::Error(_) => self.cursor_with_style(cursor, new_style),
            _ => new_style.apply_to(cursor).to_string(),
        };

        format!(
            "{bar}  {input}\n",
            bar = self.state_color(state).apply_to(S_BAR)
        )
    }

    fn format_placeholder(&self, state: &ThemeState, cursor: &StringCursor) -> String {
        let new_style = &self.placeholder_style(state);

        let placeholder = &match state {
            ThemeState::Active | ThemeState::Error(_) => self.cursor_with_style(cursor, new_style),
            ThemeState::Cancel => "".to_string(),
            _ => new_style.apply_to(cursor).to_string(),
        };

        format!(
            "{bar}  {placeholder}\n",
            bar = self.state_color(state).apply_to(S_BAR)
        )
    }

    fn format_select_item(
        &self,
        state: &ThemeState,
        selected: bool,
        label: &str,
        hint: &str,
    ) -> String {
        match state {
            ThemeState::Cancel | ThemeState::Submit if !selected => return String::new(),
            _ => {}
        }

        let radio = self.radio_symbol(state, selected);
        let input_style = &self.input_style(state);
        let inactive_style = &self.placeholder_style(state);

        let label = if selected {
            input_style.apply_to(label)
        } else {
            inactive_style.apply_to(label)
        }
        .to_string();

        let hint = match state {
            ThemeState::Active | ThemeState::Error(_) if !hint.is_empty() && selected => {
                inactive_style.apply_to(format!("({})", hint)).to_string()
            }
            _ => String::new(),
        };

        format!(
            "{bar}  {radio}{space1}{label}{space2}{hint}\n",
            bar = self.state_color(state).apply_to(S_BAR),
            space1 = if radio.is_empty() { "" } else { " " },
            space2 = if label.is_empty() { "" } else { " " }
        )
    }
}

pub struct ClackTheme;

impl Theme for ClackTheme {}
