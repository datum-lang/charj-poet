use regex::Regex;
use std::fmt::Write;

pub const SPECIAL_CHARACTERS: [char; 3] = [' ', '\n', '·'];
lazy_static! {
    static ref UNSAFE_LINE_START: Regex = Regex::new(r"\\s*[-+].*").unwrap();
    // static ref SPECIAL_CHARACTERS: Regex = Regex::new(r"\\s*[-+].*").unwrap();
}

///
/// Implements soft line wrapping on an appendable. To use, append characters using {@link #append}
/// or soft-wrapping spaces using {@link #wrappingSpace}.
///
#[derive(Serialize, Debug)]
pub struct LineWrapper<'a> {
    pub out: &'a mut String,
    pub indent: String,
    pub column_limit: i32,
    pub segments: Vec<String>,
    pub indent_level: i32,
    pub line_prefix: String,
    closed: bool,
}

impl<'a> LineWrapper<'a> {
    pub fn new(out: &'a mut String, indent: String, column_limit: i32) -> Self {
        LineWrapper {
            out,
            indent,
            column_limit,
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
                '·' => {
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
                    let others: String = input.chars().skip(pos).take(i as usize).collect();
                    self.segments[len - 1].push_str(others.as_str());
                    pos = next as usize;
                }
            };
        }
    }

    pub fn index_of_any(chars: &[char], special_chars: [char; 3], start_index: usize) -> i32 {
        for index in start_index..chars.len() {
            let ch: char = chars[index];
            for schar in special_chars.iter() {
                if *schar == ch {
                    return index as i32;
                }
            }
        }

        return -1;
    }

    pub fn append_non_wrapping(&mut self, str: String) {
        let len = self.segments.len();
        for char in str.chars() {
            self.segments[len - 1].push(char);
        }
    }

    pub fn check(&mut self) {}

    #[allow(unused_must_use)]
    pub fn new_line(&mut self) {
        self.check();

        self.emit_current_line();
        write!(self.out, "\n");
        self.indent_level = -1
    }

    /**
     * Any segment that starts with '+' or '-' can't have a break preceding it. Combine it with the
     * preceding segment. Note that this doesn't apply to the first segment.
     */
    #[allow(unused_must_use)]
    fn fold_unsafe_breaks(&mut self) {
        let mut i = 1;
        while i < self.segments.len() {
            let segment = &self.segments[i];
            if UNSAFE_LINE_START.is_match(segment) {
                write!(self.segments[i - 1], " ");
                let next = self.segments[i].clone();
                write!(self.segments[i - 1], "{}", next);
                self.segments.remove(i);
                if i > 1 {
                    i = i - 1;
                }
            } else {
                i = i + 1;
            }
        }
    }
    fn emit_current_line(&mut self) {
        self.fold_unsafe_breaks();

        let mut start = 0;
        let mut column_count = self.segments[0].len() as i32;

        for i in 1..self.segments.len() {
            let segment = &self.segments[i];
            let current_length = segment.len() as i32;

            let new_column_count = column_count + 1 + segment.len() as i32;
            if new_column_count > self.column_limit {
                self.emit_segment_range(start, i as i32);
                start = i as i32;
                column_count = current_length + self.indent.len() as i32 * self.indent_level;
                continue;
            }

            column_count = new_column_count;
        }

        self.emit_segment_range(start, self.segments.len() as i32);

        self.segments.clear();
        self.segments.push("".to_string());
    }

    #[allow(unused_must_use)]
    pub fn emit_segment_range(&mut self, start_index: i32, end_index: i32) {
        if start_index > 0 {
            write!(self.out, "\n");
            for _i in 0..self.indent_level {
                write!(self.out, "{}", self.indent);
            }
            write!(self.out, "{}", self.line_prefix);
        }

        write!(self.out, "{}", self.segments[start_index as usize]);
        for i in start_index + 1..end_index {
            write!(self.out, " ");
            write!(self.out, "{}", self.segments[i as usize]);
        }
    }

    pub fn close(&mut self) {
        self.emit_current_line();
        self.closed = true;
    }

    pub fn check_closed() {}
}

#[cfg(test)]
mod tests {
    use crate::poet::line_wrapper::LineWrapper;

    // TODO: after log for articles
    // #[test]
    // fn should_build_line_wrappers() {
    //     let chars: Vec<char> = "hello ".chars().collect();
    //     assert_eq!(5, LineWrapper::index_of_any(&*chars, SPECIAL_CHARACTERS, 0));
    //     assert_eq!(5, LineWrapper::index_of_any(&*chars, SPECIAL_CHARACTERS, 5));
    //
    //     let empty: Vec<char> = "hello".chars().collect();
    //     assert_eq!(
    //         -1,
    //         LineWrapper::index_of_any(&*empty, SPECIAL_CHARACTERS, 0)
    //     );
    // }
    //
    // #[test]
    // fn should_get_pending_segments() {
    //     let mut out = String::new();
    //     let mut wrapper = LineWrapper::new(&mut out, String::from(""), 0);
    //     assert_eq!(false, wrapper.has_pending_segments());
    //
    //     wrapper.append("hello ".to_string(), None, None);
    //     wrapper.new_line();
    //     wrapper.close();
    //
    //     assert_eq!("hello\n\n", out);
    // }

    #[test]
    fn wrap() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("abcde fghij"), Some(2), None);
        wrapper.close();

        assert_eq!("abcde\n    fghij", out);
    }

    #[test]
    fn nowrap() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("abcde fghi"), Some(2), None);
        wrapper.close();

        assert_eq!("abcde fghi", out);
    }

    #[test]
    fn multiple_write() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("ab cd ef gh ij kl mn op qr"), Some(1), None);
        wrapper.close();

        assert_eq!("ab cd ef\n  gh ij kl\n  mn op qr", out);
    }

    #[test]
    fn fencepost() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("abcde"), Some(2), None);
        wrapper.append(String::from("fghij k"), Some(2), None);
        wrapper.append(String::from("lmnop"), Some(2), None);
        wrapper.close();

        assert_eq!("abcdefghij\n    klmnop", out);
    }

    #[test]
    fn overly_long_lines_without_leading_space() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("abcdefghijkl"), Some(2), None);
        wrapper.close();

        assert_eq!("abcdefghijkl", out);
    }

    #[test]
    fn overly_long_lines_with_leading_space() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from(" abcdefghijkl"), Some(2), None);
        wrapper.close();

        assert_eq!("\n    abcdefghijkl", out);
    }

    #[test]
    fn no_wrap_embedded_newlines() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("abcde fghi\njklmn"), Some(2), None);
        wrapper.append(String::from("opqrstuvwxy"), Some(2), None);
        wrapper.close();

        assert_eq!("abcde fghi\njklmnopqrstuvwxy", out);
    }

    #[test]
    fn wrap_embedded_newlines() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("abcde fghij\nklmn"), Some(2), None);
        wrapper.append(String::from("opqrstuvwxy"), Some(2), None);
        wrapper.close();

        assert_eq!("abcde\n    fghij\nklmnopqrstuvwxy", out);
    }

    #[test]
    fn no_wrap_multiple_newlines() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(
            String::from("abcde fghi\nklmnopq\nr stuvwxyz"),
            Some(2),
            None,
        );
        wrapper.close();

        assert_eq!("abcde fghi\nklmnopq\nr stuvwxyz", out);
    }

    #[test]
    fn wrap_multiple_newlines() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(
            String::from("abcde fghi\nklmnopq\nrs tuvwxyz1"),
            Some(2),
            None,
        );
        wrapper.close();

        assert_eq!("abcde fghi\nklmnopq\nrs\n    tuvwxyz1", out);
    }

    #[test]
    fn no_wrap_preceding_unary_plus() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("a - b       - c"), Some(2), None);
        wrapper.close();

        assert_eq!("a - b     \n     - c", out);
    }

    #[test]
    fn append_non_wrapping() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("ab cd ef"), Some(2), None);
        wrapper.append_non_wrapping(String::from("gh ij kl mn"));
        wrapper.close();

        assert_eq!("ab cd\n    efgh ij kl mn", out);
    }

    #[test]
    fn append_non_wrapping_space() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("ab cd ef"), Some(2), None);
        wrapper.append(String::from("gh·ij·kl·mn"), Some(2), None);
        wrapper.close();

        assert_eq!("ab cd\n    efgh ij kl mn", out);
    }

    #[test]
    fn lone_unsafe_unary_operator() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from(" -1"), Some(2), None);
        wrapper.close();

        assert_eq!(" -1", out);
    }

    #[test]
    fn line_prefix() {
        let mut out = String::new();
        let mut wrapper = LineWrapper::new(&mut out, String::from("  "), 10);
        wrapper.append(String::from("/**\n"), None, None);
        wrapper.append(String::from(" * "), None, None);
        wrapper.append(
            String::from("a b c d e f g h i j k l m n\n"),
            None,
            Some(String::from(" * ")),
        );
        wrapper.append(String::from(" */"), None, None);
        wrapper.close();

        assert_eq!("/**\n * a b c d\n * e f g h i j\n * k l m n\n */"", out);
    }
}
