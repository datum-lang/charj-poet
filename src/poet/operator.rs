pub struct BaseOperator {
    pub operator: &'static str,
    pub function_name: &'static str,
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Operator {
    UNARY_PLUS(),
    PLUS(),
    UNARY_MINUS(),
    MINUS(),
    TIMES(),
    DIV(),
    REM(),
    PLUS_ASSIGN(),
    MINUS_ASSIGN(),
    TIMES_ASSIGN(),
    DIV_ASSIGN(),
    REM_ASSIGN(),
    INC(),
    DEC(),
    EQUALS(),
    NOT_EQUALS(),
    NOT(),
    RANGE_TO(),
    CONTAINS(),
    NOT_CONTAINS(),
    GT(),
    LT(),
    GE(),
    LE(),
    ITERATOR(),
}