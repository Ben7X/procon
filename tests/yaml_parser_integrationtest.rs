use procon::property_file_reader::Delimiter;

use crate::test_helper::{assert_node, parse_test_file};

mod test_helper;

#[test]
fn yaml_file_string() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/yaml/string.yaml");
    println!("{:?}", nodes);

    let string_node = nodes.get(0).unwrap();
    assert_node(
        &string_node,
        "string1".to_string(),
        "Sample String 1".to_string(),
    );

    let string_node = nodes.get(1).unwrap();
    assert_node(
        &string_node,
        "string2".to_string(),
        "Sample String 2".to_string(),
    );

    let string_node = nodes.get(2).unwrap();
    assert_node(
        &string_node,
        "string3".to_string(),
        "Sample String 3".to_string(),
    );
}

#[test]
fn yaml_file_list() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/yaml/list.yaml");
    println!("{:?}", nodes);

    let list_node = nodes.get(0).unwrap();
    assert_node(&list_node, "list".to_string(), "A,B,C".to_string());
}

#[test]
fn yaml_file_root_list() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/yaml/root-list.yaml");
    println!("{:?}", nodes);

    let list_node = nodes.get(0).unwrap();
    assert_node(&list_node, "".to_string(), "A,B,C".to_string());
}

#[test]
fn yaml_file_types() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/yaml/types.yaml");
    println!("{:?}", nodes);

    let float_node = nodes.get(0).unwrap();
    assert_node(&float_node, "a".to_string(), "123.0".to_string());
    let string_node = nodes.get(1).unwrap();
    assert_node(&string_node, "b".to_string(), "123".to_string());
}

#[test]
fn yaml_file_comment() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/yaml/comment.yaml");

    let float_node = nodes.get(0).unwrap();
    assert_node(&float_node, "key".to_string(), "value".to_string());
}

#[test]
fn yaml_file_example() {
    let nodes = parse_test_file(Delimiter::Equals, "tests/resources/yaml/example.yaml");
    println!("{:?}", nodes);

    let address_node = nodes.get(0).unwrap();
    assert_node(&address_node, "address".to_string(), "".to_string());
    let list_node = address_node.children.get(0).unwrap();
    assert_node(&list_node, "list".to_string(), "A,B,C".to_string());
    let street_node = address_node.children.get(1).unwrap();
    assert_node(
        &street_node,
        "street".to_string(),
        "123 Tornado Alley\nSuite 16            \ncity:   East Centerville\nstate:  KS\n"
            .to_string(),
    );

    let contact_node = nodes.get(1).unwrap();
    assert_node(&contact_node, "contact".to_string(), "".to_string());
    let home_node = contact_node.children.get(0).unwrap();
    assert_node(&home_node, "home".to_string(), "1012355532".to_string());
    let office_node = contact_node.children.get(1).unwrap();
    assert_node(&office_node, "office".to_string(), "5002586256".to_string());

    let name_node = nodes.get(2).unwrap();
    assert_node(&name_node, "name".to_string(), "John Smith".to_string());
}
