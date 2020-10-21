use crate::poet::modifier::Modifier;
use std::collections::HashSet;

/** A generated class, interface, or enum declaration. */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypeSpec {}

pub struct BaseKind {
    pub declaration_keyword: &'static String,
    pub property_modifiers: HashSet<Modifier>,
    pub function_modifiers: HashSet<Modifier>,
    pub type_modifiers: HashSet<Modifier>,
}

pub struct ClassKind {
    pub kind: BaseKind,
}

pub struct ObjectKind {
    pub kind: BaseKind,
}

pub struct InterfaceKind {
    pub kind: BaseKind,
}

pub enum Kind {
    Class(ClassKind),
    Object(ObjectKind),
    Interface(InterfaceKind),
}
