use cliclack::{input, intro, log, outro_note, select};
use console::style;

fn main() -> std::io::Result<()> {
    intro(style(" optional ").on_cyan().black())?;

    let optional_input: Option<u64> = input("Enter your number:")
        .required(false)
        .interact::<String>()?
        .parse() // String -> Result<u64>
        .ok(); // -> Option<u64>

    log::info(format!("{optional_input:?}"))?;

    let optional_select: Option<&str> = select("Select a driver")
        .item(None, "<none>", "skip")
        .item(Some("mysql"), "MySQL", "")
        .item(Some("postgres"), "PostgreSQL", "")
        .item(Some("sqlite"), "SQLite", "")
        .interact()?;

    log::info(format!("{optional_select:?}"))?;

    outro_note(
        "Done!",
        format!("Number: {optional_input:?}\nDriver: {optional_select:?}"),
    )?;

    Ok(())
}
