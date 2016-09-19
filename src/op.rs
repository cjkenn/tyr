#[derive(Clone, Debug)]
pub enum OpCode {
    PRINT(String), // TODO: this can probably be a &str
    ADD(i64, i64),
    SUB(i64, i64),
    MUL(i64, i64),
    DIV(i64, i64),
    HALT,
    NOP
}

// TODO: Custom error type here
pub fn get_op_from_str(op_vec: Vec<&str>) -> Result<OpCode, String> {
    // TODO: Error checking in all these cases for missing args
    // TODO: implement rest of ops
    match op_vec[0] {
        "PRINT" => Ok(OpCode::PRINT(op_vec[1].to_owned())),
        "ADD" => {
            if op_vec.len() < 3 {
                panic!("tyr: Missing arguments for ADD operation.");
            }
            let first_arg = try!(op_vec[1].parse::<i64>().map_err(|e| e.to_string()));
            let second_arg = try!(op_vec[2].parse::<i64>().map_err(|e| e.to_string()));

            Ok(OpCode::ADD(first_arg, second_arg))
        }


        _ => panic!("tyr: Invalid operation \'{:?}\' specified", op_vec[0])
    }
}
