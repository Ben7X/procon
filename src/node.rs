extern crate exitcode;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

use json::JsonValue;
use linked_hash_map::LinkedHashMap;
use log::{debug, trace};
use yaml_rust::Yaml;

#[cfg(test)]
#[path = "./node_test.rs"]
mod node_test;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    BOOLEAN(bool),
    NUMERIC(String),
    STRING(String),
    OBJECT(String),
    ARRAY(Vec<String>),
    NONE,
}

impl NodeType {
    pub fn parse(value: &str) -> NodeType {
        if value.to_lowercase() == "true" || value.to_lowercase() == "false" {
            return NodeType::BOOLEAN(FromStr::from_str(&value.to_lowercase()).unwrap());
        }

        if value.to_lowercase().starts_with("{") {
            return NodeType::OBJECT(value.to_string());
        }

        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() > 1 {
            let mut array: Vec<String> = vec![];
            for value in parts.iter() {
                // trailing commas will result in empty string
                if value.len() > 0 {
                    array.push(value.to_string())
                }
            }
            return NodeType::ARRAY(array);
        }

        let mut is_numeric = match value.parse::<f64>() {
            Ok(_) => true,
            Err(_) => false,
        };
        if !is_numeric {
            is_numeric = match value.parse::<usize>() {
                Ok(_) => true,
                Err(_) => false,
            };
        }
        if is_numeric {
            return NodeType::NUMERIC(value.parse().unwrap());
        }
        NodeType::STRING(value.to_string())
    }

    pub fn to_string(&self) -> String {
        match &self {
            NodeType::STRING(value) => value.clone(),
            NodeType::NUMERIC(value) => value.clone(),
            NodeType::BOOLEAN(value) => value.to_string(),
            NodeType::OBJECT(value) => value.clone(),
            NodeType::ARRAY(array) => {
                let mut formatted_string: String = "".to_string();
                for (index, value) in array.iter().enumerate() {
                    if index != 0 {
                        formatted_string.push_str(",");
                    }
                    formatted_string.push_str(value.as_str());
                }
                formatted_string
            }
            NodeType::NONE => "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub level: usize,
    pub parent: Option<Rc<Node>>,
    pub children: Vec<Node>,
    pub name: String,
    pub value: NodeType,
}

#[allow(dead_code)]
impl Node {
    fn new(
        level: usize,
        parent: Option<Rc<Node>>,
        children: Vec<Node>,
        name: String,
        value: NodeType,
    ) -> Node {
        let node = Node {
            level,
            parent,
            children,
            name,
            value,
        };
        debug!("Create new node {:?}", node);
        node
    }

    pub fn new_from_name_and_level(level: usize, name: &str) -> Node {
        Self::new(level, None, Vec::new(), String::from(name), NodeType::NONE)
    }

    pub fn new_from_name(name: &str) -> Node {
        Self::new_from_name_and_level(0, name)
    }

    pub fn new_child(level: usize, parent: &mut Node, name: &str) -> Node {
        Self::new(
            level,
            Some(Rc::new(parent.to_owned())),
            Vec::new(),
            String::from(name),
            NodeType::NONE,
        )
    }

    pub fn find_common_node(&mut self, new_node: &Node) -> bool {
        trace!(
            "Find common node {:?} in children {:?}",
            new_node.name,
            self.children
        );

        // case not the same base node
        if self.level == 0 && new_node.level == 0 && self != new_node {
            return true;
        }

        // case same node
        if self == new_node {
            return self.find_common_node(&new_node.children[0]);
        }

        for existing_node in &mut self.children {
            if existing_node == new_node {
                return existing_node.find_common_node(&new_node.children[0]);
            }
        }

        trace!("Merge {:?} into {:?}", new_node, self);
        let children = &mut self.children;
        children.push(new_node.to_owned());
        return false;
    }

    pub fn sort(&mut self) {
        for node in &mut self.children {
            if !node.children.is_empty() {
                node.sort();
            }
        }
        self.children.sort();
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.level == other.level
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// adapter
impl Into<String> for &Node {
    fn into(self) -> String {
        match self.value {
            // beginning just collects the backwards walk
            NodeType::NONE => {
                let mut data = String::new();
                for children in &self.children {
                    let next_part: String = children.into();
                    data.push_str(&next_part);
                }
                return data.to_string();
            }
            // walk backwards
            _ => {
                let mut data = String::new();
                let new_data = parent_name(self);
                // reverse order
                let split_parts: Vec<&str> = new_data.split(".").collect();
                for (index, node_part) in split_parts.iter().rev().enumerate() {
                    data.push_str(node_part);
                    if index != split_parts.len() - 1 {
                        data.push_str(".");
                    }
                }
                // push key and value onto string
                data.push_str("=");
                data.push_str(&self.value.to_string());
                data.push_str("\n");
                data
            }
        }
    }
}

fn parent_name(node: &Node) -> String {
    let mut data: String = node.name.to_string();
    if node.parent.is_some() {
        let new_data = parent_name(&node.parent.as_ref().unwrap());
        data.push_str(".");
        data.push_str(&new_data.to_owned());
    }
    data
}

impl Into<JsonValue> for &Node {
    fn into(self) -> JsonValue {
        let data = match &self.value {
            NodeType::NONE => {
                let mut data = JsonValue::new_object();
                for children in &self.children {
                    data[children.name.clone()] = children.into();
                }
                return data;
            }
            NodeType::BOOLEAN(value) => JsonValue::Boolean(value.clone()),
            NodeType::NUMERIC(value) => {
                // let num_value = value.parse::<i32>().unwrap();
                // JsonValue::Number(num_value.into())
                return match value.parse::<f64>() {
                    Ok(parsed_value) => JsonValue::Number(parsed_value.into()),
                    Err(_) => match value.parse::<usize>() {
                        Ok(parsed_value) => JsonValue::Number(parsed_value.into()),
                        Err(_) => JsonValue::Number(0.into()),
                    },
                };
            }
            NodeType::STRING(value) => JsonValue::String(value.clone()),
            NodeType::OBJECT(value) => JsonValue::String(value.clone()),
            NodeType::ARRAY(value) => {
                let mut array = vec![];
                for element in value {
                    let json_value: JsonValue = JsonValue::String(element.clone());
                    array.push(json_value);
                }
                JsonValue::Array(array)
            }
        };
        data
    }
}

impl Into<Yaml> for &Node {
    fn into(self) -> Yaml {
        let data = match &self.value {
            NodeType::NONE => {
                let mut map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
                for child in &self.children {
                    let key = Yaml::from_str(&child.name);
                    let yaml_node: Yaml = child.into();
                    if !map.contains_key(&key) {
                        map.insert(key.clone(), yaml_node);
                        continue;
                    }
                    map.insert(key, yaml_node);
                }
                return Yaml::Hash(map);
            }
            NodeType::BOOLEAN(value) => Yaml::from_str(&value.to_string()),
            NodeType::NUMERIC(value) => Yaml::from_str(&value),
            NodeType::STRING(value) => Yaml::from_str(&value),
            NodeType::OBJECT(value) => Yaml::from_str(&value),
            NodeType::ARRAY(value) => {
                let mut array = vec![];
                for element in value {
                    let yaml_value: Yaml = Yaml::from_str(&element);
                    array.push(yaml_value);
                }
                Yaml::Array(array)
            }
        };
        data
    }
}
