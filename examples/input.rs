use cliclack::log;

fn main() -> std::io::Result<()> {
    // This case requires input will never return an empty string.
    let name: String = cliclack::input("What's your name (required)?")
        .interact()?;

    log::remark(&format!("You entered: {}", name))?;

    // This case does not require input. The result will be `None` if the user 
    // does not provide any input (i.e. just presses <enter>), and `Some(18)` 
    // if the user provides "18".
    let age: Option<i32> = cliclack::input("How old are you (optional)?")
        .placeholder("18")
        .required(false)
        .interact()?;

    log::remark(&format!("You entered: {:?}", age))?;

    // This case shows bad usage:
    // - The result will never be empty due to `default_input()` being set, so 
    //   `required()` has no effect. 
    // - The `Option<>` return type is superfluous as it will never return `None`
    //   due to `default_input()` being set.
    let desc: Option<String> = cliclack::input("How would you describe cliclack?")
        .placeholder("Awesome!")
        .required(true) // This is not required because the result will never be empty due to `default_input()` being set.
        .default_input("Awesome!")
        .interact()?;

    log::remark(&format!("You entered: {:?}", desc))?;

    Ok(())
}