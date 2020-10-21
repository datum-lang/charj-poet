#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseOperator {
    pub operator: &'static str,
    pub function_name: &'static str,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Operator {
    UNARY_PLUS,
    PLUS,
    UNARY_MINUS,
    MINUS,
    TIMES,
    DIV,
    REM,
    PLUS_ASSIGN,
    MINUS_ASSIGN,
    TIMES_ASSIGN,
    DIV_ASSIGN,
    REM_ASSIGN,
    INC,
    DEC,
    EQUALS,
    NOT_EQUALS,
    NOT,
    RANGE_TO,
    CONTAINS,
    NOT_CONTAINS,
    GT,
    LT,
    GE,
    LE,
    ITERATOR,
}

impl Operator {
    pub const UNARY_PLUS_OPS: BaseOperator = BaseOperator { operator: "+", function_name: "unaryPlus" };
    pub const PLUS_OPS: BaseOperator = BaseOperator { operator: "+", function_name: "plus" };

    fn value(&self) -> BaseOperator {
        match *self {
            Operator::UNARY_PLUS => { Operator::UNARY_PLUS_OPS }
            Operator::PLUS => { Operator::PLUS_OPS }
            // Operator::UNARY_MINUS => {}
            // Operator::MINUS => {}
            // Operator::TIMES => {}
            // Operator::DIV => {}
            // Operator::REM => {}
            // Operator::PLUS_ASSIGN => {}
            // Operator::MINUS_ASSIGN => {}
            // Operator::TIMES_ASSIGN => {}
            // Operator::DIV_ASSIGN => {}
            // Operator::REM_ASSIGN => {}
            // Operator::INC => {}
            // Operator::DEC => {}
            // Operator::EQUALS => {}
            // Operator::NOT_EQUALS => {}
            // Operator::NOT => {}
            // Operator::RANGE_TO => {}
            // Operator::CONTAINS => {}
            // Operator::NOT_CONTAINS => {}
            // Operator::GT => {}
            // Operator::LT => {}
            // Operator::GE => {}
            // Operator::LE => {}
            // Operator::ITERATOR => {}
            _ => {
                BaseOperator {
                    operator: "",
                    function_name: "",
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::operator::Operator;

    #[test]
    fn should_enum() {
        let operator = Operator::NOT_EQUALS;
        println!("{:?}", operator.value());
    }
}