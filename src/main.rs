extern crate exitcode;
mod args;
mod command;
mod json_file_reader;
mod line;
mod node;
mod nodes;
mod nodes_converter;
mod property_file_reader;

use crate::args::Args;
use crate::command::Command;
use crate::json_file_reader::JsonFileReader;
use crate::nodes::Nodes;
use crate::nodes_converter::{to_json, to_properties, to_yaml};
use crate::property_file_reader::PropertyFileReader;
use clap::Parser;
use log::{debug, error, trace, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::path::Path;
use std::process;
use std::string::String;

#[cfg(test)]
#[path = "./main_test.rs"]
mod main;

fn main() {
    let args = Args::parse();

    setup_logger(args.log_level);
    debug!("{:?}", args);

    debug!("\n####################################\nLoad property files\n####################################");
    let nodes: Nodes = load_file_to_nodes(&args).unwrap_or_else(|err| {
        error!("{}", err.to_string());
        process::exit(exitcode::CONFIG);
    });

    debug!("\n####################################\nStart format conversion\n####################################");
    convert_nodes(&args, &nodes).unwrap_or_else(|err| {
        error!("{}", err.to_string());
        process::exit(exitcode::DATAERR);
    });
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
    let extension: &str = Path::new(&args.filename)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();

    let output_filename = output_filename(&args.command, &args.filename);
    match extension.to_lowercase().as_str() {
        "properties" => Ok(PropertyFileReader::parse(&args, output_filename).unwrap()),
        "yml" => Err("From yaml conversion not implemented yet"),
        "yaml" => Err("From yaml conversion not implemented yet"),
        "json" => Ok(JsonFileReader::parse(&args, output_filename).unwrap()),
        &_ => Err("Not supported file"),
    }
}

fn output_filename(command: &Command, filename: &str) -> String {
    let filename = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    return [filename, ".", command.to_string().to_lowercase().as_str()].concat();
}

fn convert_nodes(args: &Args, nodes: &Nodes) -> Result<&'static str, std::io::Error> {
    match args.command {
        Command::Properties => {
            trace!("Converty yaml to property");
            to_properties(nodes);
        }
        Command::Yaml => {
            trace!("Converting property file to yaml");
            to_yaml(&nodes);
        }
        Command::Json => {
            trace!("Converting property file to yaml");
            to_json(&nodes);
        }
    }

    Ok("Done")
}
