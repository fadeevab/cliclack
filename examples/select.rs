use cliclack::*;

fn main() -> std::io::Result<()> {
    let maybe = select("select an option (press Tab to skip)")
        .item("1", "Cool feature 1", "")
        .item("2", "Cool feature 2", "")
        .optional()
        .interact()?;

    outro("Outro message")?;

    if let Some(msg) = maybe {
        println!("Good choice: {msg}");
    } else {
        println!("C'mon, select one!");
    }

    Ok(())
}
