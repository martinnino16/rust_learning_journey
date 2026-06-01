
pub enum CalcError {
    InvalidArity,
    UnknownOperation(String),
    ParseError(String),
    DivisionByZero,
    InvalidInput,
}
