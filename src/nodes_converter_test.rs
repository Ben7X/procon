#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use crate::args::{Args, Command};
    use crate::nodes_converter::{default_filename, determine_output_filename};
    use crate::property_file_reader::Delimiter;

    #[test]
    fn default_filename_multiple_extensions() {
        let command = Command::Yaml {
            filename: "test.properties.properties".to_string(),
        };
        assert_eq!("test.properties.yaml", default_filename(&command));
    }

    #[test]
    fn default_filename_multiple_dots_input_name() {
        let command = Command::Yaml {
            filename: "test.test2.test3.properties".to_string(),
        };
        assert_eq!("test.test2.test3.yaml", default_filename(&command));
    }

    #[test]
    fn default_filename_yaml() {
        let command = Command::Yaml {
            filename: "test.properties".to_string(),
        };
        assert_eq!("test.yaml", default_filename(&command));
    }

    #[test]
    fn default_filename_json() {
        let command = Command::Json {
            filename: "test.properties".to_string(),
        };
        assert_eq!("test.json", default_filename(&command));
    }

    #[test]
    fn default_filename_properties() {
        let command = Command::Properties {
            property_delimiter: Delimiter::Equals,
            filename: "test.yaml".to_string(),
        };
        assert_eq!("test.properties", default_filename(&command));
    }

    #[test]
    fn determine_default_filename_default() {
        let args: Args = Args {
            command: Command::Properties {
                property_delimiter: Delimiter::Equals,
                filename: "filename.properties".to_string(),
            },
            dry_run: false,
            log_level: LevelFilter::Off,
            output_filename: None,
        };

        let filename: String = determine_output_filename(&args);
        assert_eq!("filename.properties", filename);
    }

    #[test]
    fn determine_default_filename_command_line_input() {
        let cli_output_filename: String = "test.yaml".to_string();
        let args: Args = Args {
            command: Command::Properties {
                property_delimiter: Delimiter::Equals,
                filename: "filename.properties".to_string(),
            },
            dry_run: false,
            log_level: LevelFilter::Off,
            output_filename: Some(cli_output_filename.to_string()),
        };

        let filename: String = determine_output_filename(&args);
        assert_eq!(cli_output_filename, filename);
    }
}
