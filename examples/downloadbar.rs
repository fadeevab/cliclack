use cliclack::{clear_screen, intro, log, outro, progressbar};
use console::style;
use rand::{thread_rng, Rng};
use std::{sync::mpsc::channel, time::Duration};

fn main() -> std::io::Result<()> {
    // Total number of bytes to simulate downloading.
    let total_bytes = 5_000_000;

    // Create a MPSC channel which will be used by the Ctrl-C handler to
    // asynchronously send a signal to the loop down below, allowing us to cancel
    // the operation.
    let (tx, rx) = channel();

    // Set the Ctrl-C handler to send a signal to the channel.
    ctrlc::set_handler(move || {
        let _ = tx.send(true);
    })
    .expect("setting Ctrl-C handler");

    // Clear the screen and print the header + a remark.
    clear_screen()?;
    intro(style(" progressbar ").on_cyan().black())?;
    log::remark("Press Esc, Enter, or Ctrl-C")?;

    // Create a new download progress bar
    let mut progressbar = progressbar().as_downloadbar();
    // Start the progress bar with the total number of bytes and a label
    progressbar.start(total_bytes, "Downloading, please wait...");

    // Loop until the progress bar reaches the total number of bytes
    while progressbar.get_position() < total_bytes {
        // Use a random timeout to simulate some work.
        let timeout = Duration::from_millis(thread_rng().gen_range(10..150));

        // Check if we received a signal from the channel (the Ctrl-C handler
        // registered above). We use this as our "thread::sleep()" operation
        // as well, to simulate work.
        if rx.recv_timeout(timeout).is_ok() {
            // If a message is received, cancel the progress bar and display an outro message
            progressbar.cancel("Installation")?;
            outro("Cancelled")?;
            return Ok(());
        }

        // Increment the progress bar with a random number of bytes
        progressbar.increment(thread_rng().gen_range(1_000..200_000));
    }

    // Stop the progress bar and display a completion message
    progressbar.stop("Installation");
    outro("Done!")?;

    Ok(())
}
