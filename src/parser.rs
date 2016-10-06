use op::{OpCode, OpError};
use sym_tab::SymbolTable;

/// The Parser is responsible for converting strings in a file to
/// OpCodes that can be executed by the vm. The struct also holds
/// some info for error reporting, as well as a symbol table to
/// insert label names.
pub struct Parser<'s> {
    // TODO: Hold some more error handling info here
    // line number, char number, etc.
    /// Current line being parsed.
    line: usize,
    /// Symbol table for execution of this program.
    sym_tab: &'s mut SymbolTable
}

pub type ParseResult = Result<OpCode, OpError>;
pub type ArgResult = Result<i64, OpError>;

impl<'s> Parser<'s> {
    pub fn new(table: &'s mut SymbolTable) -> Parser {
        Parser {
            line: 1,
            sym_tab: table
        }
    }

    /// parse_line takes in a line as a String, and returns a result
    /// containing an OpCode or OpError, as defined in op.rs. In the
    /// match expression, we assume any non-operation string to be a
    /// label. Label handling is done in the parse_label function.
    ///
    /// When we're given an empty string, we assume a NOP. This
    /// function will not panic; that can be decided by the caller
    /// when they handle the Result.
    ///
    /// ## Example
    ///
    /// ```
    /// use tyr::op::OpCode;
    /// use tyr::sym_tab::SymbolTable;
    /// use tyr::parser::Parser;
    ///
    /// let prog = "PRINT Hello!".to_string();
    /// let mut sym_tab = SymbolTable::new();
    /// let mut parser = Parser::new(&mut sym_tab);
    ///
    /// let result = parser.parse_line(&prog).ok().unwrap();
    ///
    /// assert_eq!(OpCode::PRINT("Hello!".to_string()), result);
    /// ```
    pub fn parse_line(&mut self, line: &String) -> ParseResult {
        let op_vec: Vec<&str> = line.split(' ').collect();
        if op_vec.is_empty() {
            return Ok(OpCode::NOP);
        }

        let result = match op_vec[0] {
            "PRINT" => Ok(OpCode::PRINT(op_vec[1].to_string())),
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
            "LOADV" => {
                let arg = try!(self.extract_arg(&op_vec));
                Ok(OpCode::LOADV(arg))
            },
            "STOREV" => {
                let arg = try!(self.extract_arg(&op_vec));
                Ok(OpCode::STOREV(arg))
            },
            "JMP" => Ok(OpCode::JMP(op_vec[1].to_string())),
            "JMPZ" => Ok(OpCode::JMPZ(op_vec[1].to_string())),
            "LOADC" => {
                let arg = try!(self.extract_arg(&op_vec));
                Ok(OpCode::LOADC(arg))
            },
            "JMPI" => {
                let arg = try!(self.extract_arg(&op_vec));
                Ok(OpCode::JMPI(arg))
            },
            "DUP" => Ok(OpCode::DUP),
            _ => self.parse_label(&op_vec)
        };

        self.line = self.line + 1;
        result
    }

    /// Parses a label into the correct OpCode, given a label as a string.
    /// This returns a Result, and will never panic. Before returning,
    /// this function will insert the label into the symbol table for
    /// future use by the vm.
    ///
    /// An Error will be returned if the provided label does not end
    /// with a colon, or if the label has already been declared
    /// (that is, the label already exists in the symbol table).
    fn parse_label(&mut self, op_vec: &Vec<&str>) -> ParseResult {
        let result: Result<OpCode, OpError>;
        let label = op_vec[0];
        let last_char = label.chars().nth(label.len() - 1).unwrap();

        if last_char != ':' {
            result = Err(OpError::Label(
                "tyr: Illegal label name - labels must end with a colon.".to_string()
            ));

            return result;
        }

        if self.sym_tab.is_duplicate(&label.to_string()) {
            let error = format!("tyr [{:?}]: Duplicate label {:?} found!", self.line, label);
            result = Err(OpError::Label(error));

            return result;
        }

        let jmp_label = label.split_at(label.len()-1).0;
        self.sym_tab.insert(jmp_label.to_string(), self.line);
        result = Ok(OpCode::LABEL(jmp_label.to_string(), self.line));

        result
    }

    /// Given a line of a program, split into a vector of strings,
    /// extract the argument provided in the operation into an i64 value.
    /// For example, if we pass in a vector like ["LOADC", "1"],
    /// this function would return Ok(1).
    ///
    /// However, if we pass in something like ["LOADC", "hello"],
    /// we eould return with an OpError::Parse type.
    fn extract_arg(&self, op_vec: &Vec<&str>) -> ArgResult {
        if op_vec.len() < 2 {
            // TODO: Why panic here instead of returning an OpError?
            panic!("tyr: Missing arguments for load operation {:?}.", op_vec[0]);
        }

        let arg = try!(op_vec[1].parse::<i64>());

        Ok(arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use op::{OpCode, OpError};
    use sym_tab::SymbolTable;

    #[test]
    fn parse_line_print() {
        let prog = "PRINT test".to_string();
        let expected = OpCode::PRINT("test".to_string());
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_halt() {
        let prog = "HALT".to_string();
        let expected = OpCode::HALT;
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_nop() {
        let prog = "NOP".to_string();
        let expected = OpCode::NOP;
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_add() {
        let prog = "ADD".to_string();
        let expected = OpCode::ADD;
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_loadc() {
        let prog = "LOADC 5".to_string();
        let expected = OpCode::LOADC(5);
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_label() {
        let prog = "testlabel:".to_string();
        let expected = OpCode::LABEL("testlabel".to_string(), 1);
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_label_illegal_value() {
        let prog = "testlabel".to_string();
        let expected = OpError::Label(
             "tyr: Illegal label name - labels must end with a colon.".to_string()
        );
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).err().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_illegal_op() {
        let prog = "TEST".to_string();
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog);
        // Parse should fail when trying to parse the operation as a label.
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn parse_line_illegal_arg() {
        let prog = "LOADC h".to_string();
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog);
        // Parse should fail when trying to parse "h" as an i64.
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn parse_line_loadv() {
        let prog = "LOADV 5".to_string();
        let expected = OpCode::LOADV(5);
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_storev() {
        let prog = "STOREV 5".to_string();
        let expected = OpCode::STOREV(5);
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_jmpi() {
        let prog = "JMPI 5".to_string();
        let expected = OpCode::JMPI(5);
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_line_dup() {
        let prog = "DUP".to_string();
        let expected = OpCode::DUP;
        let mut sym_tab = SymbolTable::new();
        let mut parser = Parser::new(&mut sym_tab);

        let result = parser.parse_line(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }
}
