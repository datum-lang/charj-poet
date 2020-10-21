use crate::poet::annotation_spec::AnnotationSpec;
use crate::poet::code_block::CodeBlock;
use crate::poet::modifier::Modifier;
use crate::poet::parameter_spec::ParameterSpec;
use crate::poet::type_name::TypeName;

/** A generated constructor or method declaration. */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MethodSpec {
    pub name: String,
    pub modifiers: Vec<Modifier>,
    pub receiver_type: TypeName,
    pub return_type: TypeName,
    pub parameters: Vec<ParameterSpec>,
    pub annotations: Vec<AnnotationSpec>,
    pub code: CodeBlock,
}

impl MethodSpec {}
