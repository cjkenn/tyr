extern crate tyr;

use std::process;
use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use tyr::op::{OpCode, get_op_from_str};

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        panic!("tyr: Expected an input file to execute.");
    });

    let prog = read_file(filename);
    let mut pc = 0;

    loop {
        let ref curr_instr = prog[pc];
        execute(curr_instr);

        pc = pc + 1;
    }
}

// TODO: Make return an Ok?
fn execute(instr: &OpCode) {
    match instr.clone() {
        OpCode::HALT => {
            println!("Halt instruction detected, exiting...");
            process::exit(0);
        },
        OpCode::PRINT(message) => {
            println!("{}", message);
        },
        OpCode::ADD(x, y) => {
            println!("{}", x + y);
        },
        OpCode::SUB(x, y) => {
            println!("{}", x - y);
        },
        OpCode::MUL(x, y) => {
            println!("{}", x * y);
        },
        OpCode::DIV(x, y) => {
            println!("{}", x / y);
        },
        OpCode::NOP => {},
    }
}

fn read_file(filename: String) -> Vec<OpCode> {
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
                let op = parse_line_into_op(result);
                instructions.push(op);
            }
        }
    };

    instructions
}

fn parse_line_into_op(line: String) -> OpCode {
    let words: Vec<&str> = line.split(' ').collect();
    if words.is_empty() {
        return OpCode::NOP;
    }

    // TODO: Check for missing args here, on ops that need them

    match get_op_from_str(words) {
        Ok(result) => result,
        Err(error) => panic!("tyr: {:?}", error)
    }
}
