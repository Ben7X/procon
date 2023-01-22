use procon::property_file_reader::Delimiter;
use test_helper::parse_test_file;

use crate::test_helper::assert_node;

mod test_helper;

#[test]
fn json_file_values_list() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/json/list.json");

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
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/json/string.json");

    let string_node = nodes.get(0).unwrap();
    assert_node(
        &string_node,
        "string".to_string(),
        "This is a string".to_string(),
    );
}

#[test]
fn json_file_values_usize() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/json/usize.json");

    let string_node = nodes.get(0).unwrap();
    assert_node(&string_node, "usize".to_string(), "20".to_string());
}

#[test]
fn json_file_values_float() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/json/float.json");

    let string_node = nodes.get(0).unwrap();
    assert_node(&string_node, "float".to_string(), "20.4".to_string());
}

#[test]
fn json_file_nodes_nested() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/json/application.json");

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
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/json/root-list.json");

    let list_node = nodes.get(0).unwrap();
    assert_node(&list_node, "".to_string(), "test,20,20.4,true".to_string());
}
