use std::{sync::mpsc::channel, time::Duration};

use cliclack::{clear_screen, intro, log, outro, progressbar};
use console::style;
use rand::{thread_rng, Rng};

fn main() -> std::io::Result<()> {
    // Create a MPSC channel which will be used by the Ctrl-C handler to
    // asynchronously send a signal to the loop down below, allowing us to cancel
    // the operation.
    let (tx, rx) = channel();

    // Set the Ctrl-C handler to send a signal to the channel.
    ctrlc::set_handler(move || {
        let _ = tx.send(true);
    })
    .expect("setting Ctrl-C handler");

    // Clear the screen and print the header.
    clear_screen()?;
    intro(style(" progressbar ").on_cyan().black())?;
    log::remark("Press Esc, Enter, or Ctrl-C")?;

    // Create a new progressbar and set the text to "Installation".
    let mut progressbar = progressbar();
    progressbar.start(100, "Installation");

    // Simulate doing some stuff....
    for _ in 0..100 {
        // Use a random timeout to simulate some work.
        let timeout = Duration::from_millis(thread_rng().gen_range(10..75));

        // Check if we received a signal from the channel (the Ctrl-C handler
        // registered above). We use this as our "thread::sleep()" operation
        // as well, to simulate work.
        if rx.recv_timeout(timeout).is_ok() {
            // If a message is received, cancel the progress bar and display an outro message
            progressbar.cancel("Installation")?;
            outro("Cancelled")?;
            return Ok(());
        }

        // Otherwise, we increase the progressbar by the delta. In this case we're
        // using a fixed delta of 1, but otherwise this would be the _change in
        // progress_ from the last iteration to this one.
        progressbar.increment(1);
    }

    // Once we're done, we stop the progressbar and print the outro message.
    // This removes the progressbar and prints the message to the terminal.
    progressbar.stop("Installation");
    outro("Done!")?;

    Ok(())
}
