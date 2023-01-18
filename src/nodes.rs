use std::slice::Iter;

use crate::node::Node;
use log::debug;

#[derive(Debug)]
pub struct Nodes {
    nodes: Vec<Node>,
}

#[allow(dead_code)]
impl Nodes {
    pub fn new() -> Nodes {
        Nodes { nodes: Vec::new() }
    }

    pub fn iter(&self) -> Iter<'_, Node> {
        self.nodes.iter()
    }

    pub fn push(&mut self, node: Node) {
        debug!("Add node to the yaml nodes {:?} ", node);
        self.nodes.push(node);
    }

    pub fn get(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
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
