extern crate tyr;

use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use tyr::op::OpCode;
use tyr::vm::Vm;
use tyr::parser::Parser;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        panic!("tyr: Expected an input file to execute.");
    });

    let parser = Parser::new();
    let prog = read_file(filename, parser);
    // TODO: Could make this JIT by using the parser in vm,
    // parse a line and then executing it in the execute loop
    let mut vm = Vm::new(&prog);

    vm.run();
}

fn read_file(filename: String, parser: Parser) -> Vec<OpCode> {
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
