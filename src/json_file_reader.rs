use crate::args::Args;
use crate::node::{Node, NodeType};
use crate::nodes::Nodes;
use log::error;
use serde_json::Value;
use std::fs;

pub struct JsonFileReader {}

impl JsonFileReader {
    pub fn parse(args: &Args, output_filename: String) -> Result<Nodes, std::io::Error> {
        // todo return error codes
        let data: String = fs::read_to_string(&args.filename).expect("Unable to read file");
        let json_data: Value = serde_json::from_str(&data).expect("Unable to parse");
        Ok(Self::convert_json_values_to_nodes(
            &json_data,
            output_filename,
        ))
    }

    fn convert_json_values_to_nodes(json_data: &Value, output_filename: String) -> Nodes {
        let mut yaml_nodes: Nodes = Nodes::new(output_filename);
        match json_data {
            Value::Object(ref obj) => {
                for (key, value) in obj.iter() {
                    let mut parent = Self::json_to_node(&key, value, None, 0).unwrap();
                    yaml_nodes.merge(&mut parent);
                }
            }
            Value::Array(obj) => {
                for value in obj.iter() {
                    let mut parent = Self::json_to_node("", value, None, 0).unwrap();
                    yaml_nodes.merge(&mut parent);
                }
            }
            _ => error!("not valid json"),
        };
        yaml_nodes
    }

    fn json_to_node(
        key: &str,
        value: &Value,
        parent: Option<&mut Node>,
        level: usize,
    ) -> Option<Node> {
        let mut new_node: Node;
        if level == 0 {
            new_node = Node::new_json_node(key);
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
                    children.push(value.to_string());
                }
                new_node.value = NodeType::LIST(children);
                Some(new_node)
            }
            Value::Null => None,
        };
        new_node_option
    }
}
