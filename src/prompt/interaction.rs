use console::{Key, Term};
use std::io::{self, Write};

pub enum State<R> {
    Active,
    Submit(R),
    Cancel,
    Error(String),
}

#[derive(PartialEq, Eq)]
pub enum Event {
    Key(Key),
}

/// Wraps text to fit the terminal width.
fn wrap(text: &str, width: usize) -> String {
    use textwrap::{core::Word, fill, Options, WordSeparator};

    fill(
        text,
        Options::new(width).word_separator(
            // Workaround to prevent textwrap from splitting words by spaces
            // which breaks the layout of the prompt. Instead, we treat
            // each line as a single word which forces wrapping it hardly
            // at the end of the terminal width.
            WordSeparator::Custom(|line| Box::new(vec![Word::from(line)].into_iter())),
        ),
    )
}

pub trait PromptInteraction<R> {
    fn render(&mut self, state: &State<R>) -> String;

    fn on(&mut self, event: &Event) -> State<R>;

    fn interact(&mut self) -> io::Result<R> {
        self.interact_on(&mut Term::stderr())
    }

    fn interact_on(&mut self, term: &mut Term) -> io::Result<R> {
        if !term.is_term() {
            return Err(io::ErrorKind::NotConnected.into());
        }

        term.hide_cursor()?;

        let mut state = State::Active;
        let mut prev_frame = String::new();

        loop {
            let frame = self.render(&state);

            if frame != prev_frame {
                let prev_frame_check = wrap(&prev_frame, term.size().1 as usize);

                term.clear_last_lines(prev_frame_check.lines().count())?;
                term.write_all(frame.as_bytes())?;
                term.flush()?;

                prev_frame = frame;
            }

            if let State::Submit(result) = state {
                return Ok(result);
            }

            if let State::Cancel = state {
                return Err(io::ErrorKind::Interrupted.into());
            }

            match term.read_key()? {
                Key::Escape => {
                    state = State::Cancel;
                }
                other => {
                    state = self.on(&Event::Key(other));
                }
            }
        }
    }
}
