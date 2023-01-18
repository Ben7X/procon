use log::LevelFilter;

use procon::args::{Args, TargetFormat};
use procon::parse_input_file;
use procon::property_file_reader::Delimiter;

#[test]
fn property_file_values_list() {
    let args: Args = Args {
        target_format: TargetFormat::Json {
            property_delimiter: Delimiter::Equals,
            filename: "tests/resources/list.properties".to_string(),
        },
        dry_run: false,
        log_level: LevelFilter::Off,
        output_filename: None,
    };
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    let list_node = nodes.get(0).unwrap();
    assert_eq!("list", list_node.name);
    let values = list_node.children.get(0).unwrap();
    assert_eq!("all-types", values.name);
    assert_eq!("value1,2,20.4", values.value.to_string());
}

#[test]
fn property_file_values_string() {
    let args: Args = Args {
        target_format: TargetFormat::Json {
            property_delimiter: Delimiter::Equals,
            filename: "tests/resources/string.properties".to_string(),
        },
        dry_run: false,
        log_level: LevelFilter::Off,
        output_filename: None,
    };
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    let string_node = nodes.get(0).unwrap();
    assert_eq!("string", string_node.name);
    assert_eq!("This is a string", string_node.value.to_string());
}

#[test]
fn property_file_values_usize() {
    let args: Args = Args {
        target_format: TargetFormat::Json {
            property_delimiter: Delimiter::Equals,
            filename: "tests/resources/usize.properties".to_string(),
        },
        dry_run: false,
        log_level: LevelFilter::Off,
        output_filename: None,
    };
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    let string_node = nodes.get(0).unwrap();
    assert_eq!("usize", string_node.name);
    assert_eq!("20", string_node.value.to_string());
}

#[test]
fn property_file_values_float() {
    let args: Args = Args {
        target_format: TargetFormat::Json {
            property_delimiter: Delimiter::Equals,
            filename: "tests/resources/float.properties".to_string(),
        },
        dry_run: false,
        log_level: LevelFilter::Off,
        output_filename: None,
    };
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    let string_node = nodes.get(0).unwrap();
    assert_eq!("float", string_node.name);
    assert_eq!("20.4", string_node.value.to_string());
}

#[test]
fn property_file_nodes_nested() {
    let args: Args = Args {
        target_format: TargetFormat::Json {
            property_delimiter: Delimiter::Equals,
            filename: "tests/resources/nodes_nested.properties".to_string(),
        },
        dry_run: false,
        log_level: LevelFilter::Off,
        output_filename: None,
    };
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    // reader node
    let reader_node = nodes.get(0).unwrap();
    assert_eq!("reader", reader_node.name);
    assert_eq!("", reader_node.value.to_string());

    let datasource_node = reader_node.children.get(0).unwrap();
    assert_eq!("datasource", datasource_node.name);
    assert_eq!("", datasource_node.value.to_string());

    let host_node = datasource_node.children.get(0).unwrap();
    assert_eq!("host", host_node.name);
    assert_eq!("localhost", host_node.value.to_string());
    let username_node = datasource_node.children.get(1).unwrap();
    assert_eq!("username", username_node.name);
    assert_eq!("user", username_node.value.to_string());
}

#[test]
fn property_file_nodes_multiple() {
    let args: Args = Args {
        target_format: TargetFormat::Json {
            property_delimiter: Delimiter::Equals,
            filename: "tests/resources/nodes_multiple.properties".to_string(),
        },
        dry_run: false,
        log_level: LevelFilter::Off,
        output_filename: None,
    };
    let mut nodes = parse_input_file(&args).unwrap();
    nodes.sort();
    println!("{:?}", nodes);

    // reader node
    let reader_node = nodes.get(0).unwrap();
    assert_eq!("reader", reader_node.name);
    assert_eq!("", reader_node.value.to_string());
    let host_node = reader_node.children.get(0).unwrap();
    assert_eq!("host", host_node.name);
    assert_eq!("localhost2", host_node.value.to_string());
    let username_node = reader_node.children.get(1).unwrap();
    assert_eq!("username", username_node.name);
    assert_eq!("user", username_node.value.to_string());

    // writer node
    let writer_node = nodes.get(1).unwrap();
    assert_eq!("writer", writer_node.name);
    assert_eq!("", writer_node.value.to_string());
    let host_node = writer_node.children.get(0).unwrap();
    assert_eq!("host", host_node.name);
    assert_eq!("localhost", host_node.value.to_string());
}
