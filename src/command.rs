use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Properties,
    Yaml,
    Json,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input.to_lowercase().as_str() {
            "properties" => Ok(Command::Properties),
            "yaml" => Ok(Command::Yaml),
            "yml" => Ok(Command::Yaml),
            "json" => Ok(Command::Json),
            &_ => Err(String::from(input.to_owned() + "Not a valid command")),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
