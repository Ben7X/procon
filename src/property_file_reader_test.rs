#[cfg(test)]
mod tests {
    use crate::line::Line;
    use crate::property_file_reader::{Delimiter, PropertyFileReader};
    use std::collections::HashMap;

    fn assert_content(map: &HashMap<String, Line>, key: &str, value: &str) {
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(key).unwrap().value, value);
    }

    #[test]
    fn process_line_delimiter_equals() {
        let line = String::from("website=https://en.wikipedia.org/");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Equals);

        assert_content(
            &property_file.content,
            "website",
            "https://en.wikipedia.org/",
        );
    }

    #[test]
    fn process_line_with_spaces_on_key() {
        let line = String::from("website =https://en.wikipedia.org/");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Equals);

        assert_content(
            &property_file.content,
            "website",
            "https://en.wikipedia.org/",
        );
    }

    #[test]
    fn process_line_with_spaces_on_value() {
        let line = String::from("website= https://en.wikipedia.org/");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Equals);

        assert_content(
            &property_file.content,
            "website",
            "https://en.wikipedia.org/",
        );
    }

    #[test]
    fn process_line_delimiter_colon() {
        let line = String::from("website:https://en.wikipedia.org/");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Colon);

        assert_content(
            &property_file.content,
            "website",
            "https://en.wikipedia.org/",
        );
    }

    #[test]
    fn process_line_delimiter_whitespace() {
        let line = String::from("website https://en.wikipedia.org/");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Whitespace);

        assert_content(
            &property_file.content,
            "website",
            "https://en.wikipedia.org/",
        );
    }

    #[test]
    fn process_line_empty_line() {
        let line = String::from("");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Equals);

        assert!(&property_file.content.is_empty());
    }

    #[test]
    fn process_line_comment_hash() {
        let line = String::from("#website=https://en.wikipedia.org/");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Equals);

        assert!(&property_file.content.is_empty());
    }

    #[test]
    fn process_line_comment_exclamation_mark() {
        let line = String::from("!website=https://en.wikipedia.org/");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Whitespace);

        assert!(&property_file.content.is_empty());
    }

    #[test]
    fn process_line_empty_value() {
        let line = String::from("empty");
        let mut property_file = PropertyFileReader::new();
        property_file.process_line(line, 1, &Delimiter::Whitespace);

        assert_content(&property_file.content, "empty", "");
    }

    #[test]
    fn process_line_comment_multiline() {
        let mut property_file = PropertyFileReader::new();
        let line = String::from("multiline=This line \\");
        let line2 = String::from("#continues");
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);
        property_file.process_line(line2, 2, &delimiter);

        assert_content(&property_file.content, "multiline", "This line #continues");
    }

    #[test]
    fn process_line_multiline() {
        let mut property_file = PropertyFileReader::new();
        let line = String::from("multiline=This line \\");
        let line2 = String::from("continues");
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);
        property_file.process_line(line2, 2, &delimiter);

        assert_content(&property_file.content, "multiline", "This line continues");
    }

    #[test]
    fn process_line_multiline_with_whitespace() {
        let mut property_file = PropertyFileReader::new();
        let line = String::from("multiline=This line \\");
        let line2 = String::from("    continues");
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);
        property_file.process_line(line2, 2, &delimiter);

        assert_content(&property_file.content, "multiline", "This line continues");
    }

    #[test]
    fn process_line_multiline_even() {
        let mut property_file = PropertyFileReader::new();
        // this \\\\ represents two slashes \\
        let line = String::from("evenKey = This is on one line\\\\");
        let line2 = String::from(
            "# This line is a normal comment and is not included in the value for evenKey",
        );
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);
        property_file.process_line(line2, 2, &delimiter);

        assert_content(&property_file.content, "evenKey", "This is on one line\\\\");
    }

    #[test]
    fn process_line_multiline_odd() {
        let mut property_file = PropertyFileReader::new();
        let line = String::from("oddKey = This is on one line\\\\\\");
        let line2 = String::from("# This is line two off an odd key");
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);
        property_file.process_line(line2, 2, &delimiter);

        assert_content(
            &property_file.content,
            "oddKey",
            "This is on one line\\\\# This is line two off an odd key",
        );
    }

    #[test]
    fn process_line_multiline_sanitize() {
        let mut property_file = PropertyFileReader::new();
        let line = String::from("welcome = Welcome to \\");
        let line2 = String::from("          Wikipedia!");
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);
        property_file.process_line(line2, 2, &delimiter);

        assert_content(&property_file.content, "welcome", "Welcome to Wikipedia!");
    }

    #[test]
    fn process_line_encoded_with_uft8() {
        let mut property_file = PropertyFileReader::new();
        let line = String::from("helloInJapanese = こんにちは");
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);

        assert_content(&property_file.content, "helloInJapanese", "こんにちは");
    }

    #[test]
    fn process_line_encoded() {
        let mut property_file = PropertyFileReader::new();
        let line = String::from("encodedHelloInJapanese = \\u3053\\u3093\\u306b\\u3061\\u306");
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);

        assert_content(
            &property_file.content,
            "encodedHelloInJapanese",
            "\\u3053\\u3093\\u306b\\u3061\\u306",
        );
    }

    #[test]
    fn process_line_duplicate_key_last_wins() {
        let mut property_file = PropertyFileReader::new();
        let line = String::from("duplicateKey = first");
        let line2 = String::from("duplicateKey = second");
        let delimiter = Delimiter::Equals;
        property_file.process_line(line, 1, &delimiter);
        property_file.process_line(line2, 2, &delimiter);

        assert_content(&property_file.content, "duplicateKey", "second");
    }

    #[test]
    fn is_multiline_no_slash() {
        let property_file = PropertyFileReader::new();
        let line = String::from("oddKey = This is on one line");
        assert_eq!(false, property_file.is_multiline(&line));
    }

    #[test]
    fn is_multiline_odd_slash() {
        let property_file = PropertyFileReader::new();
        let line = String::from("oddKey = This is on one line\\\\\\");
        assert_eq!(true, property_file.is_multiline(&line));
    }

    #[test]
    fn is_multiline_even_slash() {
        let property_file = PropertyFileReader::new();
        let line = String::from("evenKey = This is on one line\\\\");
        assert_eq!(false, property_file.is_multiline(&line));
    }
}
