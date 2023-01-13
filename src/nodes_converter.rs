use std::fs::File;
use std::io::Write;
use std::path::Path;

use linked_hash_map::LinkedHashMap;
use log::debug;
use yaml_rust::{Yaml, YamlEmitter};

use crate::args::{Args, Command};
use crate::nodes::Nodes;

#[cfg(test)]
#[path = "./nodes_converter_test.rs"]
mod nodes_converter_test;

pub fn to_yaml(args: &Args, nodes: &Nodes) {
    let mut map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    for node in nodes.iter() {
        map.insert(Yaml::from_str(&node.name), node.into());
    }
    let mut content = String::new();
    let mut emitter = YamlEmitter::new(&mut content);
    let final_node = Yaml::Hash(map);
    emitter.dump(&final_node).unwrap();

    output_content(&args, content);
}

pub fn to_json(args: &Args, nodes: &Nodes) {
    let mut json_data = json::JsonValue::new_object();
    for node in nodes.iter() {
        json_data[node.name.clone()] = node.into();
    }
    output_content(&args, json_data.pretty(1));
}

pub fn to_properties(args: &Args, nodes: &Nodes) {
    let mut string_content = "".to_string();
    for node in nodes.iter() {
        let content: String = node.into();
        string_content.push_str(&content);
    }
    output_content(&args, string_content);
}

fn output_content(args: &Args, content: String) {
    if args.dry_run {
        println!("{}", content);
    } else {
        let output_filename = determine_output_filename(&args);
        let mut output_file: File = File::create(output_filename).unwrap();
        write!(output_file, "{}", content).unwrap();
    }
}

fn determine_output_filename(args: &Args) -> String {
    let output_filename: String;
    if args.output_filename.is_some() {
        output_filename = args.output_filename.as_ref().unwrap().to_string();
    } else {
        output_filename = default_filename(&args.command);
    }
    debug!("using output filename {}", output_filename);
    output_filename
}

fn default_filename(command: &Command) -> String {
    let (filename, extension) = match command {
        Command::Properties { filename, .. } => (filename, "properties".to_string()),
        Command::Json { filename, .. } => (filename, "json".to_string()),
        Command::Yaml { filename, .. } => (filename, "yaml".to_string()),
    };
    let filename = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    return [filename, ".", extension.to_string().to_lowercase().as_str()].concat();
}
