use std::io;

use cliclack::{clear_screen, intro, log, outro, outro_cancel, spinner};
use console::{style, Key, Term};

fn main() -> std::io::Result<()> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    clear_screen()?;
    intro(style(" spinner ").on_cyan().black())?;

    let mut spinner = spinner();
    let message = format!("{}\n{}\n\n{}",
        style("Installation").bold(),
        style("Press Esc, Enter, or Ctrl-C").dim(),
        style("Check it out, we're multilining!").magenta().italic()
    );
    spinner.start(message);

    let term = Term::stderr();
    loop {
        match term.read_key() {
            Ok(Key::Escape) => {
                spinner.cancel("Installation");
                outro_cancel("Cancelled")?;
            }
            Ok(Key::Enter) => {
                spinner.stop("Installation");
                outro("Done!")?;
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                spinner.error("Installation");
                outro_cancel("Interrupted")?;
            }
            _ => continue,
        }
        break;
    }

    Ok(())
}
