fn main() -> std::io::Result<()> {
    use cliclack::Input;

    let res: String = Input::new("Try input mutiline text:")
        .placeholder("Type here...")
        .multiline(true)
        .interact()?;
    println!("Your input is:\n {}", res);
    println!("Your input len is:\n {}", res.len());
    Ok(())
}
