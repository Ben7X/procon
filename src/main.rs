extern crate exitcode;

use std::path::Path;
use std::process;

use clap::Parser;
use log::{debug, trace, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

use crate::args::{Args, TargetFormat};
use crate::json_file_reader::JsonFileReader;
use crate::nodes::Nodes;
use crate::nodes_converter::{to_json, to_properties, to_yaml};
use crate::property_file_reader::PropertyFileReader;
use crate::yaml_file_reader::YamlFileReader;

mod args;
mod json_file_reader;
mod line;
mod node;
mod nodes;
mod nodes_converter;
mod property_file_reader;
mod yaml_file_reader;

#[cfg(test)]
#[path = "./main_test.rs"]
mod main_test;
fn main() {
    let args = Args::parse();

    setup_logger(args.log_level);
    debug!("{:?}", args);

    validate_args(&args);

    debug!("\n####################################\nLoad property files\n####################################");
    let nodes: Nodes = load_file_to_nodes(&args).unwrap_or_else(|err| {
        eprintln!("{}", err.to_string());
        process::exit(exitcode::CONFIG);
    });

    debug!("\n####################################\nStart format conversion\n####################################");
    convert_nodes(&args, &nodes).unwrap_or_else(|err| {
        eprintln!("{}", err.to_string());
        process::exit(exitcode::DATAERR);
    });
}

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

fn load_file_to_nodes(args: &Args) -> Result<Nodes, &'static str> {
    let filename = &args.target_format.filename();
    let extension: &str = Path::new(filename).extension().unwrap().to_str().unwrap();

    match extension.to_lowercase().as_str() {
        "properties" => Ok(PropertyFileReader::parse(&args).unwrap()),
        "yml" => Ok(YamlFileReader::parse(&args).unwrap()),
        "yaml" => Ok(YamlFileReader::parse(&args).unwrap()),
        "json" => Ok(JsonFileReader::parse(&args).unwrap()),
        &_ => Err("Not supported file type. Properties, Json, Yaml"),
    }
}

fn convert_nodes(args: &Args, nodes: &Nodes) -> Result<&'static str, std::io::Error> {
    match args.target_format {
        TargetFormat::Properties { .. } => {
            trace!("Converty yaml to property");
            to_properties(&args, &nodes);
        }
        TargetFormat::Json { .. } => {
            trace!("Converting property file to yaml");
            to_json(&args, &nodes);
        }
        TargetFormat::Yaml { .. } => {
            trace!("Converting property file to yaml");
            to_yaml(&args, &nodes);
        }
    }

    Ok("Done")
}
