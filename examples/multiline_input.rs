fn main() -> std::io::Result<()> {
    use cliclack::Input;

    let res: String = Input::new("Try input mutiline text:")
        .placeholder("Type here...")
        .multiline(true)
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: String = Input::new("Try input one line:")
        .placeholder("Type here...")
        .interact()?;
    cliclack::outro(format!("Your input is: {res}"))?;
    Ok(())
}
