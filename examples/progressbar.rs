use std::{sync::mpsc::channel, time::Duration};

use cliclack::{clear_screen, intro, log, outro, progressbar};
use console::style;
use rand::{thread_rng, Rng};

fn main() -> std::io::Result<()> {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || {
        let _ = tx.send(true);
    })
        .expect("setting Ctrl-C handler");

    clear_screen()?;
    intro(style(" progressbar ").on_cyan().black())?;
    log::remark("Press Esc, Enter, or Ctrl-C")?;

    let mut progressbar = progressbar();
    progressbar.start(100, "Installation");

    for _ in 0..100 {
        let timeout = Duration::from_millis(thread_rng().gen_range(10..75));
        if rx.recv_timeout(timeout).is_ok() { 
            progressbar.cancel("Installation")?;
            outro("Cancelled")?;
            return Ok(());
        }
        progressbar.increment(1);
    }

    progressbar.stop("Installation");
    outro("Done!")?;

    Ok(())
}
