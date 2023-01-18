use std::path::Path;
use std::process;

use clap::Parser;
use log::{debug, trace, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

use crate::args::{Args, TargetFormat};
use crate::errors::ConfigFileError;
use crate::json_file_reader::JsonFileReader;
use crate::nodes::Nodes;
use crate::nodes_converter::{to_json, to_properties, to_yaml};
use crate::property_file_reader::PropertyFileReader;
use crate::yaml_file_reader::YamlFileReader;

pub mod args;
pub mod errors;
pub mod json_file_reader;
pub mod line;
pub mod node;
pub mod nodes;
pub mod nodes_converter;
pub mod property_file_reader;
pub mod yaml_file_reader;

pub fn run() -> Result<String, ConfigFileError> {
    let args: Args = parse_args_and_setup_logger();

    debug!("\n####################################\nLoad property files\n####################################");
    let filename = &args.target_format.filename();
    let extension: &str = Path::new(filename).extension().unwrap().to_str().unwrap();

    let nodes = match extension.to_lowercase().as_str() {
        "properties" => PropertyFileReader::parse(&args),
        "yml" => YamlFileReader::parse(&args),
        "yaml" => YamlFileReader::parse(&args),
        "json" => JsonFileReader::parse(&args),
        &_ => Err(ConfigFileError {
            error: "Not supported file type:\n\t*.properties\n\t*.json\n\t*.yaml".to_string(),
        }),
    }?;

    convert_nodes(&args, &nodes)
}

fn parse_args_and_setup_logger() -> Args {
    let args = Args::parse();

    setup_logger(args.log_level);
    debug!("{:?}", args);

    validate_args(&args);
    args
}

// todo propagate error
fn validate_args(args: &Args) {
    if args.dry_run && args.output_filename.is_some() {
        eprintln!("Option -d and -o are mutual exclusive. Consult the man page or use --help");
        process::exit(exitcode::CONFIG);
    }
}

fn setup_logger(log_level: LevelFilter) {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log_level))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
}

fn convert_nodes(args: &Args, nodes: &Nodes) -> Result<String, ConfigFileError> {
    debug!("\n####################################\nStart format conversion\n####################################");
    match args.target_format {
        TargetFormat::Properties { .. } => {
            trace!("Converty yaml to property");
            to_properties(&args, &nodes)
        }
        TargetFormat::Json { .. } => {
            trace!("Converting property file to yaml");
            to_json(&args, &nodes)
        }
        TargetFormat::Yaml { .. } => {
            trace!("Converting property file to yaml");
            to_yaml(&args, &nodes)
        }
    }
}
