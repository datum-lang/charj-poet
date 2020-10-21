use std::collections::HashMap;
use crate::poet::import::Import;
use crate::poet::class_name::StructName;
use crate::poet::member_name::MemberName;

pub const DEFAULT_INDENT: &'static str = "  ";
///
/// Converts a [FileSpec] to a string suitable to both human- and kotlinc-consumption. This honors
/// imports, indentation, and deferred variable names.
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CodeWriter {
    pub out: String,
    pub ident: &'static str,
    pub imports: HashMap<String, Import>,
    pub import_types: HashMap<String, StructName>,
    pub import_members: HashMap<String, MemberName>,
    pub column_limit: i32,
    pub indent_level: i32,
    pub statement_line: i32
}

impl CodeWriter {
    pub fn new(ident: &'static str) -> Self {
        CodeWriter {
            out: "".to_string(),
            ident,
            imports: Default::default(),
            import_types: Default::default(),
            import_members: Default::default(),
            column_limit: 100,
            indent_level: 0,
            statement_line: -1
        }
    }
}
