use cliclack::{clear_screen, intro, log, outro, outro_cancel, spinner};
use console::{style, Key, Term};

fn main() -> std::io::Result<()> {
    clear_screen()?;
    intro(style(" spinner ").on_cyan().black())?;
    log::remark("Press Esc, Enter, or Ctrl-C")?;

    let spinner = spinner();
    spinner.start("Installation");

    let term = Term::stderr();
    loop {
        match term.read_key_raw() {
            Ok(Key::Escape) => {
                spinner.cancel("Installation");
                outro_cancel("Cancelled")?;
            }
            Ok(Key::Enter) => {
                spinner.stop("Installation");
                outro("Done!")?;
            }
            Ok(Key::CtrlC) => {
                spinner.error("Installation");
                outro_cancel("Interrupted")?;
            }
            _ => continue,
        }
        break;
    }

    Ok(())
}
