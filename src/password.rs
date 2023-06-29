use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::{
        cursor::StringCursor,
        interaction::{Event, PromptInteraction, State},
    },
    theme::{ClackTheme, Theme},
};

type ValidatorFn = Box<dyn Fn(&str) -> Result<(), String>>;

pub struct Password {
    prompt: String,
    input: StringCursor,
    mask: char,
    validate: Option<ValidatorFn>,
}

impl Password {
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            input: StringCursor::default(),
            mask: ClackTheme.password_mask(),
            validate: None,
        }
    }

    pub fn mask(mut self, mask: char) -> Self {
        self.mask = mask;
        self
    }

    pub fn validate<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        self.validate = Some(Box::new(validator));
        self
    }

    pub fn interact(&mut self) -> io::Result<String> {
        <Self as PromptInteraction<String>>::interact(self)
    }
}

impl PromptInteraction<String> for Password {
    fn on(&mut self, event: &Event) -> State<String> {
        match event {
            Event::Key(key) => match key {
                Key::Char(chr) if !chr.is_ascii_control() => {
                    self.input.insert(*chr);
                }
                Key::Backspace => {
                    self.input.delete_left();
                }
                Key::Del => {
                    self.input.delete_right();
                }
                Key::ArrowLeft => {
                    self.input.move_left();
                }
                Key::ArrowRight => {
                    self.input.move_right();
                }
                Key::Home => {
                    self.input.move_home();
                }
                Key::End => {
                    self.input.move_end();
                }
                Key::Enter => {
                    if let Some(validator) = &self.validate {
                        if let Err(err) = validator(&self.input.to_string()) {
                            return State::Error(err);
                        }
                    }
                    return State::Submit(self.input.to_string());
                }
                _ => {}
            },
        }

        State::Active
    }

    fn render(&mut self, state: &State<String>) -> String {
        let mut masked = self.input.clone();
        for chr in masked.iter_mut() {
            *chr = self.mask;
        }

        let line1 = ClackTheme.format_header(&state.into(), &self.prompt);
        let line2 = ClackTheme.format_input(&state.into(), &masked);
        let line3 = ClackTheme.format_footer(&state.into());

        line1 + &line2 + &line3
    }
}
