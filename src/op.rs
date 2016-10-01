use std::num::ParseIntError;

/// Contains the name of each operation that can be performed
/// by the vm. Any operations that require arguments to
/// be executed can be passed to the enum constructor.
#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    PRINT(String),
    LOADC(i64),
    LOADV(i64),
    LABEL(String, usize),
    LOAD,
    STORE,
    STOREV(i64),
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
    JMPI(i64),
    DUP,
    NOP
}

/// OpErrors are used by methods in the vm and parser that
/// need to return a Result type.
///
/// Parse: Encountered when trying to parse a string to an int.
/// Lavel: Encountered when trying to jmp to or parse a label.
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
