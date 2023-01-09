use crate::line::Line;
use crate::node::Node;
use log::{info, trace};

pub struct Nodes {
    pub nodes: Vec<Node>,
    input_lines: usize,
    comments: Vec<Line>,
    blank_lines: Vec<u32>,
}

#[allow(dead_code)]
impl Nodes {
    pub fn new(input_lines: usize, comments: Vec<Line>, blank_lines: Vec<u32>) -> Nodes {
        Nodes {
            nodes: Vec::new(),
            input_lines,
            comments,
            blank_lines,
        }
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

    pub fn print_statistics(&self) {
        let mut statistic = "Statistic\n".to_string();
        statistic.push_str("Processed ");
        statistic.push_str(&self.input_lines.to_string());
        statistic.push_str(" properties\n");
        for comment in &self.comments {
            statistic.push_str("Ignore Comment at line ");
            statistic.push_str(&comment.line_number.to_string());
            statistic.push_str(" -> ");
            statistic.push_str(&comment.value.to_string());
            statistic.push_str("\n");
        }

        for blank_line in &self.blank_lines {
            statistic.push_str("Ignore blank at line ");
            statistic.push_str(&blank_line.to_string());
            statistic.push_str("\n");
        }
        info!("{}", statistic);
    }
}
