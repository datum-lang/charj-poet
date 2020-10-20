use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct BaseModifier {
    pub keyword: &'static str,
    pub targets: HashSet<Target>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Target {
    CLASS,
    VARIANCE_ANNOTATION,
    PARAMETER,
    TYPE_PARAMETER,
    FUNCTION,
    PROPERTY,
    INTERFACE,
}

#[derive(Clone, Debug)]
pub struct PublicModifier {
    pub modifier: BaseModifier
}

impl Default for PublicModifier {
    fn default() -> Self {
        let mut tragets = HashSet::new();
        tragets.insert(Target::PROPERTY);

        PublicModifier {
            modifier: BaseModifier {
                keyword: "public",
                targets: tragets
            }
        }
    }
}

pub enum Modifier {
    PUBLIC(PublicModifier),
    PROTECTED(),
    PRIVATE(),
    INTERNAL(),
}