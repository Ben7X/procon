#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use clap_verbosity_flag::Verbosity;

    use crate::args::{Args, TargetFormat};
    use crate::nodes_writer::{default_filename, determine_output_filename};
    use crate::property_file_reader::Delimiter;

    #[test]
    fn default_filename_multiple_extensions() {
        let command = TargetFormat::Yaml {
            property_delimiter: Delimiter::Equals,
            file: PathBuf::from("test.properties.properties"),
        };
        assert_eq!("test.properties.yaml", default_filename(&command));
    }

    #[test]
    fn default_filename_multiple_dots_input_name() {
        let command = TargetFormat::Yaml {
            property_delimiter: Delimiter::Equals,
            file: PathBuf::from("test.test2.test3.properties"),
        };
        assert_eq!("test.test2.test3.yaml", default_filename(&command));
    }

    #[test]
    fn default_filename_yaml() {
        let command = TargetFormat::Yaml {
            property_delimiter: Delimiter::Equals,
            file: PathBuf::from("test.properties"),
        };
        assert_eq!("test.yaml", default_filename(&command));
    }

    #[test]
    fn default_filename_json() {
        let command = TargetFormat::Json {
            property_delimiter: Delimiter::Equals,
            file: PathBuf::from("test.properties"),
        };
        assert_eq!("test.json", default_filename(&command));
    }

    #[test]
    fn default_filename_properties() {
        let command = TargetFormat::Properties {
            property_delimiter: Delimiter::Equals,
            file: PathBuf::from("test.yaml"),
        };
        assert_eq!("test.properties", default_filename(&command));
    }

    #[test]
    fn determine_default_filename_default() {
        let args: Args = Args {
            target_format: TargetFormat::Properties {
                property_delimiter: Delimiter::Equals,
                file: PathBuf::from("filename.properties"),
            },
            dry_run: false,
            from_property_file: false,
            from_yaml_file: false,
            from_json_file: false,
            output_filename: None,
            verbose: Verbosity::new(0, 0),
        };

        let file: String = determine_output_filename(&args);
        assert_eq!("filename.properties", file);
    }

    #[test]
    fn determine_default_filename_command_line_input() {
        let cli_output_file: String = "test.yaml".to_string();
        let args: Args = Args {
            target_format: TargetFormat::Properties {
                property_delimiter: Delimiter::Equals,
                file: PathBuf::from("filename.properties"),
            },
            dry_run: false,
            from_property_file: false,
            from_yaml_file: false,
            from_json_file: false,
            output_filename: Some(cli_output_file.to_string()),
            verbose: Verbosity::new(0, 0),
        };

        let file: String = determine_output_filename(&args);
        assert_eq!(cli_output_file, file);
    }
}
