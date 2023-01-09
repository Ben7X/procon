#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::output_filename;

    #[test]
    fn output_filename_multiple_extensions() {
        let command = Command::Yaml;
        let filename = "test.properties.properties";
        assert_eq!("test.properties.yaml", output_filename(&command, filename));
    }

    #[test]
    fn output_filename_multiple_dots_input_name() {
        let command = Command::Yaml;
        let filename = "test.test2.test3.properties";
        assert_eq!("test.test2.test3.yaml", output_filename(&command, filename));
    }

    #[test]
    fn output_filename_yaml() {
        let command = Command::Yaml;
        let filename = "test.properties";
        assert_eq!("test.yaml", output_filename(&command, filename));
    }

    #[test]
    fn output_filename_json() {
        let command = Command::Json;
        let filename = "test.properties";
        assert_eq!("test.json", output_filename(&command, filename));
    }

    #[test]
    fn output_filename_properties() {
        let command = Command::Properties;
        let filename = "test.yaml";
        assert_eq!("test.properties", output_filename(&command, filename));
    }
}
