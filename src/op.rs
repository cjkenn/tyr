use std::num;

#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    PRINT(String),
    LOADC(i64),
    ADD,
    SUB,
    MUL,
    DIV,
    HALT,
    NOP
}

// TODO: Move to separate error module?
#[derive(Clone, Debug)]
pub enum OpError {
    Parse(num::ParseIntError)
}

impl From<num::ParseIntError> for OpError {
    fn from(err: num::ParseIntError) -> OpError {
        OpError::Parse(err)
    }
}

pub fn lex(op_vec: &Vec<&str>) -> Result<OpCode, OpError> {
    // TODO: More general handling of getting args
    match op_vec[0] {
        "PRINT" => Ok(OpCode::PRINT(op_vec[1].to_owned())),
        "HALT" => Ok(OpCode::HALT),
        "NOP" => Ok(OpCode::NOP),
        "ADD" => Ok(OpCode::ADD),
        "SUB" => Ok(OpCode::SUB),
        "MUL" => Ok(OpCode::MUL),
        "DIV" => Ok(OpCode::DIV),
        "LOADC" => {
            let arg = try!(extract_arg(op_vec));
            Ok(OpCode::LOADC(arg))
        },
        _ => panic!("tyr: Invalid operation \'{:?}\' specified", op_vec[0])
    }
}

fn extract_arg(op_vec: &Vec<&str>) -> Result<i64, OpError> {
    if op_vec.len() < 2 {
        panic!("tyr: Missing arguments for load operation {:?}.", op_vec[0]);
    }

    let arg = try!(op_vec[1].parse::<i64>());

    Ok(arg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_print() {
        let prog = vec!["PRINT", "test"];
        let expected = OpCode::PRINT("test".to_owned());

        let result = lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn lex_halt() {
        let prog = vec!["HALT"];
        let expected = OpCode::HALT;

        let result = lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn lex_nop() {
        let prog = vec!["NOP"];
        let expected = OpCode::NOP;

        let result = lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn lex_add() {
        let prog = vec!["ADD"];
        let expected = OpCode::ADD;

        let result = lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn lex_loadc() {
        let prog = vec!["LOADC", "5"];
        let expected = OpCode::LOADC(5);

        let result = lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic(expected = "tyr: Invalid operation")]
    fn lex_illegal_op() {
        let prog = vec!["TEST"];
        lex(&prog).ok();
    }

    #[test]
    #[should_panic]
    fn lex_illegal_arg() {
        let prog = vec!["LOADC", "h"];
        lex(&prog).ok();
    }
}
