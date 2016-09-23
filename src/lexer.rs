use op::{OpCode, OpError};

pub struct Lexer {
    // TODO: Hold some error handling info here
    // line number, char number, etc.
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {}
    }

    pub fn lex(&self, line: &String) -> Result<OpCode, OpError> {
        let op_vec: Vec<&str> = line.split(' ').collect();
        if op_vec.is_empty() {
            return Ok(OpCode::NOP);
        }

        // TODO: More general handling of getting args
        match op_vec[0] {
            "PRINT" => Ok(OpCode::PRINT(op_vec[1].to_owned())),
            "HALT" => Ok(OpCode::HALT),
            "NOP" => Ok(OpCode::NOP),
            "ADD" => Ok(OpCode::ADD),
            "SUB" => Ok(OpCode::SUB),
            "MUL" => Ok(OpCode::MUL),
            "DIV" => Ok(OpCode::DIV),
            "MOD" => Ok(OpCode::MOD),
            "AND" => Ok(OpCode::AND),
            "OR" => Ok(OpCode::OR),
            "NEG" => Ok(OpCode::NEG),
            "LOADC" => {
                let arg = try!(self.extract_arg(&op_vec));
                Ok(OpCode::LOADC(arg))
            },
            _ => panic!("tyr: Invalid operation \'{:?}\' specified", op_vec[0])
        }
    }

    fn extract_arg(&self, op_vec: &Vec<&str>) -> Result<i64, OpError> {
        if op_vec.len() < 2 {
            panic!("tyr: Missing arguments for load operation {:?}.", op_vec[0]);
        }

        let arg = try!(op_vec[1].parse::<i64>());

        Ok(arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use op::OpCode;

    #[test]
    fn lex_print() {
        let prog = "PRINT test".to_string();
        let expected = OpCode::PRINT("test".to_string());
        let lexer = Lexer::new();

        let result = lexer.lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn lex_halt() {
        let prog = "HALT".to_string();
        let expected = OpCode::HALT;
        let lexer = Lexer::new();

        let result = lexer.lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn lex_nop() {
        let prog = "NOP".to_string();
        let expected = OpCode::NOP;
        let lexer = Lexer::new();

        let result = lexer.lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn lex_add() {
        let prog = "ADD".to_string();
        let expected = OpCode::ADD;
        let lexer = Lexer::new();

        let result = lexer.lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn lex_loadc() {
        let prog = "LOADC 5".to_string();
        let expected = OpCode::LOADC(5);
        let lexer = Lexer::new();

        let result = lexer.lex(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic(expected = "tyr: Invalid operation")]
    fn lex_illegal_op() {
        let prog = "TEST".to_string();
        let lexer = Lexer::new();

        lexer.lex(&prog).ok();
    }

    #[test]
    fn lex_illegal_arg() {
        let prog = "LOADC h".to_string();
        let lexer = Lexer::new();

        let result = lexer.lex(&prog);
        assert_eq!(result.is_ok(), false)
    }
}
