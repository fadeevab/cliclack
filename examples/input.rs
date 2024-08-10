use cliclack::log;

fn main() -> std::io::Result<()> {
    // This case demonstrates the default behavior of `input()` when a non-`Option<>`
    // return type is used, i.e. `required(true)`.
    let name: String = cliclack::input("What's your name (required)?")
        .interact()?;

    log::remark(&format!("You entered: {}", name))?;

    // We can still use `required(false)` with a non-`Option<>` return type, of course.
    let city: String = cliclack::input("What city do you live in (optional)?")
        .required(false)
        .interact()?;

    log::remark(&format!("You entered: {}", city))?;

    // This case demonstrates the implicit behavior of `input()` when the return 
    // type is an `Option<T>`, i.e. `required(false)`.
    let email: Option<String> = cliclack::input("What's your email (optional)?")
        .interact()?;

    log::remark(&format!("You entered: {:?}", email))?;

    // But we can still use `required(true)` with an `Option<>` return type if we want...
    // Don't know why you would, but you can.
    let age: Option<i32> = cliclack::input("How old are you (required)?")
        .required(true)
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