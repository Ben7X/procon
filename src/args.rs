use crate::command::Command;
use crate::property_file_reader::Delimiter;
use clap::Parser;
use log::LevelFilter;

/// Program to convert between different property formats
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// file to convert
    pub filename: String,

    /// Supported conversions Properties, Yaml, Json
    pub command: Command,

    /// delimiter
    #[arg(short, long, default_value_t = Delimiter::Equals)]
    pub delimiter: Delimiter,

    /// create debug logs
    #[arg(short, long, default_value_t = LevelFilter::Info)]
    pub log_level: LevelFilter,
}
