use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::interaction::{Event, PromptInteraction, State},
    theme::{ClackTheme, Theme},
};

#[derive(Default)]
pub struct Confirm {
    prompt: String,
    input: bool,
    initial_value: bool,
}

impl Confirm {
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            ..Default::default()
        }
    }

    pub fn initial_value(mut self, initial_value: bool) -> Self {
        self.initial_value = initial_value;
        self
    }

    pub fn interact(&mut self) -> io::Result<bool> {
        <Self as PromptInteraction<bool>>::interact(self)
    }
}

impl PromptInteraction<bool> for Confirm {
    fn on(&mut self, event: &Event) -> State<bool> {
        let Event::Key(key) = event;

        match key {
            Key::ArrowDown | Key::ArrowRight | Key::ArrowUp | Key::ArrowLeft => {
                self.input = !self.input;
            }
            Key::Char('y') | Key::Char('Y') => {
                self.input = true;
                return State::Submit(self.input);
            }
            Key::Char('n') | Key::Char('N') => {
                self.input = false;
                return State::Submit(self.input);
            }
            Key::Enter => return State::Submit(self.input),
            _ => {}
        }

        State::Active
    }

    fn render(&mut self, state: &State<bool>) -> String {
        let line1 = ClackTheme.format_header(&state.into(), &self.prompt);
        let line2 = ClackTheme.format_confirm(&state.into(), self.input);
        let line3 = ClackTheme.format_footer(&state.into());

        line1 + &line2 + &line3
    }
}
