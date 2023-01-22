use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use log::{debug, trace};

use crate::args::Args;
use crate::errors::ConfigFileError;
use crate::line::Line;
use crate::node::{Node, NodeType};
use crate::nodes::Nodes;

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
    pub(crate) content: HashMap<String, Line>,
    pub(crate) last_key: String,
}

#[allow(dead_code)]
impl PropertyFileReader {
    pub fn parse(args: &Args) -> Result<Nodes, ConfigFileError> {
        let filename = &args.target_format.filename();
        let file = File::open(filename).map_err(|_| ConfigFileError {
            message: "Cannot open file".to_string(),
        })?;

        let config_file = Self::read_lines(args, file);
        Self::convert_property_to_nodes(&config_file)
    }
    fn convert_property_to_nodes(
        config_file: &PropertyFileReader,
    ) -> Result<Nodes, ConfigFileError> {
        let mut yaml_nodes: Nodes = Nodes::new();
        for (prop_key, line) in config_file.content.iter() {
            let mut node_parts = prop_key.split(".").collect::<Vec<&str>>();
            trace!("Node parts: {:?}", node_parts);
            if node_parts.is_empty() {
                trace!("Ignore empty parts");
                continue;
            }

            let name = node_parts[0];
            node_parts.remove(0);
            let mut new_node = Node::new_from_name(name);

            // case key has no sub nodes
            if node_parts.len() == 0 {
                new_node.value = NodeType::parse(&line.value);
                yaml_nodes.merge(&mut new_node);
                continue;
            }

            // create children
            Self::create_child_nodes(&mut new_node, &mut node_parts, &line.value);
            yaml_nodes.merge(&mut new_node);
        }

        Ok(yaml_nodes)
    }

    pub fn create_child_nodes(node: &mut Node, parts: &mut Vec<&str>, value: &str) {
        let mut last_node = &mut *node;
        for (index, name) in parts.iter().enumerate() {
            let mut new_node = Node::new_child(index + 1, last_node, name);
            if index == parts.len() - 1 {
                new_node.value = NodeType::parse(value);
            }

            let children = &mut last_node.children;
            children.push(new_node.clone());
            last_node = &mut children[0];
        }
    }

    fn read_lines(args: &Args, file: File) -> PropertyFileReader {
        let reader = BufReader::new(file);
        let mut config_file = PropertyFileReader::new();
        let mut line_number = 1;
        let delimiter = &args.target_format.delimiter();
        for result_line in reader.lines() {
            let line = result_line.unwrap();
            config_file.process_line(line, line_number, &delimiter.unwrap());
            line_number = line_number + 1;
        }
        config_file
    }

    fn new() -> PropertyFileReader {
        PropertyFileReader {
            content: HashMap::new(),
            last_key: String::from(""),
        }
    }

    fn process_line(&mut self, line: String, line_number: u32, delimiter: &Delimiter) {
        // case empty lines
        if line.is_empty() {
            return;
        }
        // case comments consider multiline here
        if !self.consider_multiline() && line.starts_with("#") || line.starts_with("!") {
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
            debug!("'{}' is a multiline", value);
            self.last_key = self.sanitize_key(key);
        }
        self.add(key, value, line_number);
    }

    fn add(&mut self, key: &str, value: &str, line_number: u32) -> Option<Line> {
        // ignore whitespaces ent the end of key and at the beginning of value
        let line = Line::new(key, value, line_number);
        debug!("Adding to content {:?}", line);
        return self.content.insert(line.key.clone(), line.clone());
    }

    fn add_multiline(&mut self, line: &String) {
        // get last line
        let previous_line = self.content.get_mut(&self.last_key).unwrap();
        previous_line.add_multiline(line);
        self.last_key = String::from("");
    }

    /// ignore whitespaces ent the end of key
    fn sanitize_key(&self, key: &str) -> String {
        key.trim_end().to_string()
    }

    /// ignore whitespaces ent the end of key
    fn sanitize_value(&self, value: &str) -> String {
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
        debug!("{}", even_odd);

        even_odd == 1
    }
}
