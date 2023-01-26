use std::fmt::{Display, Formatter};

use thiserror::Error;

#[derive(Error, Debug)]
pub struct ProconError {
    pub message: String,
}

impl Display for ProconError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
