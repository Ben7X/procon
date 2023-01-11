use linked_hash_map::LinkedHashMap;
use yaml_rust::{Yaml, YamlEmitter};

use crate::nodes::Nodes;
use std::fs::File;
use std::io::Write;

pub fn to_yaml(nodes: &Nodes) {
    let mut output_file = File::create(nodes.get_output_filename()).unwrap();
    // convert into yaml-rust datatypes
    let mut map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    for node in nodes.iter() {
        map.insert(Yaml::from_str(&node.name), node.into());
    }

    let mut content = String::new();
    let mut emitter = YamlEmitter::new(&mut content);
    let final_node = Yaml::Hash(map);
    emitter.dump(&final_node).unwrap();
    writeln!(output_file, "{}", content).unwrap();
}

pub fn to_json(nodes: &Nodes) {
    let mut output_file = File::create(nodes.get_output_filename()).unwrap();
    let mut json_data = json::JsonValue::new_object();
    for node in nodes.iter() {
        json_data[node.name.clone()] = node.into();
    }
    writeln!(output_file, "{}", json_data.pretty(1)).unwrap();
}

pub fn to_properties(nodes: &Nodes) {
    let mut output_file = File::create(nodes.get_output_filename()).unwrap();
    for node in nodes.iter() {
        let content: String = node.into();
        write!(output_file, "{}", content).unwrap();
    }
}
