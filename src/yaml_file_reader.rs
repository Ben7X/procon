use std::fs::File;
use std::io::Read;
use std::process;

use log::error;
use serde::Deserialize;
use serde_yaml::Value;

use crate::args::Args;
use crate::node::{Node, NodeType};
use crate::nodes::Nodes;

#[cfg(test)]
#[path = "./yaml_file_reader_test.rs"]
mod yaml_file_reader_test;

pub struct YamlFileReader {}

impl YamlFileReader {
    pub fn parse(args: &Args) -> Result<Nodes, &'static str> {
        let filename = args.target_format.filename();
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => {
                process::exit(exitcode::CONFIG);
            }
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        Ok(Self::convert_yaml_values_to_nodes(&mut content))
    }

    fn convert_yaml_values_to_nodes(content: &mut String) -> Nodes {
        let mut nodes: Nodes = Nodes::new();
        for document in serde_yaml::Deserializer::from_str(&content) {
            let value: Value = Value::deserialize(document).unwrap();
            match value {
                Value::Mapping(ref obj) => {
                    for (map_key, map_value) in obj.iter() {
                        let mut parent =
                            Self::yaml_to_node(map_key.as_str().unwrap(), &map_value, None, 0)
                                .unwrap();
                        nodes.merge(&mut parent);
                    }
                }
                _ => error!("not valid yaml"),
            }
        }

        nodes
    }

    fn yaml_to_node(
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

        let new_node_option = match value {
            Value::Bool(yaml_value) => {
                new_node.value = NodeType::parse(&yaml_value.to_string());
                Some(new_node)
            }
            Value::Number(yaml_value) => {
                new_node.value = NodeType::parse(&yaml_value.to_string());
                Some(new_node)
            }
            Value::String(yaml_value) => {
                new_node.value = NodeType::parse(yaml_value);
                Some(new_node)
            }
            Value::Sequence(yaml_value) => {
                let mut children: Vec<String> = vec![];
                for value in yaml_value.iter() {
                    let string_value = Self::value_to_string(value);
                    children.push(string_value);
                }
                new_node.value = NodeType::ARRAY(children);
                Some(new_node)
            }
            Value::Mapping(yaml_value) => {
                let mut children: Vec<Node> = vec![];
                for (map_key, map_value) in yaml_value.iter() {
                    let child_node = Self::yaml_to_node(
                        map_key.as_str().unwrap(),
                        map_value,
                        Some(&mut new_node),
                        level + 1,
                    );
                    if child_node.is_some() {
                        children.push(child_node.unwrap());
                    }
                }
                new_node.children = children;
                Some(new_node)
            }
            Value::Tagged(_) => None,
            Value::Null => None,
        };
        new_node_option
    }

    fn value_to_string(value: &Value) -> String {
        if value.is_number() {
            return value.as_f64().unwrap().to_string();
        }
        value.as_str().unwrap().to_string()
    }
}
