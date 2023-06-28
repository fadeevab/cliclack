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

pub trait PromptInteraction<R> {
    fn render(&mut self, state: &State<R>) -> String;

    fn notify(&mut self, event: &Event) -> State<R>;

    fn interact(&mut self) -> io::Result<R> {
        self.interact_on(&mut Term::stderr())
    }

    fn interact_on(&mut self, term: &mut Term) -> io::Result<R> {
        if !term.is_term() {
            return Err(io::ErrorKind::NotConnected.into());
        }

        let mut state = State::Active;
        let mut prev_frame = String::new();

        loop {
            let frame = self.render(&state);

            if frame != prev_frame {
                // TODO: clear only the lines that have changed
                term.clear_last_lines(prev_frame.lines().count())?;
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
                    state = self.notify(&Event::Key(other));
                }
            }
        }
    }
}
