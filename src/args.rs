use std::fmt::Display;

use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::property_file_reader::Delimiter;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Procon (Pro)perty (Con)verter \
    \nA programm to convert between different property formats.
    \nProperty -> Json
    \nProperty -> Yaml
    \nJson -> Property
    \nJson -> Yaml
    \nYaml -> Property *not yet implemented
    \nYaml-> Yaml *not yet implemented
    "
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Dry run
    ///
    /// Prints the converted format to the console
    /// This option is mutual exclusive with the --output-filename option.
    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,

    /// Log level of the program
    #[arg(short, long, default_value_t = LevelFilter::Info)]
    pub log_level: LevelFilter,

    /// File to write the converted format to the console
    ///
    /// This option is mutual exclusive with the --dry-run option.
    #[arg(short, long)]
    pub output_filename: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Property format to convert to: Properties file
    Properties {
        /// Property delimiter
        ///
        /// only used in combination with properties command
        #[arg(short, long, default_value_t = Delimiter::Equals)]
        property_delimiter: Delimiter,

        /// Path of the file to convert
        filename: String,
    },

    /// Property format to convert to: Json
    Json {
        /// Path of the file to convert
        filename: String,
    },

    /// Property format to convert to: Yaml
    Yaml {
        /// Path of the file to convert
        filename: String,
    },
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
impl Command {
    pub fn filename(&self) -> String {
        match self {
            Command::Properties { filename, .. } => filename.to_string(),
            Command::Json { filename, .. } => filename.to_string(),
            Command::Yaml { filename, .. } => filename.to_string(),
        }
    }
    pub fn delimiter(&self) -> Option<&Delimiter> {
        match self {
            Command::Properties {
                property_delimiter, ..
            } => Some(property_delimiter),
            Command::Json { .. } => None,
            Command::Yaml { .. } => None,
        }
    }
}
