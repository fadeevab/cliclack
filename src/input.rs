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

#[derive(Default)]
struct MultilineEditing {
    enabled: bool,
    editing: bool,
}

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
    multiline: MultilineEditing,
    validate_on_enter: Option<ValidationCallback>,
    validate_interactively: Option<ValidationCallback>,
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

    /// Enables the multiline input.
    ///
    /// The user should press `Tab` to switch between the `edit` and `view` mode.
    ///
    /// In the edit mode, the user can input multiple lines of text.
    ///
    /// In the view mode, the user can press `Enter` to submit the input.
    pub fn multiline(mut self) -> Self {
        self.multiline.enabled = true;
        self.multiline.editing = true;
        self
    }

    /// Sets a validation callback for the input that is called when the user submits.
    /// The same as [`Input::validate_on_enter`].
    pub fn validate<V>(mut self, validator: V) -> Self
    where
        V: Validate<String> + 'static,
        V::Err: ToString,
    {
        self.validate_on_enter = Some(Box::new(move |input: &String| {
            validator.validate(input).map_err(|err| err.to_string())
        }));
        self
    }

    /// Sets a validation callback for the input that is called when the user submits.
    pub fn validate_on_enter<V>(self, validator: V) -> Self
    where
        V: Validate<String> + 'static,
        V::Err: ToString,
    {
        self.validate(validator)
    }

    /// Validates input while user is typing.
    pub fn validate_interactively<V>(mut self, validator: V) -> Self
    where
        V: Validate<String> + 'static,
        V::Err: ToString,
    {
        self.validate_interactively = Some(Box::new(move |input: &String| {
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

        if self.multiline.enabled {
            match key {
                Key::Tab => {
                    self.multiline.editing = !self.multiline.editing;
                    return State::Active;
                }
                Key::Enter => {
                    if self.multiline.editing {
                        self.input.insert('\n');
                        return State::Active;
                    }
                }
                _ => {
                    if !self.multiline.editing {
                        return State::Active;
                    }
                }
            }
        }

        if *key == Key::Enter && self.input.is_empty() {
            if let Some(default) = &self.default {
                self.input.extend(default);
            } else if self.input_required {
                return State::Error("Input required".to_string());
            }
        }

        // Cannot move this upper: If moved upper, when the input is empty
        // and default is allowed, the default value may never be set, validation will fail.
        // Cannot remove this although validated before: if removed, when editing is false,
        // the validation will not be checked.
        if let Some(validator) = &self.validate_interactively {
            if let Err(err) = validator(&self.input.to_string()) {
                return State::Error(err);
            }

            if self.input.to_string().parse::<T>().is_err() {
                return State::Error("Invalid value format".to_string());
            }
        }

        if *key == Key::Enter {
            if let Some(validator) = &self.validate_on_enter {
                if let Err(err) = validator(&self.input.to_string()) {
                    return State::Error(err);
                }
            }

            match self.input.to_string().parse::<T>() {
                Ok(value) => return State::Submit(value),
                Err(_) => return State::Error("Invalid value format".to_string()),
            }
        }

        State::Active
    }

    fn render(&mut self, state: &State<T>) -> String {
        let theme = THEME.lock().unwrap();

        let part1 = theme.format_header(&state.into(), &self.prompt);
        let part2 = if self.input.is_empty() {
            theme.format_placeholder(&state.into(), &self.placeholder)
        } else {
            theme.format_input(&state.into(), &self.input)
        };
        let part3 = theme.format_footer_with_message(
            &state.into(),
            match self.multiline.editing {
                true if self.multiline.enabled => "[Tab] => View",
                false if self.multiline.enabled => "[Tab] => Edit | Enter => Submit",
                _ => "",
            },
        );

        part1 + &part2 + &part3
    }
}
