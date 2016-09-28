use std::num::ParseIntError;

#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    PRINT(String),
    LOADC(i64),
    LABEL(String, usize),
    LOAD,
    STORE,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    AND,
    OR,
    NEG,
    HALT,
    JMP(String),
    JMPZ(String),
    NOP
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpError {
    Parse(ParseIntError),
    Label(String)
}

impl From<ParseIntError> for OpError {
    fn from(err: ParseIntError) -> OpError {
        OpError::Parse(err)
    }
}
