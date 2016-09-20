use std::num;

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_op_from_str_for_print() {
        let prog = vec!["PRINT", "test"];
        let expected = OpCode::PRINT("test".to_owned());

        let result = get_op_from_str(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_op_from_str_for_halt() {
        let prog = vec!["HALT"];
        let expected = OpCode::HALT;

        let result = get_op_from_str(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_op_from_str_for_nop() {
        let prog = vec!["NOP"];
        let expected = OpCode::NOP;

        let result = get_op_from_str(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic(expected = "tyr: Invalid operation")]
    fn test_get_op_from_str_illegal_op() {
        let prog = vec!["TEST"];
        get_op_from_str(&prog).ok();
    }

    #[test]
    fn test_get_op_from_str_for_add() {
        let prog = vec!["ADD", "5", "5"];
        let expected = OpCode::ADD((5, 5));

        let result = get_op_from_str(&prog).ok().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_op_from_str_wrong_arg_type() {
        let prog = vec!["ADD", "5", "t"];
        let result = get_op_from_str(&prog);

        assert_eq!(result.is_ok(), false);
    }

    #[test]
    #[should_panic]
    fn test_get_op_from_str_missing_arg() {
        let prog = vec!["ADD", "5"];
        get_op_from_str(&prog).ok();
    }

}
