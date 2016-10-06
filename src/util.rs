use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use op::OpCode;
use parser::Parser;

/// Given a file name and a Parser struct, open that file,
/// read it line by line (as strings), and convert each
/// line into a valid opcode.  Each valid opcode is added
/// to a vector, which is returned. This vector can then be
/// read by the vm.
///
/// This function panics on parse errors, or when the filename
/// provided cannot be opened.
pub fn read_file(filename: String, mut parser: Parser) -> Vec<OpCode> {
    let path = Path::new(&filename);
    let display = path.display();

    let input_file = match File::open(&path) {
        Err(error) => panic!("tyr: Failed to open {}: {}", display, Error::description(&error)),
        Ok(file) => file
    };

    let reader = BufReader::new(input_file);
    let mut instructions: Vec<OpCode> = Vec::new();

    for line in reader.lines() {
        match line {
            Err(_) => panic!("tyr: cannot parse line {:?}", line),
            Ok(result) => {
                // TODO: Make this a try?
                let op = match parser.parse_line(&result) {
                    Ok(op) => op,
                    Err(error) => panic!("tyr: {:?}", error)
                };

                instructions.push(op);
            }
        }
    };

    instructions
}

/// Convert an i64 to usize. If the i64 cannot be converted
/// to usize, then return None. This method is provided
/// as convenience so that stack values (as i64) can be
/// used as indices into the stack for some operations.
pub fn maybe_i64_to_usize(num: i64) -> Option<usize> {
    if num < 0 {
        return None;
    }

    Some(num as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_i64_to_usize_valid_input() {
        let result = maybe_i64_to_usize(-10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_maybe_i64_to_usize_invalid_input() {
        let val: usize = 5;
        let result = maybe_i64_to_usize(5);
        assert_eq!(result, Some(val));
    }
}
