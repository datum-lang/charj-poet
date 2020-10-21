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
    closed: bool,
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
            closed: false,
        }
    }

    pub fn has_pending_segments(&self) -> bool {
        self.segments.len() != 1 || !self.segments[0].is_empty()
    }

    pub fn append(
        &mut self,
        input: String,
        _indent_level: Option<i32>,
        _line_prefix: Option<String>,
    ) {
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

        let chars: Vec<char> = input.chars().collect();
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
                    // Render · as a non-breaking space.
                    let len = self.segments.len();
                    self.segments[len - 1].push(' ');
                    pos = pos + 1;
                }
                _ => {
                    let mut next = LineWrapper::index_of_any(&*chars, SPECIAL_CHARACTERS, pos);
                    if next == -1 {
                        next = chars.len() as i32;
                    }

                    let len = self.segments.len();
                    let i = next as usize - pos;
                    self.segments[len - 1] = input.chars().skip(pos).take(i as usize).collect();
                    pos = next as usize;
                }
            };
        }
    }

    pub fn index_of_any(
        chars: &[char],
        special_chars: [&'static str; 3],
        start_index: usize,
    ) -> i32 {
        for index in start_index..chars.len() {
            let char = chars[index];
            for schar in special_chars.iter() {
                if char == schar.parse().unwrap() {
                    return index as i32;
                }
            }
        }

        return -1;
    }

    pub fn new_line(&mut self) {
        self.emit_current_line();
        write!(self.out, "\n");
        self.indent_level = -1
    }

    pub fn emit_current_line(&mut self) {}

    pub fn close(&mut self) {
        self.closed = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::line_wrapper::{LineWrapper, SPECIAL_CHARACTERS};

    #[test]
    fn should_build_line_wrappers() {
        let chars: Vec<char> = "hello ".chars().collect();
        assert_eq!(5, LineWrapper::index_of_any(&*chars, SPECIAL_CHARACTERS, 0));
        assert_eq!(5, LineWrapper::index_of_any(&*chars, SPECIAL_CHARACTERS, 5));

        let empty: Vec<char> = "hello".chars().collect();
        assert_eq!(
            -1,
            LineWrapper::index_of_any(&*empty, SPECIAL_CHARACTERS, 0)
        );
    }

    #[test]
    fn should_get_pending_segments() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(out, String::from(""), 0);
        assert_eq!(false, wrapper.has_pending_segments());

        wrapper.append("hello ".to_string(), None, None);
        wrapper.new_line();
        wrapper.close();

        assert_eq!("\n", wrapper.out);
    }

    #[test]
    fn wrap() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(out, String::from("  "), 10);
        wrapper.append(String::from("abcde fghij"), Some(2), None);
        wrapper.close();
    }
}
