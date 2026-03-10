use console::style;
use std::io;

fn main() -> io::Result<()> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    cliclack::clear_screen()?;

    cliclack::intro(style(" autocomplete ").on_cyan().black())?;

    let languages = vec![
        "rust".to_string(),
        "javascript".to_string(),
        "typescript".to_string(),
        "python".to_string(),
        "go".to_string(),
        "java".to_string(),
        "c".to_string(),
        "cpp".to_string(),
        "ruby".to_string(),
        "swift".to_string(),
        "kotlin".to_string(),
        "php".to_string(),
    ];

    let language: String = cliclack::input("Pick a language")
        .autocomplete(languages)
        .interact()?;

    cliclack::outro(format!("Selected: {language}"))?;

    Ok(())
}
