use std::io;
use std::{fmt::Display, str::FromStr};

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

/// A prompt that accepts a single line of text input.
///
/// # Example
///
/// ```
/// use cliclack::Input;
///
/// # fn test() -> std::io::Result<()> {
/// let input: String = Input::new("Tea or coffee?")
///     .placeholder("Yes")
///     .interact()?;
/// # Ok(())
/// # }
/// # test().ok();
/// ```
#[derive(Default)]
pub struct Input {
    prompt: String,
    input: StringCursor,
    input_required: bool,
    default: Option<String>,
    placeholder: StringCursor,
    validate: Option<ValidationCallback>,
}

impl Input {
    /// Creates a new input prompt.
    pub fn new(prompt: impl Display) -> Self {
        Self {
            prompt: prompt.to_string(),
            input_required: true,
            ..Default::default()
        }
    }

    /// Sets the placeholder (hint) text for the input.
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder.extend(placeholder);
        self
    }

    /// Sets the default value for the input and also a hint (placeholder) if one is not already set.
    ///
    /// [`Input::placeholder`] overrides a hint set by `default()`, however, default value
    /// is used is no value has been supplied.
    pub fn default_input(mut self, value: &str) -> Self {
        self.default = Some(value.into());
        self
    }

    /// Sets whether the input is required. Default: `true`.
    ///
    /// [`Input::default_input`] is used if no value is supplied.
    pub fn required(mut self, required: bool) -> Self {
        self.input_required = required;
        self
    }

    /// Sets a validation callback for the input.
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
    pub fn interact<T>(&mut self) -> io::Result<T>
    where
        T: FromStr,
    {
        if self.placeholder.is_empty() {
            if let Some(default) = &self.default {
                self.placeholder.extend(default);
                self.placeholder.extend(" (default)");
            }
        }
        <Self as PromptInteraction<T>>::interact(self)
    }
}

impl<T> PromptInteraction<T> for Input
where
    T: FromStr,
{
    fn input(&mut self) -> Option<&mut StringCursor> {
        Some(&mut self.input)
    }

    fn on(&mut self, event: &Event) -> State<T> {
        let Event::Key(key) = event;

        if *key == Key::Enter && self.input.is_empty() {
            if let Some(default) = &self.default {
                self.input.extend(default);
            } else if self.input_required {
                return State::Error("Input required".to_string());
            }
        }

        if let Some(validator) = &self.validate {
            if let Err(err) = validator(&self.input.to_string()) {
                return State::Error(err);
            }
        }

        match self.input.to_string().parse::<T>() {
            Ok(value) if *key == Key::Enter => return State::Submit(value),
            Err(_) => return State::Error("Invalid value format".to_string()),
            _ => {}
        }

        State::Active
    }

    fn render(&mut self, state: &State<T>) -> String {
        let theme = THEME.lock().unwrap();

        let line1 = theme.format_header(&state.into(), &self.prompt);
        let line2 = if self.input.is_empty() {
            theme.format_placeholder(&state.into(), &self.placeholder)
        } else {
            theme.format_input(&state.into(), &self.input)
        };
        let line3 = theme.format_footer(&state.into());

        line1 + &line2 + &line3
    }
}
