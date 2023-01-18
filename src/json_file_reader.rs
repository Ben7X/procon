use std::fs;

use log::{debug, error};
use serde_json::Value;

use crate::args::Args;
use crate::errors::ConfigFileError;
use crate::node::{Node, NodeType};
use crate::nodes::Nodes;

#[cfg(test)]
#[path = "./json_file_reader_test.rs"]
mod json_file_reader_test;

pub struct JsonFileReader {}

impl JsonFileReader {
    pub fn parse(args: &Args) -> Result<Nodes, ConfigFileError> {
        let filename = args.target_format.filename();
        let data: String = fs::read_to_string(filename).map_err(|_| ConfigFileError {
            error: "Unable to read file".to_string(),
        })?;
        let json_data: Value = serde_json::from_str(&data).map_err(|_| ConfigFileError {
            error: "Unable to parse".to_string(),
        })?;
        Self::convert_json_values_to_nodes(&json_data)
    }

    fn convert_json_values_to_nodes(json_data: &Value) -> Result<Nodes, ConfigFileError> {
        let mut nodes: Nodes = Nodes::new();
        match json_data {
            Value::Object(ref obj) => {
                for (key, value) in obj.iter() {
                    let mut parent = Self::json_to_node(&key, value, None, 0).unwrap();
                    nodes.merge(&mut parent);
                }
            }
            Value::Array(obj) => {
                for value in obj.iter() {
                    let mut parent = Self::json_to_node("", value, None, 0).unwrap();
                    nodes.merge(&mut parent);
                }
            }
            _ => error!("not valid json"),
        };
        Ok(nodes)
    }

    fn json_to_node(
        key: &str,
        value: &Value,
        parent: Option<&mut Node>,
        level: usize,
    ) -> Option<Node> {
        let mut new_node: Node;
        if level == 0 {
            new_node = Node::new_from_name(key);
        } else {
            new_node = Node::new_child(level, parent.unwrap(), key);
        }

        // get values
        let new_node_option = match value {
            Value::String(json_value) => {
                new_node.value = NodeType::parse(json_value);
                return Some(new_node);
            }
            Value::Bool(json_value) => {
                new_node.value = NodeType::parse(&json_value.to_string());
                Some(new_node)
            }
            Value::Number(json_value) => {
                new_node.value = NodeType::parse(&json_value.to_string());
                Some(new_node)
            }
            Value::Object(json_value) => {
                let mut children: Vec<Node> = vec![];
                for (map_key, map_value) in json_value.iter() {
                    let child_node =
                        Self::json_to_node(map_key, map_value, Some(&mut new_node), level + 1);
                    if child_node.is_some() {
                        children.push(child_node.unwrap());
                    }
                }
                new_node.children = children;
                Some(new_node)
            }
            Value::Array(json_value) => {
                let mut children: Vec<String> = vec![];
                for value in json_value.iter() {
                    debug!("{}", value);
                    children.push(value.to_string().replace("\"", ""));
                }
                new_node.value = NodeType::ARRAY(children);
                Some(new_node)
            }
            Value::Null => None,
        };
        new_node_option
    }
}
