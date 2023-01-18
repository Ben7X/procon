use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub struct ConfigFileError {
    pub message: String,
}

impl Display for ConfigFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
