use std::fmt::Display;
use std::io;

use console::Key;

use crate::{
    prompt::{
        cursor::StringCursor,
        interaction::{Event, PromptInteraction, State},
    },
    theme::THEME,
    validate::Validate,
};

type ValidationCallback = Box<dyn Fn(&String) -> Result<(), String>>;

/// A prompt that masks the input.
#[derive(Default)]
pub struct Password {
    prompt: String,
    mask: char,
    input: StringCursor,
    validate: Option<ValidationCallback>,
}

impl Password {
    /// Creates a new password prompt.
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            mask: THEME.lock().unwrap().password_mask(),
            ..Default::default()
        }
    }

    /// Sets the mask character. E.g. `*` or `•`.
    pub fn mask(mut self, mask: char) -> Self {
        self.mask = mask;
        self
    }

    /// Sets the validation callback.
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

    /// Starts the prompt interaction.
    pub fn interact(&mut self) -> io::Result<String> {
        <Self as PromptInteraction<String>>::interact(self)
    }
}

impl PromptInteraction<String> for Password {
    fn input(&mut self) -> Option<&mut StringCursor> {
        Some(&mut self.input)
    }

    fn allow_word_editing(&self) -> bool {
        // Disallow word editing for password prompts so as not to reveal
        // password structure.
        false
    }

    fn on(&mut self, event: &Event) -> State<String> {
        let Event::Key(key) = event;

        if *key == Key::Enter {
            if self.input.is_empty() {
                return State::Error("Input required".to_string());
            }

            if let Some(validator) = &self.validate {
                if let Err(err) = validator(&self.input.to_string()) {
                    return State::Error(err);
                }
            }
            return State::Submit(self.input.to_string());
        }

        State::Active
    }

    fn render(&mut self, state: &State<String>) -> String {
        let mut masked = self.input.clone();
        for chr in masked.iter_mut() {
            *chr = self.mask;
        }

        let theme = THEME.lock().unwrap();

        let line1 = theme.format_header(&state.into(), &self.prompt);
        let line2 = theme.format_input(&state.into(), &masked);
        let line3 = theme.format_footer(&state.into());

        line1 + &line2 + &line3
    }
}
