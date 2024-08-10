//! Error types for CliClack.

use std::io;

use thiserror::Error;

/// Error type for CLI prompts.
#[derive(Debug, Error)]
pub enum CliError {
    /// I/O error.
    #[error("i/o error: {0}")]
    Io(io::Error),
    /// Error parsing a string to a target type.
    #[error("parse error: {0}")]
    Parse(String),
    /// Validation error.
    #[error("validation error: {0}")]
    Validate(String),
}

impl Into<io::Error> for CliError {
    fn into(self) -> io::Error {
        match self {
            CliError::Io(err) => err,
            CliError::Parse(msg) => io::Error::new(io::ErrorKind::InvalidInput, msg),
            CliError::Validate(msg) => io::Error::new(io::ErrorKind::InvalidInput, msg),
        }
    }
}