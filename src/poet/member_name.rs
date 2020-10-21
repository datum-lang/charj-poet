use crate::poet::class_name::StructName;

/**
 * Represents the name of a member (such as a function or a property).
 *
 * @param packageName e.g. `kotlin.collections`
 * @param enclosingClassName e.g. `Map.Entry.Companion`, if the member is declared inside the
 * companion object of the Map.Entry class
 * @param simpleName e.g. `isBlank`, `size`
 */
#[derive(Serialize, Clone, Debug)]
pub struct MemberName {
    pub package_name: String,
    pub enclosing_class_name: Option<StructName>,
    pub simple_name: String,
    pub operator: Option<String>
}

impl MemberName {}
