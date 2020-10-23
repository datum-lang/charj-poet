use crate::poet::code_writer::CodeWriter;
use crate::poet::{index_of, DEFAULT_INDENT};
use core::fmt;
use serde::export::Formatter;

///
/// A fragment of a .kt file, potentially containing declarations, statements, and documentation.
/// Code blocks are not necessarily well-formed Kotlin code, and are not validated. This class
/// assumes kotlinc will check correctness later!
///
/// Code blocks support placeholders like [java.text.Format]. This class primarily uses a percent
/// sign `%` but has its own set of permitted placeholders:
///
/// * `%L` emits a *literal* value with no escaping. Arguments for literals may be strings,
///   primitives, [type declarations][TypeSpec], [annotations][AnnotationSpec] and even other code
///   blocks.
/// * `%N` emits a *name*, using name collision avoidance where necessary. Arguments for names may
///   be strings (actually any [character sequence][CharSequence]), [parameters][ParameterSpec],
///   [properties][PropertySpec], [functions][FunSpec], and [types][TypeSpec].
/// * `%S` escapes the value as a *string*, wraps it with double quotes, and emits that. For
///   example, `6" sandwich` is emitted `"6\" sandwich"`. `%S` will also escape all dollar signs
///   (`$`), use `%P` for string templates.
/// * `%P` - Similar to `%S`, but doesn't escape dollar signs (`$`) to allow creation of string
///   templates. If the string contains dollar signs that should be escaped - use `%S`.
/// * `%T` emits a *type* reference. Types will be imported if possible. Arguments for types may be
///   [classes][Class].
/// * `%M` emits a *member* reference. A member is either a function or a property. If the member is
///   importable, e.g. it's a top-level function or a property declared inside an object, the import
///   will be resolved if possible. Arguments for members must be of type [MemberName].
/// * `%%` emits a percent sign.
/// * `·` emits a space that never wraps. KotlinPoet prefers to wrap lines longer than 100 columns.
///   It does this by replacing normal spaces with a newline and indent. Note that spaces in strings
///   are never wrapped.
/// * `⇥` increases the indentation level.
/// * `⇤` decreases the indentation level.
/// * `«` begins a statement. For multiline statements, every line after the first line is
///   double-indented.
/// * `»` ends a statement.
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CodeBlock {
    pub format_parts: Vec<String>,
    pub args: Vec<String>,
}

impl CodeBlock {
    pub fn new(builder: &CodeBlockBuilder) -> Self {
        CodeBlock {
            format_parts: builder.format_parts.clone(),
            args: builder.args.clone(),
        }
    }

    pub fn of(format: &str, args: Vec<String>) -> CodeBlock {
        let mut builder: CodeBlockBuilder = CodeBlockBuilder::new();
        builder.add(format, args);
        builder.build()
    }
}

impl fmt::Display for CodeBlock {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let mut out = "".to_string();
        CodeWriter::new(&mut out, DEFAULT_INDENT);
        Ok(())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CodeBlockBuilder {
    pub format_parts: Vec<String>,
    pub args: Vec<String>,
}

impl CodeBlockBuilder {
    pub fn new() -> Self {
        CodeBlockBuilder {
            format_parts: vec![],
            args: vec![],
        }
    }

    pub fn build(&self) -> CodeBlock {
        return CodeBlock::new(self);
    }

    ///
    /// @param control_flow the control flow construct and its code, such as "if (foo == 5)".
    /// Shouldn't contain braces or newline characters.
    ///
    pub fn next_control_flow(&mut self, _control_flow: &'static str, _args: String) {}

    pub fn end_control_flow_none(&mut self) {}

    ///
    /// @param controlFlow the optional control flow construct and its code, such as
    ///     "while(foo == 20)". Only used for "do/while" control flows.
    ///
    pub fn end_control_flow(&mut self, _control_flow: &'static str, _args: String) {}

    pub fn add_statement(&mut self, _control_flow: &'static str, _args: String) {}

    ///
    /// Add code with positional or relative arguments.
    ///
    /// Relative arguments map 1:1 with the placeholders in the format string.
    ///
    /// Positional arguments use an index after the placeholder to identify which argument index
    /// to use. For example, for a literal to reference the 3rd argument: "%3L" (1 based index)
    ///
    /// Mixing relative and positional arguments in a call to add is invalid and will result in an
    /// error.
    ///
    pub fn add(&mut self, format: &str, args: Vec<String>) -> &mut CodeBlockBuilder {
        let mut has_relative: bool = false;
        let mut has_indexed: bool = false;
        let mut relative_parameter_count: i32 = 0;

        let mut indexed_parameter_count: Vec<i32> = Vec::with_capacity(args.len());

        let chars: Vec<char> = format.chars().collect();
        for mut p in 0..format.len() {
            if chars[p] != '$' {
                let mut next_p = index_of(&chars, p);
                if next_p == -1 {
                    next_p = format.len() as i32;
                }
                let x: String = format.chars().skip(p).take(next_p as usize - p).collect();
                self.format_parts.push(x);
                continue;
            }

            p = p + 1; // '$'.
            let index_start = p.clone();

            p = p + 1;
            let c: char = chars[p];

            let index_end = p - 1;

            let mut index = 0;
            if index_start > index_end {
                index = relative_parameter_count;
                has_relative = true;
                relative_parameter_count = relative_parameter_count + 1;
            }

            self.add_argument(format, c, args[index as usize].clone());
            self.format_parts
                .push(CodeBlockBuilder::merge_str_c("$", c));
        }
        self
    }

    fn merge_str_c(s: &str, c: char) -> String {
        let mut string = String::from(s);
        string.push_str(&*c.to_string());
        string
    }

    pub fn add_argument(&mut self, format: &str, c: char, arg: String) {
        match c {
            'L' => {
                self.args.push(self.arg_to_literal(arg));
            }
            _ => println!("invalid format string: {:?}", format),
        }
    }

    pub fn arg_to_literal(&self, arg: String) -> String {
        return arg;
    }

    pub fn unindent(&mut self) -> &mut CodeBlockBuilder {
        self.format_parts.push(String::from("$<"));
        self
    }

    pub fn indent<'a>(&mut self) -> &mut CodeBlockBuilder {
        self.format_parts.push(String::from("$>"));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::code_block::CodeBlock;

    #[test]
    fn of() {
        let code_block = CodeBlock::of("$L taco", vec![String::from("delicious")]);
        println!("{}", code_block);
    }
}
