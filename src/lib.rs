use std::fs::File;
use std::io::{stdin, BufReader, Read};
use std::path::PathBuf;

use clap::Parser;
use is_terminal::IsTerminal as _;
use log::{debug, info};

use crate::args::{Args, TargetFormat};
use crate::errors::ProconError;
use crate::json_file_reader::JsonFileReader;
use crate::nodes::Nodes;
use crate::nodes_writer::{to_json, to_properties, to_yaml};
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

pub fn run() -> Result<String, ProconError> {
    let args: Args = parse_args_and_setup_logger()?;

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    debug!("Setup logger");

    validate_args(&args)?;

    if stdin().is_terminal() {
        debug!("User: terminal");
    }

    let nodes = parse_input_file(&args)?;
    convert_nodes(&args, &nodes)
}

fn parse_args_and_setup_logger() -> Result<Args, ProconError> {
    let args = Args::parse();
    debug!("{:?}", args);
    Ok(args)
}

fn validate_args(args: &Args) -> Result<(), ProconError> {
    if args.dry_run && args.output_filename.is_some() {
        return Err(ProconError {
            message: "Option -d and -o are mutual exclusive".to_string(),
        });
    }
    Ok(())
}

pub fn parse_input_file(args: &Args) -> Result<Nodes, ProconError> {
    debug!("\n####################################\nLoad property files\n####################################");
    let content: String = read_file_or_stdin(&args)?;
    return if args.target_format.path_buf() == &PathBuf::from("-") {
        try_reader_from_flag_or_all_sequential(&args, &content)
    } else {
        find_parser_via_extension(&args, &content)
    };
}

fn read_file_or_stdin(args: &Args) -> Result<String, ProconError> {
    let mut content = String::new();
    let count;

    // todo I guess this should work with generics somehow
    let path_buf = args.target_format.path_buf();
    if path_buf == &PathBuf::from("-") {
        if stdin().is_terminal() {
            return Err(ProconError {
                message: "Wrong use of pipe".to_string(),
            });
        }
        let mut buffer = BufReader::new(stdin().lock());
        count = buffer.read_to_string(&mut content);
    } else {
        let file = File::open(&path_buf).map_err(|_| ProconError {
            message: "Unable to read file".to_string(),
        })?;
        let mut buffer = BufReader::new(file);
        count = buffer.read_to_string(&mut content);
    }

    debug!("Read {:?} bytes", count);
    Ok(content)
}

fn try_reader_from_flag_or_all_sequential(
    args: &Args,
    content: &String,
) -> Result<Nodes, ProconError> {
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

fn try_all_readers(args: &Args, content: &String) -> Result<Nodes, ProconError> {
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

fn find_parser_via_extension(args: &Args, content: &String) -> Result<Nodes, ProconError> {
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
        &_ => Err(ProconError {
            message: "Not supported file type:\n\t*.properties\n\t*.json\n\t*.yaml".to_string(),
        }),
    }?;

    info!("Read {}", &args.target_format.path_buf().to_str().unwrap());
    Ok(nodes)
}

fn convert_nodes(args: &Args, nodes: &Nodes) -> Result<String, ProconError> {
    debug!("\n####################################\nStart format conversion\n####################################");
    match args.target_format {
        TargetFormat::Properties { .. } => {
            debug!("Convert to properties");
            to_properties(&args, &nodes)
        }
        TargetFormat::Json { .. } => {
            debug!("Convert to json");
            to_json(&args, &nodes)
        }
        TargetFormat::Yaml { .. } => {
            debug!("Convert to yaml");
            to_yaml(&args, &nodes)
        }
    }
}
