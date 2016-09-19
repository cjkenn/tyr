#[derive(Clone, Debug)]
pub enum OpCode {
    PRINT(String), // TODO: this can probably be a &str
    ADD((i64, i64)),
    SUB((i64, i64)),
    MUL((i64, i64)),
    DIV((i64, i64)),
    HALT,
    NOP
}

// TODO: Custom error type here
pub fn get_op_from_str(op_vec: &Vec<&str>) -> Result<OpCode, String> {
    match op_vec[0] {
        "PRINT" => Ok(OpCode::PRINT(op_vec[1].to_owned())),
        "HALT" => Ok(OpCode::HALT),
        "ADD" => {
            let args = try!(extract_args(op_vec));
            Ok(OpCode::ADD(args))
        },
        "SUB" => {
            let args = try!(extract_args(op_vec));
            Ok(OpCode::SUB(args))
        },
        "MUL" => {
            let args = try!(extract_args(op_vec));
            Ok(OpCode::MUL(args))
        },
        "DIV" => {
            let args = try!(extract_args(op_vec));
            Ok(OpCode::DIV(args))
        },
        _ => panic!("tyr: Invalid operation \'{:?}\' specified", op_vec[0])
    }
}

fn extract_args(op_vec: &Vec<&str>) -> Result<(i64, i64), String> {
    if op_vec.len() < 3 {
        panic!("tyr: Missing arguments for arithmetic operation {:?}.", op_vec[0]);
    }

    let first_arg = try!(op_vec[1].parse::<i64>().map_err(|e| e.to_string()));
    let second_arg = try!(op_vec[2].parse::<i64>().map_err(|e| e.to_string()));

    Ok((first_arg, second_arg))
}

fn validate_arg_len()  {
    // TODO
}
