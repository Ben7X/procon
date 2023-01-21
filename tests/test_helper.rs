use log::LevelFilter;

use procon::args::{Args, TargetFormat};
use procon::node::Node;
use procon::nodes::Nodes;
use procon::parse_input_file;
use procon::property_file_reader::Delimiter;

pub fn create_args(delimiter: Delimiter, filename: &str) -> Args {
    let args: Args = Args {
        target_format: TargetFormat::Json {
            property_delimiter: delimiter,
            filename: filename.to_string(),
        },
        dry_run: false,
        log_level: LevelFilter::Off,
        output_filename: None,
    };
    args
}

pub fn parse_test_file(delimiter: Delimiter, filename: &str) -> Nodes {
    let args = create_args(delimiter, filename);
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    nodes
}

#[allow(dead_code)]
pub fn assert_node(node: &Node, name: String, value: String) {
    assert_eq!(name, node.name);
    assert_eq!(value, node.value.to_string());
}
