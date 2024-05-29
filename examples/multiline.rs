#[cfg(not(feature = "multiline"))]
compile_error!("This example needs feature `multiline`");

use cliclack::Input;

fn main() -> std::io::Result<()> {
    let res: String = Input::new("Try input mutiline text:")
        .placeholder("Yes")
        .multiline(true)
        .interact()?;
    println!("Your input is:\n {}", res);
    Ok(())
}
