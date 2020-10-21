#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseOperator {
    pub operator: &'static str,
    pub function_name: &'static str,
}

#[allow(non_camel_case_types)]
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
    pub const UNARY_MINUS_OPS: BaseOperator = BaseOperator { operator: "-", function_name: "unaryMinus" };
    pub const MINUS_OPS: BaseOperator = BaseOperator { operator: "-", function_name: "minus" };
    pub const TIMES_OPS: BaseOperator = BaseOperator { operator: "*", function_name: "times" };
    pub const DIV_OPS: BaseOperator = BaseOperator { operator: "/", function_name: "div" };
    pub const REM_OPS: BaseOperator = BaseOperator { operator: "%", function_name: "rem" };
    pub const PLUS_ASSIGN_OPS: BaseOperator = BaseOperator { operator: "+=", function_name: "plusAssign" };
    pub const MINUS_ASSIGN_OPS: BaseOperator = BaseOperator { operator: "-=", function_name: "minusAssign" };
    pub const TIMES_ASSIGN_OPS: BaseOperator = BaseOperator { operator: "*=", function_name: "timesAssign" };
    pub const DIV_ASSIGN_OPS: BaseOperator = BaseOperator { operator: "/=", function_name: "divAssign" };
    pub const REM_ASSIGN_OPS: BaseOperator = BaseOperator { operator: "%=", function_name: "remAssign" };
    pub const INC_OPS: BaseOperator = BaseOperator { operator: "++", function_name: "inc" };
    pub const DEC_OPS: BaseOperator = BaseOperator { operator: "--", function_name: "dec" };
    pub const EQUALS_OPS: BaseOperator = BaseOperator { operator: "==", function_name: "equals" };
    pub const NOT_EQUALS_OPS: BaseOperator = BaseOperator { operator: "!=", function_name: "equals" };
    pub const NOT_OPS: BaseOperator = BaseOperator { operator: "!", function_name: "not" };
    pub const RANGE_TO_OPS: BaseOperator = BaseOperator { operator: "..", function_name: "rangeTo" };
    pub const CONTAINS_OPS: BaseOperator = BaseOperator { operator: "in", function_name: "contains" };
    pub const NOT_CONTAINS_OPS: BaseOperator = BaseOperator { operator: "!in", function_name: "contains" };
    pub const GT_OPS: BaseOperator = BaseOperator { operator: ">", function_name: "compareTo" };
    pub const LT_OPS: BaseOperator = BaseOperator { operator: "<", function_name: "compareTo" };
    pub const GE_OPS: BaseOperator = BaseOperator { operator: ">=", function_name: "compareTo" };
    pub const LE_OPS: BaseOperator = BaseOperator { operator: "<=", function_name: "compareTo" };
    pub const ITERATOR_OPS: BaseOperator = BaseOperator { operator: "in", function_name: "iterator" };

    fn value(&self) -> BaseOperator {
        match *self {
            Operator::UNARY_PLUS => { Operator::UNARY_PLUS_OPS }
            Operator::PLUS => { Operator::PLUS_OPS }
            Operator::UNARY_MINUS => { Operator::UNARY_MINUS_OPS }
            Operator::MINUS => { Operator::MINUS_OPS }
            Operator::TIMES => { Operator::TIMES_OPS }
            Operator::DIV => { Operator::DIV_OPS }
            Operator::REM => { Operator::REM_OPS }
            Operator::PLUS_ASSIGN => { Operator::PLUS_ASSIGN_OPS }
            Operator::MINUS_ASSIGN => { Operator::MINUS_ASSIGN_OPS }
            Operator::TIMES_ASSIGN => { Operator::TIMES_ASSIGN_OPS }
            Operator::DIV_ASSIGN => { Operator::DIV_ASSIGN_OPS }
            Operator::REM_ASSIGN => { Operator::REM_ASSIGN_OPS }
            Operator::INC => { Operator::INC_OPS }
            Operator::DEC => { Operator::DEC_OPS }
            Operator::EQUALS => { Operator::EQUALS_OPS }
            Operator::NOT_EQUALS => { Operator::NOT_EQUALS_OPS }
            Operator::NOT => { Operator::NOT_OPS }
            Operator::RANGE_TO => { Operator::RANGE_TO_OPS }
            Operator::CONTAINS => { Operator::CONTAINS_OPS }
            Operator::NOT_CONTAINS => { Operator::NOT_CONTAINS_OPS }
            Operator::GT => { Operator::GT_OPS }
            Operator::LT => { Operator::LT_OPS }
            Operator::GE => { Operator::GE_OPS }
            Operator::LE => { Operator::LE_OPS }
            Operator::ITERATOR => { Operator::ITERATOR_OPS }
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