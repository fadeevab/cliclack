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
///
/// Multiline editing is also supported ([`Input::multiline`]).
/// Press `ESC` to switch from the `edit` to `view` mode.
///
/// In `view` mode, press `Enter` to submit the input,
/// and other keys to switch back to `edit` mode.
///
/// # Example
///
/// ```
/// use cliclack::input;
/// # fn test() -> std::io::Result<()> {
/// let path: String = input("Input multiple lines: ")
///     .multiline()
///     .interact()?;
/// # Ok(())
/// # }
/// # test().ok(); // Ignoring I/O runtime errors.
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
                if self.multiline.enabled {
                    self.multiline.editing = false;
                }
            }
        }
        <Self as PromptInteraction<T>>::interact(self)
    }

    /// Interactively validate the input value.
    fn interactively_validate<T: FromStr>(&self) -> State<T> {
        if let Some(validator) = &self.validate_interactively {
            if let Err(err) = validator(&self.input.to_string()) {
                return State::Error(err);
            }

            if self.input.to_string().parse::<T>().is_err() {
                return State::Error("Invalid value format".to_string());
            }
        }
        State::Active
    }

    /// Submit the input value.
    /// - If the input is empty, use the default value if it exists.
    /// - If the input is empty and no default value is set, return an error.
    /// - If validation fails or recving invalid format, switch to editing mode.
    fn submit_validate<T: FromStr>(&mut self) -> State<T> {
        if self.input.is_empty() {
            if let Some(default) = &self.default {
                self.input.extend(default);
            } else if self.input_required {
                return State::Error("Input is required".to_string());
            }
        }
        if !self.multiline.editing {
            match self.interactively_validate() {
                State::Active => {}
                state => return state,
            }
        }
        if let Some(validator) = &self.validate_on_enter {
            if let Err(err) = validator(&self.input.to_string()) {
                self.switch_mode::<T>();
                return State::Error(err);
            }
        }
        match self.input.to_string().parse::<T>() {
            Ok(res) => State::Submit(res),
            Err(_) => {
                self.switch_mode::<T>();
                State::Error("Invalid value format".to_string())
            }
        }
    }

    /// Try mode switching.
    /// - Switch to view mode if passed interactively validation .
    /// - Switch to edit mode if in view mode.
    fn switch_mode<T: FromStr>(&mut self) -> State<T> {
        if self.multiline.editing {
            if let State::Error(_) = self.interactively_validate::<T>() {
                // If interactive validation failed and key == Escap,
                // activate State::Cancel
                return State::Active;
            }
        }
        self.multiline.editing = !self.multiline.editing;
        // Only Escape cares this return value.
        // In this context, Active means to activate State::Cancel,
        // Cancel means to cancel State::Cancel.
        if self.multiline.editing {
            State::Active
        } else {
            State::Cancel
        }
    }
}

impl<T> PromptInteraction<T> for Input
where
    T: FromStr,
{
    fn input(&mut self) -> Option<&mut StringCursor> {
        if self.multiline.enabled && !self.multiline.editing {
            return None;
        }
        Some(&mut self.input)
    }

    fn on(&mut self, event: &Event) -> State<T> {
        let Event::Key(key) = event;

        match *key {
            Key::Escape => {
                if !self.multiline.enabled | !self.multiline.editing {
                    // When key == Escape, activate State::Cancel
                    return State::Active;
                }
                self.switch_mode()
            }
            Key::Tab => {
                if !self.multiline.enabled | self.multiline.editing {
                    for _ in 0..4 {
                        self.input.insert(' ');
                    }
                }
                State::Active
            }
            Key::Enter => {
                if self.multiline.enabled && self.multiline.editing {
                    self.input.insert('\n');
                    return State::Active;
                }
                // if not, gonna submit
                self.submit_validate()
            }
            Key::Char(c) if !c.is_ascii_control() => {
                if !self.multiline.editing && self.multiline.enabled {
                    self.switch_mode::<T>();
                    self.input.insert(c);
                }
                // The char has been inserted, so we need to interactively validate it if need
                self.interactively_validate()
            }
            Key::Backspace => {
                if !self.multiline.editing && self.multiline.enabled {
                    self.switch_mode::<T>();
                    self.input.delete_left();
                }
                self.interactively_validate()
            }
            _ => State::Active,
        }
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
                true if self.multiline.enabled => "[ESC] => View",
                false if self.multiline.enabled => "[Enter] => Submit",
                _ => "",
            },
        );

        part1 + &part2 + &part3
    }
}
