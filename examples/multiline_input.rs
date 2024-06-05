fn main() -> std::io::Result<()> {
    use cliclack::Input;

    let res: String = Input::new("Normal test")
        .placeholder("Type here...")
        .multiline(true)
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: usize = Input::new("Only number:")
        .placeholder("Type here...")
        .multiline(true)
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: String = Input::new("Interactively validation:")
        .multiline(true)
        .validate_interactively(|s: &String| match s.len() & 1 == 0 {
            true => Ok(()),
            false => Err("The length of the input should be even"),
        })
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: String = Input::new("Default value test:")
        .multiline(true)
        .default_input("Default value")
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: String = Input::new("Default value with interactively validation test:")
        .multiline(true)
        .default_input("Default value.")
        .validate_interactively(|s: &String| match s.len() & 1 == 0 {
            true => Ok(()),
            false => Err("The length of the input should be even"),
        })
        .interact()?;
    cliclack::note("Your input is:", res)?;

    // one-line
    let res: String = Input::new("Normal test (one-line)")
        .placeholder("Type here...")
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: usize = Input::new("Only number (one-line)")
        .placeholder("Type here...")
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: String = Input::new("Interactively validation (one-line)")
        .validate_interactively(|s: &String| match s.len() & 1 == 0 {
            true => Ok(()),
            false => Err("The length of the input should be even"),
        })
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: String = Input::new("Default value test (one-line)")
        .default_input("Default value")
        .interact()?;
    cliclack::note("Your input is:", res)?;

    let res: String = Input::new("Default value with interactively validation test (one-line)")
        .default_input("Default value.")
        .validate_interactively(|s: &String| match s.len() & 1 == 0 {
            true => Ok(()),
            false => Err("The length of the input should be even"),
        })
        .interact()?;
    cliclack::note("Your input is:", res)?;
    Ok(())
}
