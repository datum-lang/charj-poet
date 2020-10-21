use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct BaseModifier {
    pub keyword: &'static str,
    // todo: check different behavior for hashset & vector
    pub targets: Vec<Target>,
}

#[allow(non_camel_case_types)]
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
        PublicModifier {
            modifier: BaseModifier {
                keyword: "public",
                targets: [Target::PROPERTY].iter().cloned().collect(),
            }
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Modifier {
    PUBLIC,
    PROTECTED,
    PRIVATE,
    INTERNAL,
}

impl Modifier {
    fn value(&self) -> BaseModifier {
        match *self {
            Modifier::PUBLIC => {
                BaseModifier {
                    keyword: "public",
                    targets: vec![Target::PROPERTY],
                }
            }
            _ => {
                BaseModifier { keyword: "", targets: Default::default() }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::operator::Operator;
    use crate::poet::modifier::Modifier;

    #[test]
    fn should_enum() {
        let public = Modifier::PUBLIC;
        println!("{:?}", public.value());
    }
}
