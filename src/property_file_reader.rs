use crate::line::Line;
use crate::node::{Node, NodeType};
use crate::nodes::Nodes;
use log::trace;
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[cfg(test)]
#[path = "./property_file_reader_test.rs"]
mod property_file_reader_test;

#[derive(Debug, Clone)]
pub enum Delimiter {
    Equals,
    Colon,
    Whitespace,
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Delimiter {
    fn value(&self) -> char {
        match self {
            Delimiter::Equals => '=',
            Delimiter::Colon => ':',
            Delimiter::Whitespace => ' ',
        }
    }
}

impl FromStr for Delimiter {
    type Err = String;

    fn from_str(input: &str) -> Result<Delimiter, Self::Err> {
        match input.to_lowercase().as_str() {
            "=" => Ok(Delimiter::Equals),
            ":" => Ok(Delimiter::Colon),
            " " => Ok(Delimiter::Whitespace),
            &_ => Ok(Delimiter::Equals),
        }
    }
}

#[derive(Debug)]
pub struct PropertyFileReader {
    content: HashMap<String, Line>,
    error_lines: Vec<Line>,
    comments: Vec<Line>,
    blank_lines: Vec<u32>,
    last_key: String,
}

fn json_to_node(key: &str, value: &Value, parent: &mut Node, level: usize) -> Node {
    trace!("create node with key {} and value {}", key, value);
    let new_node = match value {
        Value::String(json_value) => {
            let mut new_node = Node::new_child(level, parent, key);
            new_node.value = NodeType::parse(json_value);
            new_node
        }
        Value::Bool(json_value) => {
            let mut new_node = Node::new_child(level, parent, key);
            new_node.value = NodeType::parse(&json_value.to_string());
            new_node
        }
        Value::Number(json_value) => {
            let mut new_node = Node::new_child(level, parent, key);
            new_node.value = NodeType::parse(&json_value.to_string());
            new_node
        }
        Value::Object(json_value) => {
            let mut new_parent = Node::new_child(level, parent, key);
            let mut children: Vec<Node> = vec![];
            for (map_key, map_value) in json_value.iter() {
                let child_level = level + 1;
                let child_node = json_to_node(map_key, map_value, &mut new_parent, child_level);
                children.push(child_node.to_owned());
            }
            new_parent.children.append(&mut children.to_owned());
            new_parent
        }
        Value::Null => Node::new_json_node(key),
        Value::Array(_) => Node::new_json_node(key),
    };

    trace!("create node {:?}", new_node);
    new_node
}

#[allow(dead_code)]
impl PropertyFileReader {
    pub fn parse_json_file(path: &str) -> Result<Nodes, std::io::Error> {
        let data: String = fs::read_to_string(path).expect("Unable to read file");
        let res: Value = serde_json::from_str(&data).expect("Unable to parse");

        // create new content
        let mut config_file = PropertyFileReader::new();
        let property_map: &HashMap<String, Line> = config_file.get_content();
        let mut yaml_nodes: Nodes = Nodes::new(
            property_map.len(),
            config_file.comments.to_owned(),
            config_file.blank_lines.to_owned(),
        );

        // check what json it is
        match res {
            Value::Object(ref obj) => {
                for (mut key, value) in obj.iter() {
                    let mut parent = Node::new_json_node(key);
                    let child = json_to_node(&key, value, &mut parent, 0);
                    parent.children.append(&mut vec![child]);
                    yaml_nodes.merge(&mut parent);
                }
            }
            _ => println!("not a valid serde_json value"),
        };
        for node in yaml_nodes.nodes.iter() {
            println!("node {}", node.name)
        }
        Ok(yaml_nodes)
    }
    pub fn parse_property_file(
        file: &File,
        filename: &str,
        delimiter: &Delimiter,
    ) -> Result<Nodes, std::io::Error> {
        let reader = BufReader::new(file);

        let mut config_file = PropertyFileReader::new();
        let mut line_number = 1;
        for result_line in reader.lines() {
            let line = result_line.unwrap();
            config_file.process_line(line, line_number, delimiter);
            line_number = line_number + 1;
        }
        trace!("Read {} successfully", filename);

        let property_map: &HashMap<String, Line> = config_file.get_content();
        let mut yaml_nodes: Nodes = Nodes::new(
            property_map.len(),
            config_file.comments.to_owned(),
            config_file.blank_lines.to_owned(),
        );

        let mut new_node: Node;
        for (prop_key, line) in property_map.iter() {
            let mut node_parts = prop_key.split(".").collect::<Vec<&str>>();
            trace!("Node parts {:?}", node_parts);
            if node_parts.is_empty() {
                trace!("Ignoring empty parts");
                continue;
            }
            new_node = Node::new(&mut node_parts, &line.value);
            yaml_nodes.merge(&mut new_node);
        }

        Ok(yaml_nodes)
    }

    fn new() -> PropertyFileReader {
        PropertyFileReader {
            content: HashMap::new(),
            error_lines: Vec::new(),
            comments: Vec::new(),
            blank_lines: Vec::new(),
            last_key: String::from(""),
        }
    }

    fn get_content(&self) -> &HashMap<String, Line> {
        &self.content
    }

    fn process_line(&mut self, line: String, line_number: u32, delimiter: &Delimiter) {
        // case empty lines
        if line.is_empty() {
            self.add_blank_line(line_number);
            return;
        }
        // case comments consider multiline here
        if !self.consider_multiline() && line.starts_with("#") || line.starts_with("!") {
            self.add_comment(line, line_number);
            return;
        }

        // only support one delimiter per file
        let parts = line.split_once(delimiter.value());
        // get value pop gets the last value
        let (key, value) = match parts {
            None => ("", ""),
            Some(key_value) => key_value,
        };

        // case empty key if delimiter cannot split or multiline part 2...
        if key.is_empty() {
            if self.content.contains_key(&self.last_key) {
                self.add_multiline(&line);
                return;
            }
            self.add(&line, value, line_number);
            return;
        }

        // check for multi line comment
        if self.is_multiline(value) {
            trace!("'{}' is a multiline", value);
            self.last_key = self.sanitize_key(key);
        }
        self.add(key, value, line_number);
    }

    fn add(&mut self, key: &str, value: &str, line_number: u32) -> Option<Line> {
        // ignore whitespaces ent the end of key and at the beginning of value
        let line = Line::new(key, value, line_number);
        trace!("Adding to content {:?}", line);
        return self.content.insert(line.key.clone(), line.clone());
    }

    fn add_multiline(&mut self, line: &String) {
        // get last line
        let previous_line = self.content.get_mut(&self.last_key).unwrap();
        previous_line.add_multiline(line);
        self.last_key = String::from("");
    }

    fn sanitize_key(&self, key: &str) -> String {
        // ignore whitespaces ent the end of key
        key.trim_end().to_string()
    }

    fn sanitize_value(&self, value: &str) -> String {
        // ignore whitespaces ent the end of key
        value.trim_start().to_string()
    }

    fn consider_multiline(&self) -> bool {
        !self.last_key.is_empty()
    }

    fn is_multiline(&self, value: &str) -> bool {
        if !value.ends_with("\\") {
            return false;
        }
        // even keys are one line odd keys are multiline
        let mut counter = 0;
        for c in value.chars().rev() {
            if c != '\\' {
                break;
            }
            counter = counter + 1;
        }
        let even_odd = counter % 2;
        trace!("{}", even_odd);

        even_odd == 1
    }

    fn add_comment(&mut self, value: String, line_number: u32) {
        let line = Line::new("", &value, line_number);
        trace!("Ignoring commented line {:?}", &line);
        self.comments.push(line)
    }

    fn add_error_line(&mut self, value: String, line_number: u32) {
        let line = Line::new("", &value, line_number);
        trace!("Add error line {:?}", &line);
        self.error_lines.push(line);
    }

    fn add_blank_line(&mut self, line_number: u32) {
        trace!("Ignoring empty line at {}", line_number);
        self.blank_lines.push(line_number);
    }
}
