use std::fs::File;
use std::io::{stdin, BufReader, Read};
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;
use is_terminal::IsTerminal as _;
use log::{debug, info, trace};

use crate::args::{Args, TargetFormat};
use crate::errors::ProconError;
use crate::json_file_reader::JsonFileReader;
use crate::nodes::Nodes;
use crate::nodes_writer::{to_json, to_properties, to_toml, to_yaml};
use crate::property_file_reader::PropertyFileReader;
use crate::yaml_file_reader::YamlFileReader;

pub mod args;
pub mod errors;
pub mod json_file_reader;
pub mod line;
pub mod node;
pub mod nodes;
pub mod nodes_writer;
pub mod nodes_writer_test;
pub mod property_file_reader;
pub mod yaml_file_reader;

pub fn run() -> Result<String> {
    let args: Args = parse_args_and_setup_logger()?;

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    debug!("Setup logger");

    if stdin().is_terminal() {
        debug!("User: terminal");
    }

    let nodes = parse_input_file(&args)?;
    convert_nodes(&args, &nodes)
}

fn parse_args_and_setup_logger() -> Result<Args> {
    let args = Args::parse();
    debug!("{:?}", args);
    Ok(args)
}

pub fn parse_input_file(args: &Args) -> Result<Nodes> {
    debug!("Load property files");
    let content: String = read_file_or_stdin(&args)?;
    return if args.target_format.path_buf() == &PathBuf::from("-") {
        try_reader_from_flag_or_all_sequential(&args, &content)
    } else {
        find_parser_via_extension(&args, &content)
    };
}

fn read_file_or_stdin(args: &Args) -> Result<String> {
    let mut content = String::new();
    let count;

    // todo I guess this should work with generics somehow
    let path_buf = args.target_format.path_buf();
    if path_buf == &PathBuf::from("-") {
        if stdin().is_terminal() {
            bail!(ProconError {
                message: "Nothing piped into stdin".to_string(),
            });
        }
        let mut buffer = BufReader::new(stdin().lock());
        count = buffer.read_to_string(&mut content);
    } else {
        let file = File::open(&path_buf)?;
        let mut buffer = BufReader::new(file);
        count = buffer.read_to_string(&mut content);
    }

    trace!("Read {:?} bytes", count);
    Ok(content)
}

fn try_reader_from_flag_or_all_sequential(args: &Args, content: &String) -> Result<Nodes> {
    if args.from_property_file {
        return PropertyFileReader::parse(&args, &content);
    }
    if args.from_json_file {
        return JsonFileReader::parse(&args, &content);
    }
    if args.from_yaml_file {
        return YamlFileReader::parse(&args, &content);
    }
    try_all_readers(&args, &content)
}

fn try_all_readers(args: &Args, content: &String) -> Result<Nodes> {
    info!("Guess input file");
    let json_nodes = JsonFileReader::parse(&args, &content);
    if json_nodes.is_ok() {
        return json_nodes;
    }
    let yaml_nodes = YamlFileReader::parse(&args, &content);
    if yaml_nodes.is_ok() {
        return yaml_nodes;
    }
    let result_nodes = PropertyFileReader::parse(&args, &content);
    if result_nodes.is_ok() {
        return result_nodes;
    }
    info!("No suitable reader found");
    Ok(Nodes::new())
}

fn find_parser_via_extension(args: &Args, content: &String) -> Result<Nodes> {
    let extension: &str = &args
        .target_format
        .path_buf()
        .extension()
        .unwrap()
        .to_str()
        .unwrap();

    let nodes = match extension.to_lowercase().as_str() {
        "properties" => PropertyFileReader::parse(&args, &content),
        "yml" => YamlFileReader::parse(&args, &content),
        "yaml" => YamlFileReader::parse(&args, &content),
        "json" => JsonFileReader::parse(&args, &content),
        &_ => bail!(ProconError {
            message: "Not supported file type:\n\t*.properties\n\t*.json\n\t*.yaml".to_string(),
        }),
    }?;

    info!("Read {}", &args.target_format.path_buf().to_str().unwrap());
    Ok(nodes)
}

fn convert_nodes(args: &Args, nodes: &Nodes) -> Result<String> {
    debug!("Start nodes conversion");
    match args.target_format {
        TargetFormat::Properties { .. } => to_properties(&args, &nodes),
        TargetFormat::Json { .. } => to_json(&args, &nodes),
        TargetFormat::Yaml { .. } => to_yaml(&args, &nodes),
        TargetFormat::Toml { .. } => to_toml(&args, &nodes),
    }
}
