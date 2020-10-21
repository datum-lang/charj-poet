use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct BaseModifier {
    pub keyword: &'static str,
    pub targets: HashSet<Target>,
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
    // pub const PUB_TARGET: HashSet<Target> = [Target::PROPERTY].iter().cloned().collect();
    // pub const PUBLIC_VAL: BaseModifier  = BaseModifier {
    //     keyword: "public",
    //     targets: Modifier::PUB_TARGET,
    // };

    fn value(&self) -> BaseModifier {
        match *self {
            _ => {
                BaseModifier {
                    keyword: "",
                    targets: Default::default()
                }
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
