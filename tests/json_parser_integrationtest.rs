use log::LevelFilter;
use procon::args::{Args, TargetFormat};
use procon::node::Node;
use procon::parse_input_file;
use procon::property_file_reader::Delimiter;

fn create_args(delimiter: Delimiter, filename: &str) -> Args {
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

fn assert_node(node: &Node, name: String, value: String) {
    assert_eq!(name, node.name);
    assert_eq!(value, node.value.to_string());
}

#[test]
fn json_file_values_list() {
    let args = create_args(Delimiter::Equals, "tests/resources/list.json");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let list_node = nodes.get(0).unwrap();
    assert_node(&list_node, "list".to_string(), "".to_string());

    let values = list_node.children.get(0).unwrap();
    assert_node(
        &values,
        "all-types".to_string(),
        "value1,2,20.4".to_string(),
    );
}
#[test]
fn json_file_values_string() {
    let args = create_args(Delimiter::Equals, "tests/resources/string.json");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let string_node = nodes.get(0).unwrap();
    assert_node(
        &string_node,
        "string".to_string(),
        "This is a string".to_string(),
    );
}

#[test]
fn json_file_values_usize() {
    let args = create_args(Delimiter::Equals, "tests/resources/usize.json");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let string_node = nodes.get(0).unwrap();
    assert_node(&string_node, "usize".to_string(), "20".to_string());
}

#[test]
fn json_file_values_float() {
    let args = create_args(Delimiter::Equals, "tests/resources/float.json");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let string_node = nodes.get(0).unwrap();
    assert_node(&string_node, "float".to_string(), "20.4".to_string());
}

#[test]
fn json_file_nodes_nested() {
    let args = create_args(Delimiter::Equals, "tests/resources/application.json");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let management_node = nodes.get(0).unwrap();
    assert_node(&management_node, "management".to_string(), "".to_string());

    let metrics_node = management_node.children.get(0).unwrap();
    assert_node(&metrics_node, "metrics".to_string(), "".to_string());
    let port_node = management_node.children.get(1).unwrap();
    assert_node(&port_node, "port".to_string(), "8080".to_string());

    let enable_node = metrics_node.children.get(0).unwrap();
    assert_node(&enable_node, "enable".to_string(), "".to_string());

    let all = enable_node.children.get(0).unwrap();
    assert_node(&all, "all".to_string(), "false".to_string());
    let http_node = enable_node.children.get(1).unwrap();
    assert_node(&http_node, "http".to_string(), "true".to_string());
}

// todo currently root list conversion is not working at all
#[test]
fn json_file_value_root_list() {
    let args = create_args(Delimiter::Equals, "tests/resources/root-list.json");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    let list_node = nodes.get(0).unwrap();
    assert_node(&list_node, "".to_string(), "test,20,20.4,true".to_string());
}
