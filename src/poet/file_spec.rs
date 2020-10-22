use crate::poet::code_writer::CodeWriter;
use crate::poet::DEFAULT_INDENT;
use core::fmt;

///
/// A Kotlin file containing top level objects like classes, objects, functions, properties, and type
/// aliases.
///
/// Items are output in the following order:
/// - Comment
/// - Annotations
/// - Package
/// - Imports
/// - Members
///
#[derive(Serialize, Debug)]
pub struct FileSpec {
    package_name: &'static str,
    name: &'static str,
    indent: &'static str,
}

impl FileSpec {
    pub fn new(builder: &mut FileSpecBuilder) -> Self {
        FileSpec {
            package_name: builder.package_name,
            name: builder.name,
            indent: builder.indent,
        }
    }

    pub fn write_to(&self, _fmt: &mut fmt::Formatter) {
        // todo: add logic
        // First pass: emit the entire class, just to collect the types we'll need to import.

        // Second pass: write the code, taking advantage of the imports.
        let mut out = "".to_string();
        let mut writer = CodeWriter::new(&mut out, self.indent);
        self.emit(&mut writer);
        writer.close();
    }

    pub fn emit(&self, writer: &mut CodeWriter) {
        writer.push_package(self.package_name);
        writer.emit_code("package·%L\n", self.package_name.to_string());
        // writer.emit("\n");
    }
}

impl fmt::Display for FileSpec {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        &self.write_to(fmt);
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct FileSpecBuilder {
    package_name: &'static str,
    name: &'static str,
    indent: &'static str,
}

impl FileSpecBuilder {
    pub fn new(package_name: &'static str, file_name: &'static str) -> Self {
        FileSpecBuilder {
            package_name,
            name: file_name,
            indent: DEFAULT_INDENT,
        }
    }

    pub fn add_file_comment(&self, _format: String) {}

    pub fn add_import(&self) {}

    pub fn add_type_alias(&self) {}

    pub fn build(&mut self) -> FileSpec {
        FileSpec::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::file_spec::FileSpecBuilder;

    #[test]
    fn should_call_builder() {
        let mut builder = FileSpecBuilder::new("com.phodal", "HelloWorld");
        let spec = builder.build();
        println!("{:?}", spec);
        assert_eq!("com.phodal", spec.package_name);
    }
}
