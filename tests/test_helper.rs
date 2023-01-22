use std::path::PathBuf;

use clap_verbosity_flag::Verbosity;

use procon::args::{Args, TargetFormat};
use procon::node::Node;
use procon::nodes::Nodes;
use procon::parse_input_file;
use procon::property_file_reader::Delimiter;

pub fn create_args(delimiter: Delimiter, filename: &str) -> Args {
    let args: Args = Args {
        target_format: TargetFormat::Json {
            property_delimiter: delimiter,
            file: PathBuf::from(filename),
        },
        dry_run: false,
        from_property_file: false,
        from_yaml_file: false,
        from_json_file: false,
        output_filename: None,
        verbose: Verbosity::new(0, 0),
    };
    args
}

pub fn parse_test_file(delimiter: Delimiter, filename: &str) -> Nodes {
    let args = create_args(delimiter, filename);
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    nodes
}

pub fn assert_node(node: &Node, name: String, value: String) {
    assert_eq!(name, node.name);
    assert_eq!(value, node.value.to_string());
}
