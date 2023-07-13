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
const S_WARN: Emoji = Emoji("▲", "!");
const S_ERROR: Emoji = Emoji("■", "x");

const S_SPINNER: Emoji = Emoji("◒◐◓◑", "•oO0");

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
    fn bar_color(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => Style::new().cyan(),
            ThemeState::Cancel => Style::new().red(),
            ThemeState::Submit => Style::new().bright().black(),
            ThemeState::Error(_) => Style::new().yellow(),
        }
    }

    fn state_symbol(&self, state: &ThemeState) -> String {
        let color = self.bar_color(state);
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

    fn checkbox_symbol(&self, state: &ThemeState, selected: bool, active: bool) -> String {
        match state {
            ThemeState::Active | ThemeState::Error(_) => {
                if selected {
                    style(S_CHECKBOX_SELECTED).green()
                } else if active && !selected {
                    style(S_CHECKBOX_ACTIVE).cyan()
                } else if !active && !selected {
                    style(S_CHECKBOX_INACTIVE).dim()
                } else {
                    style(Emoji("", ""))
                }
            }
            _ => style(Emoji("", "")),
        }
        .to_string()
    }

    fn remark_symbol(&self) -> String {
        self.bar_color(&ThemeState::Submit)
            .apply_to(S_CONNECT_LEFT)
            .to_string()
    }

    fn info_symbol(&self) -> String {
        style(S_INFO).blue().to_string()
    }

    fn warning_symbol(&self) -> String {
        style(S_WARN).yellow().to_string()
    }

    fn error_symbol(&self) -> String {
        style(S_ERROR).red().to_string()
    }

    fn active_symbol(&self) -> String {
        style(S_STEP_ACTIVE).green().to_string()
    }

    fn submit_symbol(&self) -> String {
        style(S_STEP_SUBMIT).green().to_string()
    }

    fn checkbox_style(&self, state: &ThemeState, selected: bool, active: bool) -> Style {
        match state {
            ThemeState::Cancel if selected => Style::new().dim().strikethrough(),
            ThemeState::Submit if selected => Style::new().dim(),
            _ if !active => Style::new().dim(),
            _ => Style::new(),
        }
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
        let color = self.bar_color(&ThemeState::Submit);
        format!(
            "{start_bar}  {title}\n{bar}\n",
            start_bar = color.apply_to(S_BAR_START),
            bar = color.apply_to(S_BAR),
        )
    }

    fn format_outro(&self, message: &str) -> String {
        let color = self.bar_color(&ThemeState::Submit);
        format!(
            "{bar}\n{bar_end}  {message}\n",
            bar = color.apply_to(S_BAR),
            bar_end = color.apply_to(S_BAR_END)
        )
    }

    fn format_cancel(&self, message: &str) -> String {
        let color = self.bar_color(&ThemeState::Submit);
        format!(
            "{bar}  {message}\n",
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
            self.bar_color(state).apply_to(match state {
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
            bar = self.bar_color(state).apply_to(S_BAR)
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
            bar = self.bar_color(state).apply_to(S_BAR)
        )
    }

    fn radio_item(&self, state: &ThemeState, selected: bool, label: &str, hint: &str) -> String {
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
            "{radio}{space1}{label}{space2}{hint}",
            space1 = if radio.is_empty() { "" } else { " " },
            space2 = if label.is_empty() { "" } else { " " }
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

        format!(
            "{bar}  {radio_item}\n",
            bar = self.bar_color(state).apply_to(S_BAR),
            radio_item = self.radio_item(state, selected, label, hint)
        )
    }

    fn checkbox_item(
        &self,
        state: &ThemeState,
        selected: bool, // when item is selected/checked
        active: bool,   // when cursors highlights item
        label: &str,
        hint: &str,
    ) -> String {
        match state {
            ThemeState::Cancel | ThemeState::Submit if !selected => return String::new(),
            _ => {}
        }

        let checkbox = self.checkbox_symbol(state, selected, active);
        let label_style = self.checkbox_style(state, selected, active);
        let hint_style = self.placeholder_style(state);
        let label = label_style.apply_to(label).to_string();

        let hint = match state {
            ThemeState::Active | ThemeState::Error(_) if !hint.is_empty() && active => {
                hint_style.apply_to(format!("({})", hint)).to_string()
            }
            _ => String::new(),
        };

        format!(
            "{checkbox}{space1}{label}{space2}{hint}",
            space1 = if checkbox.is_empty() { "" } else { " " },
            space2 = if label.is_empty() { "" } else { " " }
        )
    }

    fn format_multiselect_item(
        &self,
        state: &ThemeState,
        selected: bool, // when item is selected/checked
        active: bool,   // when cursors highlights item
        label: &str,
        hint: &str,
    ) -> String {
        match state {
            ThemeState::Cancel | ThemeState::Submit if !selected => return String::new(),
            _ => {}
        }

        format!(
            "{bar}  {checkbox_item}\n",
            bar = self.bar_color(state).apply_to(S_BAR),
            checkbox_item = self.checkbox_item(state, selected, active, label, hint),
        )
    }

    fn format_confirm(&self, state: &ThemeState, confirm: bool) -> String {
        let yes = self.radio_item(state, confirm, "Yes", "");
        let no = self.radio_item(state, !confirm, "No", "");

        let inactive_style = &self.placeholder_style(state);
        let divider = match state {
            ThemeState::Active => inactive_style.apply_to(" / ").to_string(),
            _ => "".to_string(),
        };

        format!(
            "{bar}  {yes}{divider}{no}\n",
            bar = self.bar_color(state).apply_to(S_BAR),
        )
    }

    fn format_spinner_start(&self) -> String {
        "{spinner:.magenta}  {msg}".into()
    }

    fn format_spinner_stop(&self) -> String {
        format!(
            "{symbol}  {{msg}}\n{bar}\n",
            symbol = self.state_symbol(&ThemeState::Submit),
            bar = self.bar_color(&ThemeState::Submit).apply_to(S_BAR)
        )
    }

    fn spinner_chars(&self) -> String {
        S_SPINNER.to_string()
    }

    fn format_note(&self, prompt: &str, message: &str) -> String {
        let message = format!("\n{message}\n");
        let width = 2 + message
            .split('\n')
            .fold(0usize, |acc, line| line.chars().count().max(acc))
            .max(prompt.chars().count());

        let symbol = self.state_symbol(&ThemeState::Submit);
        let bar_color = self.bar_color(&ThemeState::Submit);
        let text_color = self.input_style(&ThemeState::Submit);

        let header = format!(
            "{symbol}  {prompt} {horizontal_bar}{corner}\n",
            horizontal_bar =
                bar_color.apply_to(S_BAR_H.to_string().repeat(width - prompt.chars().count())),
            corner = bar_color.apply_to(S_CORNER_TOP_RIGHT),
        );
        let body = message
            .lines()
            .map(|line| {
                format!(
                    "{bar}  {line}{spaces}{bar}\n",
                    bar = bar_color.apply_to(S_BAR),
                    line = text_color.apply_to(line),
                    spaces = " ".repeat(width - line.chars().count() + 1)
                )
            })
            .collect::<String>();

        let footer = bar_color
            .apply_to(format!(
                "{S_CONNECT_LEFT}{horizontal_bar}{S_CORNER_BOTTOM_RIGHT}\n",
                horizontal_bar = S_BAR_H.to_string().repeat(width + 3),
            ))
            .to_string();

        header + &body + &footer
    }

    fn format_log(&self, text: &str, symbol: &str) -> String {
        let mut parts = vec![];
        let mut lines = text.lines().chain("\n".lines());

        if let Some(first) = lines.next() {
            parts.push(format!("{symbol}  {first}"));
        }

        for line in lines {
            parts.push(format!(
                "{bar}  {line}",
                bar = self.bar_color(&ThemeState::Submit).apply_to(S_BAR)
            ));
        }

        parts.push("".into());
        parts.join("\n")
    }
}

pub(crate) struct ClackTheme;

impl Theme for ClackTheme {}
