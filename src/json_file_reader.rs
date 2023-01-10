use crate::node::{Node, NodeType};
use crate::nodes::Nodes;
use serde_json::Value;
use std::fs;

pub struct JsonFileReader {}

impl JsonFileReader {
    pub fn parse(path: &str) -> Result<Nodes, std::io::Error> {
        // use serde_json lib to load the json
        let data: String = fs::read_to_string(path).expect("Unable to read file");
        let json_data: Value = serde_json::from_str(&data).expect("Unable to parse");

        // convert serde_json values to internal nodes
        let yaml_nodes: Nodes = Self::convert_json_values_to_nodes(&json_data);
        Ok(yaml_nodes)
    }

    fn convert_json_values_to_nodes(json_data: &Value) -> Nodes {
        let mut yaml_nodes: Nodes = Nodes::new();
        match json_data {
            Value::Object(ref obj) => {
                for (key, value) in obj.iter() {
                    let mut parent = Self::json_to_node(&key, value, None, 0).unwrap();
                    yaml_nodes.merge(&mut parent);
                }
            }
            Value::Array(obj) => {}
            _ => println!("not a valid serde_json value"),
        };
        yaml_nodes
    }

    fn json_to_node(
        key: &str,
        value: &Value,
        parent: Option<&mut Node>,
        level: usize,
    ) -> Option<Node> {
        let new_node = match value {
            Value::String(json_value) => {
                let mut new_node = Node::new_child(level, parent.unwrap(), key);
                new_node.value = NodeType::parse(json_value);
                Some(new_node)
            }
            Value::Bool(json_value) => {
                let mut new_node = Node::new_child(level, parent.unwrap(), key);
                new_node.value = NodeType::parse(&json_value.to_string());
                Some(new_node)
            }
            Value::Number(json_value) => {
                let mut new_node = Node::new_child(level, parent.unwrap(), key);
                new_node.value = NodeType::parse(&json_value.to_string());
                Some(new_node)
            }
            Value::Object(json_value) => {
                // level 0 parent node
                let mut new_parent: Node;
                if level == 0 {
                    new_parent = Node::new_json_node(key);
                } else {
                    new_parent = Node::new_child(level, parent.unwrap(), key);
                }
                // create the children
                let mut children: Vec<Node> = vec![];
                for (map_key, map_value) in json_value.iter() {
                    let child_node =
                        Self::json_to_node(map_key, map_value, Some(&mut new_parent), level + 1);
                    if child_node.is_some() {
                        children.push(child_node.unwrap());
                    }
                }
                new_parent.children = children;
                Some(new_parent)
            }
            Value::Null => Some(Node::new_json_node(key)),
            Value::Array(_) => Some(Node::new_json_node(key)),
        };
        new_node
    }
}
