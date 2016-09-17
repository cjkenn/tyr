#[derive(Clone)]
pub enum OpCode {
    PRINT(String),
    ADD(i64, i64),
    SUB(i64, i64),
    MUL(i64, i64),
    DIV(i64, i64),
    HALT
}
