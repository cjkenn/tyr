use std::num;

#[derive(Clone, Debug)]
pub enum OpCode {
    PRINT(String),
    ADD((i64, i64)),
    SUB((i64, i64)),
    MUL((i64, i64)),
    DIV((i64, i64)),
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

pub fn get_op_from_str(op_vec: &Vec<&str>) -> Result<OpCode, OpError> {
    match op_vec[0] {
        "PRINT" => Ok(OpCode::PRINT(op_vec[1].to_owned())),
        "HALT" => Ok(OpCode::HALT),
        "NOP" => Ok(OpCode::NOP),
        "ADD" => {
            let args = try!(extract_bin_args(op_vec));
            Ok(OpCode::ADD(args))
        },
        "SUB" => {
            let args = try!(extract_bin_args(op_vec));
            Ok(OpCode::SUB(args))
        },
        "MUL" => {
            let args = try!(extract_bin_args(op_vec));
            Ok(OpCode::MUL(args))
        },
        "DIV" => {
            let args = try!(extract_bin_args(op_vec));
            Ok(OpCode::DIV(args))
        },
        _ => panic!("tyr: Invalid operation \'{:?}\' specified", op_vec[0])
    }
}

fn extract_bin_args(op_vec: &Vec<&str>) -> Result<(i64, i64), OpError> {
    if op_vec.len() < 3 {
        panic!("tyr: Missing arguments for arithmetic operation {:?}.", op_vec[0]);
    }

    let first_arg = try!(op_vec[1].parse::<i64>());
    let second_arg = try!(op_vec[2].parse::<i64>());

    Ok((first_arg, second_arg))
}
