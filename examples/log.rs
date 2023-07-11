use console::style;

fn main() -> std::io::Result<()> {
    clacky::intro(style(" log ").on_cyan().black())?;
    clacky::log::remark("This is a simple message")?;
    clacky::log::warning("This is a warning")?;
    clacky::log::error("This is an error")?;
    clacky::log::success("This is a success")?;
    clacky::log::info("This is an info")?;
    clacky::log::step("This is a step")?;
    clacky::outro_cancel("Like it's cancelled")?;

    Ok(())
}
