use crate::line::Line;
use crate::node::Node;
use crate::nodes::Nodes;
use log::trace;
use std::{
    collections::HashMap,
    fmt::Display,
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
    last_key: String,
}

#[allow(dead_code)]
impl PropertyFileReader {
    pub fn parse(
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
        let mut yaml_nodes: Nodes = Nodes::new();

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
            last_key: String::from(""),
        }
    }

    fn get_content(&self) -> &HashMap<String, Line> {
        &self.content
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
        trace!("{}", even_odd);

        even_odd == 1
    }
}
