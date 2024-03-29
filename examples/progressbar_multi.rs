use std::{sync::mpsc::channel, time::Duration};

use cliclack::{clear_screen, intro, log, outro, progressbar_multi};
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
    intro(style(" multi-progressbar ").on_cyan().black())?;
    log::remark("Press Ctrl-C")?;

    let multi = progressbar_multi("Doing stuff...");

    let mut progressbar = multi.add_progressbar();
    let mut downloadbar = multi.add_downloadbar();

    progressbar.start(1000, "Copying files...");
    downloadbar.start(1000, "Downloading files...");

    // Simulate doing some stuff....
    while !progressbar.is_finished() || !downloadbar.is_finished() {
        // Use a random timeout to simulate some work.
        let timeout = Duration::from_millis(thread_rng().gen_range(10..75));

        // Check if we received a signal from the channel (the Ctrl-C handler
        // registered above). We use this as our "thread::sleep()" operation
        // as well, to simulate work.
        if rx.recv_timeout(timeout).is_ok() {
            // If a message is received, cancel the progress bar and display an outro message
            progressbar.cancel("Copying files")?;
            downloadbar.cancel("Downloading files")?;
            outro("Cancelled")?;
            return Ok(());
        }

        // Otherwise, we increase the progressbars by the delta. In this case we're
        // using a fixed delta of 1, but otherwise this would be the _change in
        // progress_ from the last iteration to this one.
        if progressbar.get_position() >= progressbar.get_length() && !progressbar.is_finished() {
            progressbar =
                progressbar.stop(format!("{} {}", style("✔").green(), "Copying files"))?;
        } else {
            progressbar.increment(thread_rng().gen_range(1..20));
        }

        if downloadbar.get_position() >= downloadbar.get_length() && !downloadbar.is_finished() {
            downloadbar =
                downloadbar.stop(format!("{} {}", style("✔").green(), "Downloading files"))?;
        } else {
            downloadbar.increment(thread_rng().gen_range(1..13));
        }
    }

    outro("Done!")?;

    Ok(())
}
