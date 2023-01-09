#[derive(Debug, Clone)]
pub struct Line {
    pub key: String,
    pub value: String,
    pub line_number: u32,
}

impl Line {
    pub fn new(key: &str, value: &str, line_number: u32) -> Self {
        Line {
            key: Line::sanitize_key(key),
            value: Line::sanitize_value(value),
            line_number,
        }
    }

    pub fn add_multiline(&mut self, value: &str) -> &Self {
        let new_value = Line::remove_last_char(&self.value);
        self.value = [new_value, Line::sanitize_value(value).as_str()].concat();
        self
    }

    fn remove_last_char(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next_back();
        chars.as_str()
    }

    fn sanitize_key(key: &str) -> String {
        // ignore whitespaces ent the end of key
        key.trim_end().to_string()
    }

    fn sanitize_value(value: &str) -> String {
        // ignore whitespaces ent the end of key
        value.trim_start().to_string()
    }
}
