use cliclack::{clear_screen, intro, log, outro, downloadbar};
use console::style;
use rand::{thread_rng, Rng};
use std::{sync::mpsc::channel, time::Duration};

fn main() -> std::io::Result<()> {
    let total_bytes = 5_000_000;
    let (tx, rx) = channel();

    ctrlc::set_handler(move || {
        let _ = tx.send(true);
    })
        .expect("setting Ctrl-C handler");

    clear_screen()?;
    intro(style(" progressbar ").on_cyan().black())?;
    log::remark("Press Esc, Enter, or Ctrl-C")?;

    let mut progressbar = downloadbar();
    progressbar.start(total_bytes, "Installation");

    let timeout = Duration::from_millis(100);
    while progressbar.get_progress() < total_bytes {
        if rx.recv_timeout(timeout).is_ok() { 
            progressbar.cancel("Installation")?;
            outro("Cancelled")?;
            return Ok(());
        }
        
        progressbar.increment(thread_rng().gen_range(1_000..200_000));
    }

    progressbar.stop("Installation");
    outro("Done!")?;

    Ok(())
}
