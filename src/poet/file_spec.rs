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
}

impl FileSpec {
    pub fn new(builder: &mut FileSpecBuilder) -> Self {
        FileSpec {
            package_name: builder.package_name,
            name: builder.name,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FileSpecBuilder {
    package_name: &'static str,
    name: &'static str,
}

impl FileSpecBuilder {
    pub fn new(package_name: &'static str, file_name: &'static str) -> Self {
        FileSpecBuilder {
            package_name,
            name: file_name,
        }
    }

    pub fn add_file_comment(&self, _format: String) {}

    pub fn add_import(&self) {}

    pub fn add_type_alias(&self) {}

    pub fn build(&mut self) -> FileSpec {
        let file_spec = FileSpec::new(self);
        file_spec
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::file_spec::FileSpecBuilder;

    #[test]
    fn should_call_builder() {
        let mut builder = FileSpecBuilder::new("com.phodal", "hello");
        let spec = builder.build();
        assert_eq!("com.phodal", spec.package_name);
    }
}
