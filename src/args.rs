use std::fmt::Display;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use crate::property_file_reader::Delimiter;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Procon (Pro)perty (Con)verter \
    \nA program to convert between different property formats.
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
    pub target_format: TargetFormat,

    /// Dry run
    ///
    /// Prints the converted format to the console
    /// This option is mutual exclusive with the --output-filename option.
    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,

    /// Flag to specify pipe in bytes are in property file format
    ///
    /// If data piped in via stdin, what is the file format
    #[arg(short = 'p', long)]
    pub from_property_file: bool,

    /// Flag to specify pipe in bytes are in yaml file format
    ///
    /// If data piped in via stdin, what is the file format
    #[arg(short = 'y', long)]
    pub from_yaml_file: bool,

    /// Flag to specify pipe in bytes are in json file format
    ///
    /// If data piped in via stdin, what is the file format
    #[arg(short = 'j', long)]
    pub from_json_file: bool,

    /// File to write the converted format to the console
    ///
    /// This option is mutual exclusive with the --dry-run option.
    #[arg(short, long)]
    pub output_filename: Option<String>,

    #[clap(flatten)]
    pub verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum TargetFormat {
    /// Property format to convert to: Properties file
    Properties {
        /// Property delimiter
        ///
        /// only used in combination with properties command
        #[arg(short, long, default_value_t = Delimiter::Equals)]
        property_delimiter: Delimiter,
        /// Path of the file to convert
        file: PathBuf,
    },

    /// Property format to convert to: Json
    Json {
        /// Property delimiter
        ///
        /// only used in combination with properties command
        #[arg(short, long, default_value_t = Delimiter::Equals)]
        property_delimiter: Delimiter,

        /// Path of the file to convert
        file: PathBuf,
    },

    /// Property format to convert to: Yaml
    Yaml {
        /// Property delimiter
        ///
        /// only used in combination with properties command
        #[arg(short, long, default_value_t = Delimiter::Equals)]
        property_delimiter: Delimiter,

        /// Path of the file to convert
        file: PathBuf,
    },
}

impl Display for TargetFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
impl TargetFormat {
    pub fn path_buf(&self) -> &PathBuf {
        match self {
            TargetFormat::Properties { file, .. } => file,
            TargetFormat::Json { file, .. } => file,
            TargetFormat::Yaml { file, .. } => file,
        }
    }
    pub fn delimiter(&self) -> Option<&Delimiter> {
        match self {
            TargetFormat::Properties {
                property_delimiter, ..
            } => Some(property_delimiter),
            TargetFormat::Yaml {
                property_delimiter, ..
            } => Some(property_delimiter),
            TargetFormat::Json {
                property_delimiter, ..
            } => Some(property_delimiter),
        }
    }
}
