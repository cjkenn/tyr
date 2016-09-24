use op::{OpCode, OpError};

pub struct Parser {
    // TODO: Hold some error handling info here
    // line number, char number, etc.
    line: usize
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            line: 1
        }
    }

    pub fn parse_line(&mut self, line: &String) -> Result<OpCode, OpError> {
        let op_vec: Vec<&str> = line.split(' ').collect();
        if op_vec.is_empty() {
            return Ok(OpCode::NOP);
        }

        let result = match op_vec[0] {
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
            "LOAD" => Ok(OpCode::LOAD),
            "STORE" => Ok(OpCode::STORE),
            "JMP" => Ok(OpCode::JMP(op_vec[1].to_owned())),
            "JMPZ" => Ok(OpCode::JMPZ(op_vec[1].to_owned())),
            "LOADC" => {
                let arg = try!(self.extract_arg(&op_vec));
                Ok(OpCode::LOADC(arg))
            },
            _ => self.parse_label(&op_vec)
        };

        self.line = self.line + 1;
        result
    }

    fn parse_label(&self, op_vec: &Vec<&str>) -> Result<OpCode, OpError> {
        let result: Result<OpCode, OpError>;
        let label = op_vec[0];
        let last_char = label.chars().nth(label.len() - 1).unwrap();

        if last_char != ':' {
            result = Err(OpError::Label(
                "tyr: Illegal label name - labels must end with a colon.".to_string()
            ))
        } else {
            result = Ok(OpCode::LABEL(label.to_string(), self.line));
        }

        result
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
        let mut parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_halt() {
        let prog = "HALT".to_string();
        let expected = OpCode::HALT;
        let mut parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_nop() {
        let prog = "NOP".to_string();
        let expected = OpCode::NOP;
        let mut parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_add() {
        let prog = "ADD".to_string();
        let expected = OpCode::ADD;
        let mut parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_loadc() {
        let prog = "LOADC 5".to_string();
        let expected = OpCode::LOADC(5);
        let mut parser = Parser::new();

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_illegal_op() {
        let prog = "TEST".to_string();
        let mut parser = Parser::new();

        let result = parser.parse_line(&prog);
        // Parse should fail when trying to parse the operation as a label.
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn parse_line_illegal_arg() {
        let prog = "LOADC h".to_string();
        let mut parser = Parser::new();

        let result = parser.parse_line(&prog);
        // Parse should fail when trying to parse "h" as an i64.
        assert_eq!(result.is_ok(), false);
    }
}
