use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub struct ProconError {
    pub message: String,
}

impl Display for ProconError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
