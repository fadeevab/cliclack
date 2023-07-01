use std::fmt::Debug;
use std::io;
use std::{fmt::Display, str::FromStr};

use console::Key;

use crate::{
    prompt::{
        cursor::StringCursor,
        interaction::{Event, PromptInteraction, State},
    },
    theme::{ClackTheme, Theme},
    validate::Validate,
};

type ValidationCallback = Box<dyn Fn(&String) -> Result<(), String>>;

#[derive(Default)]
pub struct Text {
    prompt: String,
    placeholder: StringCursor,
    input: StringCursor,
    validate: Option<ValidationCallback>,
}

impl Text {
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            ..Default::default()
        }
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder.extend(placeholder);
        self
    }

    pub fn validate<V>(mut self, validator: V) -> Self
    where
        V: Validate<String> + 'static,
        V::Err: ToString,
    {
        self.validate = Some(Box::new(move |input: &String| {
            validator.validate(input).map_err(|err| err.to_string())
        }));
        self
    }

    pub fn interact<T>(&mut self) -> io::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        <Self as PromptInteraction<T>>::interact(self)
    }
}

impl<T> PromptInteraction<T> for Text
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    fn input(&mut self) -> Option<&mut StringCursor> {
        Some(&mut self.input)
    }

    fn on(&mut self, event: &Event) -> State {
        let Event::Key(key) = event;

        if *key == Key::Enter {
            if let Some(validator) = &self.validate {
                if let Err(err) = validator(&self.input.to_string()) {
                    return State::Error(err);
                }
            }
            return State::Submit(self.input.to_string());
        }

        State::Active
    }

    fn render(&mut self, state: &State) -> String {
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
