#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::json_file_reader::JsonFileReader;

    #[test]
    fn convert_json_values_to_nodes_multiple_nodes() {
        let output_filename = String::from("output.json");
        let content = String::from("{\"reader\":{\"datasource\":{\"jdbc-url\":\"localhost\"}},\"writer\":{\"datasource\":{\"jdbc-url\":\"localhost\"}} }");
        let json_data: Value = serde_json::from_str(&content).expect("Unable to parse");

        let nodes = JsonFileReader::convert_json_values_to_nodes(&json_data, output_filename);

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("reader", reader_node.name);
        let datasource_node = reader_node.children.get(0).unwrap();
        assert_eq!("datasource", datasource_node.name);
        let jdbc_url_node = datasource_node.children.get(0).unwrap();
        assert_eq!("jdbc-url", jdbc_url_node.name);
        assert_eq!("localhost", jdbc_url_node.value.to_string());

        let writer_node = nodes.get(1).unwrap();
        assert_eq!("writer", writer_node.name);
        let datasource_node = writer_node.children.get(0).unwrap();
        assert_eq!("datasource", datasource_node.name);
        let jdbc_url_node = datasource_node.children.get(0).unwrap();
        assert_eq!("jdbc-url", jdbc_url_node.name);
        assert_eq!("localhost", jdbc_url_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_single_node() {
        let content = String::from("{\"reader\": \"reader-value\"}");
        let json_data: Value = serde_json::from_str(&content).expect("Unable to parse");

        let nodes =
            JsonFileReader::convert_json_values_to_nodes(&json_data, String::from("output.json"));

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("reader", reader_node.name);
        assert_eq!("reader-value", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_array() {
        let content = String::from("{\"readers\": [\"value-1\",\"value-1\"]}");
        let json_data: Value = serde_json::from_str(&content).expect("Unable to parse");

        let nodes =
            JsonFileReader::convert_json_values_to_nodes(&json_data, String::from("output.json"));

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("readers", reader_node.name);
        assert_eq!("value-1,value-1", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_empty_array() {
        let content = String::from("{\"readers\": []}");
        let json_data: Value = serde_json::from_str(&content).expect("Unable to parse");

        let nodes =
            JsonFileReader::convert_json_values_to_nodes(&json_data, String::from("output.json"));

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("readers", reader_node.name);
        assert_eq!("", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_bool() {
        let content = String::from("{\"isReader\": true}");
        let json_data: Value = serde_json::from_str(&content).expect("Unable to parse");

        let nodes =
            JsonFileReader::convert_json_values_to_nodes(&json_data, String::from("output.json"));

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("isReader", reader_node.name);
        assert_eq!("true", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_number() {
        let content = String::from("{\"isReader\": 1}");
        let json_data: Value = serde_json::from_str(&content).expect("Unable to parse");

        let nodes =
            JsonFileReader::convert_json_values_to_nodes(&json_data, String::from("output.json"));

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("isReader", reader_node.name);
        assert_eq!("1", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_float_number() {
        let content = String::from("{\"isReader\": 1.78}");
        let json_data: Value = serde_json::from_str(&content).expect("Unable to parse");

        let nodes =
            JsonFileReader::convert_json_values_to_nodes(&json_data, String::from("output.json"));

        let reader_node = nodes.get(0).unwrap();
        assert_eq!("isReader", reader_node.name);
        assert_eq!("1.78", reader_node.value.to_string());
    }

    #[test]
    fn convert_json_values_to_nodes_empty_json() {
        let content = String::from("{}");
        let json_data: Value = serde_json::from_str(&content).expect("Unable to parse");

        let nodes =
            JsonFileReader::convert_json_values_to_nodes(&json_data, String::from("output.json"));

        assert_eq!(None, nodes.get(0));
    }
}
