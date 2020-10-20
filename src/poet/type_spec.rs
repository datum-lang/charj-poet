/** A generated class, interface, or enum declaration. */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypeSpec {

}

pub struct ClassKind {}
pub struct ObjectKind {}
pub struct InterfaceKind {}

pub enum Kind {
    Class(ClassKind),
    Object(ObjectKind),
    Interface(InterfaceKind)
}