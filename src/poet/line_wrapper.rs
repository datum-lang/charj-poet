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
}
