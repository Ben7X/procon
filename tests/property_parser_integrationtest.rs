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
fn property_file_values_list() {
    let args = create_args(Delimiter::Equals, "tests/resources/list.properties");
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
fn property_file_values_string() {
    let args = create_args(Delimiter::Equals, "tests/resources/string.properties");
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
fn property_file_values_usize() {
    let args = create_args(Delimiter::Equals, "tests/resources/usize.properties");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let string_node = nodes.get(0).unwrap();
    assert_node(&string_node, "usize".to_string(), "20".to_string());
}

#[test]
fn property_file_values_float() {
    let args = create_args(Delimiter::Equals, "tests/resources/float.properties");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let string_node = nodes.get(0).unwrap();
    assert_node(&string_node, "float".to_string(), "20.4".to_string());
}

#[test]
fn property_file_nodes_nested() {
    let args = create_args(Delimiter::Equals, "tests/resources/nodes_nested.properties");
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    // reader node
    let reader_node = nodes.get(0).unwrap();
    assert_node(&reader_node, "reader".to_string(), "".to_string());

    let datasource_node = reader_node.children.get(0).unwrap();
    assert_node(&datasource_node, "datasource".to_string(), "".to_string());

    let host_node = datasource_node.children.get(0).unwrap();
    assert_node(&host_node, "host".to_string(), "localhost".to_string());
    let username_node = datasource_node.children.get(1).unwrap();
    assert_node(&username_node, "username".to_string(), "user".to_string());
}

#[test]
fn property_file_nodes_multiple() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/nodes_multiple.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    // reader node
    let reader_node = nodes.get(0).unwrap();
    assert_node(&reader_node, "reader".to_string(), "".to_string());
    let host_node = reader_node.children.get(0).unwrap();
    assert_node(&host_node, "host".to_string(), "localhost2".to_string());
    let username_node = reader_node.children.get(1).unwrap();
    assert_node(&username_node, "username".to_string(), "user".to_string());

    // writer node
    let writer_node = nodes.get(1).unwrap();
    assert_node(&writer_node, "writer".to_string(), "".to_string());
    let host_node = writer_node.children.get(0).unwrap();
    assert_node(&host_node, "host".to_string(), "localhost".to_string());
}

#[test]
fn property_file_delimiter_colon() {
    let args = create_args(
        Delimiter::Colon,
        "tests/resources/edge_case_colon.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let datasource_node = nodes.get(0).unwrap();
    assert_node(&datasource_node, "datasource".to_string(), "".to_string());
    let jdbc_url_node = datasource_node.children.get(0).unwrap();
    assert_node(
        &jdbc_url_node,
        "jdbc-url".to_string(),
        "jdbc:postgresql://localhost:5432/user".to_string(),
    );
    let username_node = datasource_node.children.get(1).unwrap();
    assert_node(&username_node, "username".to_string(), "user".to_string());
}

#[test]
fn property_file_edge_case_empty_value() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_empty_value.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let empty_node = nodes.get(0).unwrap();
    assert_node(&empty_node, "empty".to_string(), "".to_string());
}

#[test]
fn property_file_edge_case_whitespace_ignored() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_whitespace_ignored.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    let level1_node = nodes.get(0).unwrap();
    assert_node(&level1_node, "hello".to_string(), "hello".to_string());
}

#[test]
fn property_file_edge_case_duplicated_keys() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_duplicated_keys.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    let level1_node = nodes.get(0).unwrap();
    assert_node(
        &level1_node,
        "duplicateKey".to_string(),
        "second".to_string(),
    );
}

#[test]
fn property_file_edge_case_escape_even() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_escape_even.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    let even_key = nodes.get(0).unwrap();
    assert_node(
        &even_key,
        "evenKey".to_string(),
        "This is on one line\\\\".to_string(),
    );
}

#[test]
fn property_file_edge_case_multiline() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_multiline.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let multiline = nodes.get(0).unwrap();
    assert_node(
        &multiline,
        "multiline".to_string(),
        "This line continues".to_string(),
    );
}

#[test]
fn property_file_edge_case_escape_odd() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_escape_odd.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let odd_key = nodes.get(0).unwrap();
    assert_node(
        &odd_key,
        "oddKey".to_string(),
        "This is line one and\\\\# This is line two".to_string(),
    );
}

#[test]
fn property_file_edge_case_escape_path() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_escape_path.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let path = nodes.get(0).unwrap();
    assert_node(
        &path,
        "path".to_string(),
        "c:\\\\wiki\\\\templates".to_string(),
    );
}

#[test]
fn property_file_edge_case_escape_values() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_escape_values.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let value_with_escapes = nodes.get(0).unwrap();
    assert_node(
        &value_with_escapes,
        "valueWithEscapes".to_string(),
        "This is a newline\\n and a carriage return\\r and a tab\\t.".to_string(),
    );
}

#[test]
fn property_file_edge_case_escape() {
    let args = create_args(
        Delimiter::Equals,
        "tests/resources/edge_case_escape.properties",
    );
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();

    let welcome = nodes.get(0).unwrap();
    assert_node(
        &welcome,
        "welcome".to_string(),
        "Welcome to Wikipedia!".to_string(),
    );
}
