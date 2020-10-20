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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileSpec {
    builder: FileSpecBuilder
}

impl FileSpec {
    pub fn new(builder: FileSpecBuilder) -> Self {
        FileSpec {
            builder
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileSpecBuilder {
    package_name: String,
    name: String,
}

impl FileSpecBuilder {
    pub fn new(package_name: String, name: String) -> Self {
        FileSpecBuilder { package_name, name }
    }

    pub fn add_file_comment(&self, _format: String) {}

    pub fn add_import(&self) {}

    pub fn add_type_alias(&self) {}

    pub fn build(&self) -> FileSpec {
        let file_spec = FileSpec::new(self.clone());
        file_spec
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::file_spec::FileSpecBuilder;

    #[test]
    fn should_call_builder() {
        FileSpecBuilder::new(String::from("com.phodal"), String::from("hello"));
    }
}
