use cliclack::{outro, select};

fn main() -> std::io::Result<()> {
    let selected = select("Select a word")
        .item("hello", "hello", "hi")
        .item("world", "world", "world")
        .item("how", "how", "how")
        .item("are", "are", "are")
        .item("you", "you", "you")
        .item(
            "hello how are YOU",
            "hello how are YOU",
            "hello how are YOU",
        )
        .filter_mode()
        .interact()?;

    outro(format!("You chose: {selected}"))?;

    Ok(())
}
