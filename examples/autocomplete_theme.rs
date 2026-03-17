use cliclack::{Theme, ThemeState};
use console::style;
use std::io;

struct AutocompleteTheme;

impl Theme for AutocompleteTheme {
    fn format_footer_for_autocomplete(&self, state: &ThemeState, _message: &str) -> String {
        match state {
            ThemeState::Active => format!("{}\n", self.bar_color(state).apply_to("└◇ ")),
            _ => self.format_footer(state),
        }
    }

    fn format_autocomplete_item(&self, state: &ThemeState, active: bool, label: &str) -> String {
        format!(
            " {bar} {item}\n",
            bar = self.bar_color(state).apply_to("│"),
            item = if active {
                self.bar_color(state)
            } else {
                self.input_style(state)
            }
            .apply_to(label)
        )
    }
}

fn main() -> io::Result<()> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    cliclack::set_theme(AutocompleteTheme);

    cliclack::clear_screen()?;

    cliclack::intro(style(" autocomplete ").on_cyan().black())?;

    let languages = vec![
        "javascript".to_string(),
        "typescript".to_string(),
        "python".to_string(),
        "go".to_string(),
        "java".to_string(),
        "c".to_string(),
        "cpp".to_string(),
        "ruby".to_string(),
        "rust".to_string(),
        "swift".to_string(),
        "kotlin".to_string(),
        "php".to_string(),
    ];

    let language: String = cliclack::input("Pick a language")
        .default_input("rust")
        .autocomplete(languages)
        .interact()?;

    let food: String = cliclack::input("What's your favorite food?")
        .autocomplete(|_query: &str| {
            // Pretend being dynamic.
            vec![
                "pizza".to_string(),
                "sushi".to_string(),
                "ice cream".to_string(),
            ]
        })
        .interact()?;

    cliclack::outro(format!("Selected: {language} and {food}"))?;

    Ok(())
}
