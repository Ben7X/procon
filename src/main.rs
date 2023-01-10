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
use crate::property_file_reader::{Delimiter, PropertyFileReader};
use clap::Parser;
use log::{debug, error, trace};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::fs::File;
use std::path::Path;
use std::process;

#[cfg(test)]
#[path = "./main_test.rs"]
mod main;

fn main() {
    let args = Args::parse();

    // logger
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(args.log_level))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    debug!("{:?}", args);

    trace!("\n####################################\nLoad property files\n####################################");
    let nodes: Nodes = load_file_to_nodes(&args.filename, &args.delimiter);

    trace!("\n####################################\nStart format conversion\n####################################");
    run(&args.command, &args.filename, &nodes).unwrap_or_else(|err| {
        error!("{}", err.to_string());
        process::exit(exitcode::DATAERR);
    });
}

fn run(command: &Command, filename: &str, nodes: &Nodes) -> Result<String, std::io::Error> {
    let output_filename = &output_filename(command, filename);

    match command {
        Command::Properties => {
            trace!("Converty yaml to property");
            to_properties(output_filename, nodes);
        }
        Command::Yaml => {
            trace!("Converting property file to yaml");
            to_yaml(output_filename, &nodes);
        }
        Command::Json => {
            trace!("Converting property file to yaml");
            to_json(output_filename, &nodes);
        }
    }

    Ok(String::from("Done"))
}

fn load_file_to_nodes(filename: &str, delimiter: &Delimiter) -> Nodes {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            error!("{} {}", filename, err.to_string());
            process::exit(exitcode::CONFIG);
        }
    };

    let extension: &str = Path::new(filename).extension().unwrap().to_str().unwrap();
    match extension.to_lowercase().as_str() {
        "properties" => PropertyFileReader::parse(&file, filename, delimiter).unwrap(),
        "yml" => {
            panic!("From yaml conversion not implemented yet")
        }
        "yaml" => {
            panic!("From yaml conversion not implemented yet")
        }
        "json" => JsonFileReader::parse(filename).unwrap(),
        &_ => {
            panic!("Not supported file")
        }
    }
}

fn output_filename(command: &Command, filename: &str) -> String {
    let filename = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    return [filename, ".", command.to_string().to_lowercase().as_str()].concat();
}
