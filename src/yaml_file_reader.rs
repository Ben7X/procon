use std::fs::File;
use std::io::Read;

use log::error;
use serde_yaml::Value;

use crate::args::Args;
use crate::errors::ConfigFileError;
use crate::node::{Node, NodeType};
use crate::nodes::Nodes;

#[cfg(test)]
#[path = "./yaml_file_reader_test.rs"]
mod yaml_file_reader_test;

pub struct YamlFileReader {}
impl YamlFileReader {
    pub fn parse(args: &Args) -> Result<Nodes, ConfigFileError> {
        let filename = args.target_format.filename();
        let mut file = File::open(filename).map_err(|_| ConfigFileError {
            error: "Cannot open file".to_string(),
        })?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|_| ConfigFileError {
                error: "Cannot read from file".to_string(),
            })?;
        let yaml_value: Value = serde_yaml::from_str(&content).map_err(|_| ConfigFileError {
            error: "Wrong yaml format".to_string(),
        })?;

        Self::convert_yaml_values_to_nodes(&yaml_value)
    }

    fn convert_yaml_values_to_nodes(yaml_value: &Value) -> Result<Nodes, ConfigFileError> {
        let mut nodes: Nodes = Nodes::new();
        match yaml_value {
            Value::Mapping(ref obj) => {
                for (map_key, map_value) in obj.iter() {
                    let mut parent =
                        Self::yaml_to_node(map_key.as_str().unwrap(), &map_value, None, 0).unwrap();
                    nodes.merge(&mut parent);
                }
            }
            _ => error!("not valid yaml"),
        }
        Ok(nodes)
    }
    // fn convert_yaml_values_to_nodes(content: &mut String) -> Result<Nodes, ConfigFileError> {
    //     let mut nodes: Nodes = Nodes::new();
    //     for document in serde_yaml::Deserializer::from_str(&content) {
    //         let value: Value = Value::deserialize(document).map_err(|_| ConfigFileError {
    //             error: "Wrong yaml format".to_string(),
    //         })?;
    //         match value {
    //             Value::Mapping(ref obj) => {
    //                 for (map_key, map_value) in obj.iter() {
    //                     let mut parent =
    //                         Self::yaml_to_node(map_key.as_str().unwrap(), &map_value, None, 0)
    //                             .unwrap();
    //                     nodes.merge(&mut parent);
    //                 }
    //             }
    //             _ => error!("not valid yaml"),
    //         }
    //     }
    //     Ok(nodes)
    // }

    fn yaml_to_node(
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
                    let string_value = Self::yaml_value_to_string(value);
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

    fn yaml_value_to_string(value: &Value) -> String {
        if value.is_number() {
            return value.as_f64().unwrap().to_string();
        }
        value.as_str().unwrap().to_string()
    }
}
