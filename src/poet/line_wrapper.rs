use std::fmt::Write;

pub const SPECIAL_CHARACTERS: [&'static str; 3] = [" ", "\n", "."];

///
/// Implements soft line wrapping on an appendable. To use, append characters using {@link #append}
/// or soft-wrapping spaces using {@link #wrappingSpace}.
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LineWrapper {
    pub out: String,
    pub indent: String,
    pub statement_line: i32,
    pub segments: Vec<String>,
    pub indent_level: i32,
    pub line_prefix: String,
}

impl LineWrapper {
    pub fn new(out: String, indent: String, statement_line: i32) -> Self {
        LineWrapper {
            out,
            indent,
            statement_line,
            segments: vec!["".to_string()],
            indent_level: -1,
            line_prefix: "".to_string(),
        }
    }

    pub fn has_pending_segments(&self) -> bool {
        self.segments.len() != 1 || !self.segments[0].is_empty()
    }

    pub fn append(&mut self, s: String, _indent_level: Option<i32>, _line_prefix: Option<String>) {
        let mut pos: usize = 0;

        let indent_level: i32;
        match _indent_level {
            None => indent_level = -1,
            Some(level) => indent_level = level,
        }

        let line_prefix: String;
        match _line_prefix {
            None => line_prefix = "".to_string(),
            Some(prefix) => line_prefix = prefix,
        }

        let chars: Vec<char> = s.chars().collect();
        while pos < chars.len() {
            let char = chars[pos];
            match char {
                ' ' => {
                    self.indent_level = indent_level;
                    self.line_prefix = line_prefix.clone();
                    self.segments.push(String::from(""));
                    pos = pos + 1;
                }
                '\n' => {
                    self.new_line();
                    pos = pos + 1;
                }
                '.' => {
                    pos = pos + 1;
                }
                _ => {
                    // s.indexo
                    pos = pos + 1;
                }
            };
        }
    }

    pub fn new_line(&mut self) {
        self.emit_current_line();
        write!(self.out, "\n");
        self.indent_level = -1
    }

    pub fn emit_current_line(&mut self) {}
}

#[cfg(test)]
mod tests {
    use crate::poet::line_wrapper::LineWrapper;

    #[test]
    fn should_get_pending_segements() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(out, String::from(""), 0);
        assert_eq!(false, wrapper.has_pending_segments());

        wrapper.append("hello ".to_string(), None, None);
        wrapper.new_line();

        assert_eq!("\n", wrapper.out);
    }
}
