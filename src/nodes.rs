use crate::node::Node;
use log::trace;
use std::slice::Iter;
use std::string::String;

pub struct Nodes {
    pub nodes: Vec<Node>,
    pub output_filename: String,
}

#[allow(dead_code)]
impl Nodes {
    pub fn new(output_filename: String) -> Nodes {
        Nodes {
            nodes: Vec::new(),
            output_filename,
        }
    }
    pub fn get_output_filename(&self) -> String {
        String::from(&self.output_filename)
    }

    pub fn iter(&self) -> Iter<'_, Node> {
        self.nodes.iter()
    }

    pub fn push(&mut self, node: Node) {
        trace!("Add node to the yaml nodes {:?} ", node);
        self.nodes.push(node);
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn sort(&mut self) {
        for node in &mut self.nodes {
            node.sort();
        }
        self.nodes.sort()
    }

    pub fn merge(&mut self, new_node: &mut Node) {
        if self.nodes.is_empty() {
            self.nodes.push(new_node.to_owned());
            return;
        }
        for existing_node in &mut self.nodes.iter_mut() {
            let to_add = existing_node.find_common_node(&new_node);
            if !to_add {
                return;
            }
        }
        self.nodes.push(new_node.to_owned());
    }
}
