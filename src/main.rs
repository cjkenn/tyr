extern crate tyr;

use std::env;
use tyr::vm::Vm;
use tyr::parser::Parser;
use tyr::sym_tab::SymbolTable;
use tyr::util;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        panic!("tyr: Expected an input file to execute.");
    });

    // Declare our symbol table and our program
    // vectors vars here. Then, in a scope block, we allow
    // the parser to mutably borrow the symbol table and insert
    // entries in to it. Once that borrow is over, we can
    // pass a reference of that symbol table to the vm
    // and use it.
    let mut sym_tab = SymbolTable::new();
    let prog;
    {
        let parser = Parser::new(&mut sym_tab);
        prog = util::read_file(filename, parser);
    }
    // TODO: Could make this JIT by using the parser in vm,
    // parse a line and then executing it in the execute loop
    let mut vm = Vm::new(&prog, &sym_tab);

    vm.run();
}
