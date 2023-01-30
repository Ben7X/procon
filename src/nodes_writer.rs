use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use linked_hash_map::LinkedHashMap;
use log::debug;
use toml::Value;
use yaml_rust::{Yaml, YamlEmitter};

use crate::args::{Args, TargetFormat};
use crate::errors::ProconError;
use crate::nodes::Nodes;

#[cfg(test)]
#[path = "./nodes_writer_test.rs"]
mod nodes_writer_test;

pub fn to_yaml(args: &Args, nodes: &Nodes) -> Result<String> {
    debug!("Convert to yaml");
    let mut content = String::new();
    let mut emitter = YamlEmitter::new(&mut content);

    // todo refactor this
    let mut map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    for node in nodes.iter() {
        // root list treatment
        if node.name.is_empty() {
            let array: Yaml = node.into();
            emitter.dump(&array).map_err(|_| ProconError {
                message: "Could convert to yam format".to_string(),
            })?;
            break;
        } else {
            // the rest
            map.insert(Yaml::from_str(&node.name), node.into());
        }
    }

    if !map.is_empty() {
        let final_node = Yaml::Hash(map);
        emitter.dump(&final_node).map_err(|_| ProconError {
            message: "Could convert to yaml format".to_string(),
        })?;
    }

    output_content(&args, content)
}

pub fn to_json(args: &Args, nodes: &Nodes) -> Result<String> {
    debug!("Convert to json");
    let mut json_data = json::JsonValue::new_object();
    for node in nodes.iter() {
        // root list treatment
        if node.name.is_empty() {
            json_data = node.into();
        } else {
            json_data[node.name.clone()] = node.into();
        }
    }
    output_content(&args, json_data.pretty(1))
}

pub fn to_properties(args: &Args, nodes: &Nodes) -> Result<String> {
    debug!("Convert to properties");
    let mut string_content = "".to_string();
    for node in nodes.iter() {
        let content: String = node.into();
        string_content.push_str(&content);
    }
    output_content(&args, string_content)
}

pub fn to_toml(args: &Args, nodes: &Nodes) -> Result<String> {
    let mut string_content: String = "".to_string();
    for node in nodes.iter() {
        let toml_value: Value = node.into();
        string_content.push_str(toml::to_string_pretty(&toml_value)?.as_str());
    }

    output_content(&args, string_content)
}

fn output_content(args: &Args, content: String) -> Result<String> {
    println!("{}", content);
    if args.dry_run {
        Ok(String::from("Print converted format to console"))
    } else {
        let output_filename = determine_output_filename(&args);
        let mut output_file: File = File::create(&output_filename).map_err(|_| ProconError {
            message: "Could not create file".to_string(),
        })?;
        write!(output_file, "{}", content).map_err(|_| ProconError {
            message: "Could write to file".to_string(),
        })?;

        let mut message = "Converted ".to_string();
        message.push_str(&args.target_format.path_buf().to_str().unwrap());
        message.push_str(" to ");
        message.push_str(&output_filename);
        Ok(message)
    }
}

pub(crate) fn determine_output_filename(args: &Args) -> String {
    let output_filename: String;
    if args.output_filename.is_some() {
        output_filename = args.output_filename.as_ref().unwrap().to_string();
        debug!("User provided output filename {}", output_filename);
    } else {
        output_filename = default_filename(&args.target_format);
        debug!("User default output filename {}", output_filename);
    }
    output_filename
}

pub(crate) fn default_filename(command: &TargetFormat) -> String {
    let (path_buf, extension) = match command {
        TargetFormat::Properties { file, .. } => (file, "properties".to_string()),
        TargetFormat::Json { file, .. } => (file, "json".to_string()),
        TargetFormat::Yaml { file, .. } => (file, "yaml".to_string()),
        TargetFormat::Toml { file, .. } => (file, "toml".to_string()),
    };
    let mut filename = path_buf.file_stem().unwrap().to_str().unwrap();

    // stdin
    if path_buf == &PathBuf::from("-") {
        filename = "stdin";
    }
    return [filename, ".", extension.to_string().to_lowercase().as_str()].concat();
}
