//! The `errors` module defines custom error types for TiO2.
use std::{error::Error, fmt};

/// Represents an unexpected end of file error.
#[derive(Debug, Clone)]
pub struct UnexpectedEOFError {
    /// The expected token that should have followed the unexpected end of file.
    pub token: String,
}

impl fmt::Display for UnexpectedEOFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected end of file. Expected token following {}",
            self.token
        )
    }
}

impl Error for UnexpectedEOFError {}

impl UnexpectedEOFError {
    /// Creates a new instance of `UnexpectedEOFError`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tio2::errors::UnexpectedEOFError;
    ///
    /// let error = UnexpectedEOFError::new("Expected token: {}");
    /// ```
    pub fn new(token: impl ToString) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}
