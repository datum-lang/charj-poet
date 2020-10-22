use crate::poet::class_name::StructName;
use crate::poet::code_block::CodeBlock;
use crate::poet::import::Import;
use crate::poet::line_wrapper::LineWrapper;
use crate::poet::member_name::MemberName;
use std::collections::HashMap;

pub const NO_PACKAGE: &'static str = "";
///
/// Converts a [FileSpec] to a string suitable to both human- and kotlinc-consumption. This honors
/// imports, indentation, and deferred variable names.
///
#[derive(Serialize, Debug)]
pub struct CodeWriter<'a> {
    pub out: Box<LineWrapper<'a>>,
    pub indent: &'static str,
    pub package_name: String,
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
            indent: ident,
            package_name: String::from(NO_PACKAGE),
            imports: Default::default(),
            import_types: Default::default(),
            import_members: Default::default(),
            column_limit: 100,
            indent_level: 0,
            statement_line: -1,
        }
    }

    pub fn emit_code(&mut self, format: &str, args: Vec<String>) {
        CodeBlock::of(format, args);
    }

    pub fn emit(&mut self, s: String) {}

    fn _emit_code(code_block: &mut CodeBlock, format: &str) {}
    pub fn push_package(&mut self, package_name: &str) {
        self.package_name = String::from(package_name);
    }

    pub fn close(&mut self) {
        self.out.close();
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::code_writer::CodeWriter;
    use crate::poet::DEFAULT_INDENT;

    #[test]
    fn init() {
        let mut out = "hello".to_string();
        let mut writer = CodeWriter::new(&mut out, DEFAULT_INDENT);
        writer.out.append(String::from(" zero"), None, None);
        writer.out.close();
        assert_eq!("hello zero", out);
    }
}
