use op::{OpCode, OpError};

pub struct Parser {
    // TODO: Hold some error handling info here
    // line number, char number, etc.
}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse_line(&self, line: &String) -> Result<OpCode, OpError> {
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
    fn parse_line_print() {
        let prog = "PRINT test".to_string();
        let expected = OpCode::PRINT("test".to_string());
        let parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_halt() {
        let prog = "HALT".to_string();
        let expected = OpCode::HALT;
        let parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_nop() {
        let prog = "NOP".to_string();
        let expected = OpCode::NOP;
        let parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_add() {
        let prog = "ADD".to_string();
        let expected = OpCode::ADD;
        let parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_loadc() {
        let prog = "LOADC 5".to_string();
        let expected = OpCode::LOADC(5);
        let parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic(expected = "tyr: Invalid operation")]
    fn parse_line_illegal_op() {
        let prog = "TEST".to_string();
        let parser = Parser::new();

        parser.parse_line(&prog).ok();
    }

    #[test]
    fn parse_line_illegal_arg() {
        let prog = "LOADC h".to_string();
        let parser = Parser::new();

        let result = parser.parse_line(&prog);
        assert_eq!(result.is_ok(), false)
    }
}
