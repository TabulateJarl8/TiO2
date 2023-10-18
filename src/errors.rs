use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct UnexpectedEOFError {
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
    pub fn new(token: impl ToString) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}
