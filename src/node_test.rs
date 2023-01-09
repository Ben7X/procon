#[cfg(test)]
mod tests {
    use crate::node::{Node, NodeType};
    use std::vec;

    #[test]
    fn node_value_retrieve_type_string() {
        let value = "string";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::STRING(v) => assert_eq!(value, v),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn node_value_retrieve_type_string_empty() {
        let value = "";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::STRING(v) => assert_eq!(value, v),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn node_value_retrieve_type_boolean_false() {
        let value = "faLse";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::BOOLEAN(v) => {
                assert_eq!(v, false);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn node_value_retrieve_type_boolean_true() {
        let value = "tRue";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::BOOLEAN(v) => {
                assert_eq!(v, true);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn node_value_retrieve_type_boolean_false_lowercase() {
        let value = "false";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::BOOLEAN(v) => {
                assert_eq!(v, false);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn node_value_retrieve_type_boolean_true_lowercase() {
        let value = "true";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::BOOLEAN(v) => {
                assert_eq!(v, true);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn node_value_retrieve_type_object() {
        let value = "{\"key\": \"value\"}";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::OBJECT(v) => {
                assert_eq!(v, value);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn node_value_retrieve_type_numeric() {
        let value = "1";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::NUMERIC(v) => {
                assert_eq!(v, value);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn node_value_retrieve_type_numeric_flaoting() {
        let value = "1.5";
        let node_type = NodeType::parse(value);
        match node_type {
            NodeType::NUMERIC(v) => {
                assert_eq!(v, value);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn new_node_one_level() {
        let mut keys = vec!["level0"];
        let value = "testvalue";
        let node = Node::new(&mut keys, value);

        assert_eq!("level0", node.name);
        assert_eq!(NodeType::parse(value), node.value);
        assert_eq!(0, node.children.len());
    }

    #[test]
    fn new_node_no_value() {
        let mut keys = vec!["level0"];
        let value = "";
        let node = Node::new(&mut keys, value);

        assert_eq!("level0", node.name);
        assert_eq!(NodeType::parse(value), node.value);
        assert_eq!(0, node.children.len());
    }

    #[test]
    fn new_node_multiple_level() {
        let mut keys = vec!["level0", "level1", "level2"];
        let value = "testvalue";
        let node = Node::new(&mut keys, value);

        assert_eq!(NodeType::NONE, node.value);
        assert_eq!("level0", node.name);
        assert_eq!(1, node.children.len());

        let level1 = &node.children[0];
        assert_eq!(NodeType::NONE, level1.value);
        assert_eq!("level1", level1.name);
        assert_eq!(1, level1.children.len());

        let level2 = &level1.children[0];
        assert_eq!("level2", level2.name);
        assert_eq!(NodeType::parse(value), level2.value);
        assert_eq!(0, level2.children.len());
    }

    #[test]
    fn find_common_node_same_base_level() {
        let mut keys = vec!["level0", "level1", "level2"];
        let value = "test1";
        let mut node = Node::new(&mut keys, value);
        let mut keys2 = vec!["level0", "level1", "otherLevel"];
        let value2 = "test2";
        let node2 = Node::new(&mut keys2, value2);

        let to_add = node.find_common_node(&node2);
        assert!(!to_add);

        // base level
        assert_eq!(1, node.children.len());

        // level 1
        let level1 = &node.children[0];
        assert_eq!(2, level1.children.len());

        // level 2 first children
        let level2_first_node = &level1.children[0];
        assert_eq!("level2", level2_first_node.name);
        assert_eq!(NodeType::parse(value), level2_first_node.value);

        // level 2 second children
        let level2_second_node = &level1.children[1];
        assert_eq!("otherLevel", level2_second_node.name);
        assert_eq!(NodeType::parse(value2), level2_second_node.value);
    }

    #[test]
    fn find_common_node_different_base_level() {
        let mut keys = vec!["level0", "level1", "level2"];
        let value = "test1";
        let mut node = Node::new(&mut keys, value);
        let mut keys2 = vec!["otherLevel", "level1", "level2"];
        let value2 = "test2";
        let node2 = Node::new(&mut keys2, value2);

        let to_add = node.find_common_node(&node2);
        assert!(to_add)
    }

    #[test]
    fn into_json() {
        let mut keys = vec!["level0", "level1", "level2"];
        let value = "test1";
        let node = Node::new(&mut keys, value);
        let data = json::stringify(&node);
        println!("{}", data);
    }
}
