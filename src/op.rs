use std::num;

#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    PRINT(String),
    LOADC(i64),
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    AND,
    OR,
    NEG,
    HALT,
    NOP
}

#[derive(Clone, Debug)]
pub enum OpError {
    Parse(num::ParseIntError)
}

impl From<num::ParseIntError> for OpError {
    fn from(err: num::ParseIntError) -> OpError {
        OpError::Parse(err)
    }
}
