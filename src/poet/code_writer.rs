use crate::poet::class_name::StructName;
use crate::poet::import::Import;
use crate::poet::line_wrapper::LineWrapper;
use crate::poet::member_name::MemberName;
use std::collections::HashMap;

pub const DEFAULT_INDENT: &'static str = "  ";
///
/// Converts a [FileSpec] to a string suitable to both human- and kotlinc-consumption. This honors
/// imports, indentation, and deferred variable names.
///
#[derive(Serialize, Debug)]
pub struct CodeWriter<'a> {
    pub out: Box<LineWrapper<'a>>,
    pub ident: &'static str,
    pub imports: HashMap<String, Import>,
    pub import_types: HashMap<String, StructName>,
    pub import_members: HashMap<String, MemberName>,
    pub column_limit: i32,
    pub indent_level: i32,
    pub statement_line: i32,
}

impl<'a> CodeWriter<'a> {
    pub fn new(out: &'a mut String, ident: &'static str) -> Self {
        let line_wrapper = LineWrapper::new(out, String::from(ident), 100);
        let wrapper = Box::new(line_wrapper);
        CodeWriter {
            out: wrapper,
            ident,
            imports: Default::default(),
            import_types: Default::default(),
            import_members: Default::default(),
            column_limit: 100,
            indent_level: 0,
            statement_line: -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::code_writer::{CodeWriter, DEFAULT_INDENT};

    #[test]
    fn init() {
        let mut out = "hello".to_string();
        let mut writer = CodeWriter::new(&mut out, DEFAULT_INDENT);
        writer.out.append(String::from(" zero"), None, None);
        writer.out.close();
        assert_eq!("hello zero", out);
    }
}
