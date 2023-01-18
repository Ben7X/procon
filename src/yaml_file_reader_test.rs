#[cfg(test)]
mod tests {
    use serde_yaml::Value;

    use crate::yaml_file_reader::YamlFileReader;

    #[test]
    fn convert_yaml_values_to_nodes_multiple_nodes() {
        let content = String::from(
            "writer:
        datasource:
            jdbc-url: localhost
reader:
        datasource:
            jdbc-url: localhost",
        );
        let yaml_data: Value = serde_yaml::from_str(&content).expect("Unable to parse");
        let nodes = YamlFileReader::convert_yaml_values_to_nodes(&yaml_data).unwrap();

        let writer_node = nodes.get(0).unwrap();
        assert_eq!("writer", writer_node.name);
        let datasource_node = writer_node.children.get(0).unwrap();
        assert_eq!("datasource", datasource_node.name);
        let jdbc_url_node = datasource_node.children.get(0).unwrap();
        assert_eq!("jdbc-url", jdbc_url_node.name);
        assert_eq!("localhost", jdbc_url_node.value.to_string());

        let reader_node = nodes.get(1).unwrap();
        assert_eq!("reader", reader_node.name);
        let datasource_node = reader_node.children.get(0).unwrap();
        assert_eq!("datasource", datasource_node.name);
        let jdbc_url_node = datasource_node.children.get(0).unwrap();
        assert_eq!("jdbc-url", jdbc_url_node.name);
        assert_eq!("localhost", jdbc_url_node.value.to_string());
    }

    #[test]
    fn convert_yaml_values_to_nodes_single_nodes() {
        let content = String::from(
            "writer:
        datasource:
            jdbc-url: localhost",
        );
        let yaml_data: Value = serde_yaml::from_str(&content).expect("Unable to parse");
        let nodes = YamlFileReader::convert_yaml_values_to_nodes(&yaml_data).unwrap();

        let writer_node = nodes.get(0).unwrap();
        assert_eq!("writer", writer_node.name);
        let datasource_node = writer_node.children.get(0).unwrap();
        assert_eq!("datasource", datasource_node.name);
        let jdbc_url_node = datasource_node.children.get(0).unwrap();
        assert_eq!("jdbc-url", jdbc_url_node.name);
        assert_eq!("localhost", jdbc_url_node.value.to_string());
    }

    #[test]
    fn convert_yaml_values_to_nodes_array() {
        let content = String::from(
            "readers:
        - value-1
        - value-2",
        );
        let yaml_data: Value = serde_yaml::from_str(&content).expect("Unable to parse");
        let nodes = YamlFileReader::convert_yaml_values_to_nodes(&yaml_data).unwrap();

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("readers", reader_node.name);
        assert_eq!("value-1,value-2", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_bool() {
        let content = String::from("isReader: true");
        let yaml_data: Value = serde_yaml::from_str(&content).expect("Unable to parse");
        let nodes = YamlFileReader::convert_yaml_values_to_nodes(&yaml_data).unwrap();

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("isReader", reader_node.name);
        assert_eq!("true", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_number() {
        let content = String::from("isReader: 1");
        let yaml_data: Value = serde_yaml::from_str(&content).expect("Unable to parse");
        let nodes = YamlFileReader::convert_yaml_values_to_nodes(&yaml_data).unwrap();

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("isReader", reader_node.name);
        assert_eq!("1", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_float_number() {
        let content = String::from("isReader: 1.78");
        let yaml_data: Value = serde_yaml::from_str(&content).expect("Unable to parse");
        let nodes = YamlFileReader::convert_yaml_values_to_nodes(&yaml_data).unwrap();

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("isReader", reader_node.name);
        assert_eq!("1.78", reader_node.value.to_string());
    }

    #[test]
    #[should_panic]
    fn convert_json_values_to_nodes_empty_json() {
        let content = String::from("");
        let _yaml_data: Value = serde_yaml::from_str(&content).expect("Unable to parse");
    }
}
