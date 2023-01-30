use std::fmt::Display;
use std::path::PathBuf;

use clap::ArgGroup;
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
    \nExamples:
    \nProperty -> Json
    \n\tprocon json example.properties
    \nProperty -> Yaml
    \n\tprocon yaml example.properties
    \nJson -> Properties
    \n\tprocon properties example.json
    "
)]
#[command(propagate_version = true)]
#[command(group(ArgGroup::new("from")
.multiple(false)
.args(["from_property_file", "from_yaml_file", "from_json_file"]),
))]
#[command(group(ArgGroup::new("dry-run")
.multiple(false)
.args(["dry_run", "output_filename"]),
))]
pub struct Args {
    #[command(subcommand)]
    pub target_format: TargetFormat,

    /// Dry run
    ///
    /// Only prints the converted format to the console
    ///
    /// This option is mutual exclusive with the -o --output-filename option
    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,

    /// Flag to specifying stdin bytes to be processed as properties
    ///
    /// Format of stdin bytes
    #[arg(short = 'p', long)]
    pub from_property_file: bool,

    /// Flag to specifying stdin bytes to be processed as yaml
    ///
    /// Format of stdin bytes
    #[arg(short = 'y', long)]
    pub from_yaml_file: bool,

    /// Flag to specifying stdin bytes to be processed as json
    ///
    /// Format of stdin bytes
    #[arg(short = 'j', long)]
    pub from_json_file: bool,

    /// File to write the converted format to
    ///
    /// This option is mutual exclusive with the -d --dry-run option.
    #[arg(short, long)]
    pub output_filename: Option<String>,

    #[clap(flatten)]
    pub verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum TargetFormat {
    /// Target format properties
    Properties {
        /// Property delimiter
        ///
        /// only used in combination with properties command
        #[arg(short, long, default_value_t = Delimiter::Equals)]
        property_delimiter: Delimiter,
        /// Input file
        file: PathBuf,
    },

    /// Target format yaml
    Yaml {
        /// Property delimiter
        ///
        /// only used in combination with properties command
        #[arg(short, long, default_value_t = Delimiter::Equals)]
        property_delimiter: Delimiter,

        /// Input file
        file: PathBuf,
    },

    /// Target format json
    Json {
        /// Property delimiter
        ///
        /// only used in combination with properties command
        #[arg(short, long, default_value_t = Delimiter::Equals)]
        property_delimiter: Delimiter,

        /// Input file
        file: PathBuf,
    },

    /// Target format toml
    Toml {
        /// Property delimiter
        ///
        /// only used in combination with properties command
        #[arg(short, long, default_value_t = Delimiter::Equals)]
        property_delimiter: Delimiter,

        /// Input file
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
            TargetFormat::Toml { file, .. } => file,
        }
    }
    pub fn delimiter(&self) -> Option<&Delimiter> {
        // todo I guess there is a better way to handle this
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
            TargetFormat::Toml {
                property_delimiter, ..
            } => Some(property_delimiter),
        }
    }
}
