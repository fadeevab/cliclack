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

pub struct Text {
    prompt: String,
    placeholder: StringCursor,
    input: StringCursor,
    validate: Option<ValidatorFn>,
}

impl Text {
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            placeholder: StringCursor::default(),
            input: StringCursor::default(),
            validate: None,
        }
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder.extend(placeholder);
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

impl PromptInteraction<String> for Text {
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
        let line1 = ClackTheme.format_header(&state.into(), &self.prompt);
        let line2 = if self.input.is_empty() {
            ClackTheme.format_placeholder(&state.into(), &self.placeholder)
        } else {
            ClackTheme.format_input(&state.into(), &self.input)
        };
        let line3 = ClackTheme.format_footer(&state.into());

        line1 + &line2 + &line3
    }
}
