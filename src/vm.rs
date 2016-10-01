use op::OpCode;
use sym_tab::SymbolTable;
use std::process;

/// Maximum size for program stack.
const STACK_SIZE: usize = 50;

/// Holds relevant state info for the execution of a program.
///
/// The vm is stack based, with all operations taking their
/// arguments off of the stack and returning results on top
/// of the stack. The stack supports 64-bit integers, and is given
/// a maximum size based on the constant STACK_SIZE, named above.
pub struct Vm<'p> {
    /// The program to execute, parsed from a file.
    prog: &'p Vec<OpCode>,
    /// Program Counter. Points to the current instruction
    /// in the program (ie. the instruction being executed).
    pc: usize,
    /// The stack itself, where operations are executed.
    stack: [i64; STACK_SIZE],
    /// Stack Pointer. Points to the top of the stack.
    sp: usize,
    /// The Symbol Table contains adresses of labels contained
    /// in the program. These are retrieved in order to execute
    /// jmp instructions.
    sym_tab: &'p SymbolTable
}

impl<'p> Vm<'p> {
    pub fn new(program: &'p Vec<OpCode>, table: &'p SymbolTable) -> Vm<'p> {
        Vm {
            prog: program,
            pc: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            sym_tab: table
        }
    }

    /// Runs the tyr vm. This method loops until the specified
    /// program is completed, or a HALT instruction is found.
    ///
    /// The run process is simple: Fetch the current
    /// instruction from the program, then execute it. The 'program'
    /// described here is a vector of OpCodes. This vector is generated
    /// from the parser, which converts a file of strings into the OpCode type.
    /// This logic is similar to the standard vm operation of fetch, decode,
    /// execute, except the operations are decoded in the parsing phase.
    ///
    /// This function will terminate on any errors encountered during the
    /// execute phase, with a panic. This is sort of like a run time error in
    /// a regular program.
    ///
    /// ## Example
    ///
    /// ```
    /// use tyr::vm::Vm;
    /// use tyr::op::OpCode;
    /// use tyr::sym_tab::SymbolTable;
    ///
    /// let prog = vec![OpCode::PRINT("Hello World".to_string())];
    /// let sym_tab = SymbolTable::new();
    /// let mut vm = Vm::new(&prog, &sym_tab);
    ///
    /// vm.run()
    /// ```
    pub fn run(&mut self) {
        loop {
            if self.pc >= self.prog.len() {
                break;
            }

            let ref curr_instr = self.prog[self.pc];
            self.execute(curr_instr);

            self.pc = self.pc + 1;
        }
    }

    /// Given an instruction opcode, execute it by calling the corresponding
    /// function implemented below. This function should only be called
    /// by the run() function above.
    ///
    /// This function can panic for several reasons:
    ///
    /// 1. A jump is encountered to a label that doesn't exist.
    /// 2. A label has been defined more than once in a program.
    /// 3. The stack overflows/underflows.
    /// 4. An illegal value is placed on the stack, and an operation fails
    ///    because of that value.
    fn execute(&mut self, instr: &OpCode) {
        match instr.clone() {
            OpCode::LOADC(val) => self.loadc(val),
            OpCode::ADD => self.add(),
            OpCode::SUB => self.sub(),
            OpCode::MUL => self.mul(),
            OpCode::DIV => self.div(),
            OpCode::MOD => self.modq(),
            OpCode::AND => self.and(),
            OpCode::OR => self.or(),
            OpCode::NEG => self.neg(),
            OpCode::HALT => process::exit(0),
            OpCode::LOAD => self.load(),
            OpCode::STORE => self.store(),
            OpCode::JMP(label) => self.jmp(label),
            OpCode::JMPZ(label) => self.jmpz(label),
            OpCode::JMPI(offset) => self.jmpi(offset),
            OpCode::PRINT(message) => println!("{}", message),
            OpCode::LOADV(val) => self.loadv(val),
            OpCode::STOREV(val) => self.storev(val),
            OpCode::LABEL(_, _) => {},
            OpCode::NOP => {}
        }
    }

    /// Increase the stack pointer by one. Panics if the stack pointer
    /// goes above the maximum stack size.
    fn increment_sp(&mut self) {
        if self.sp == STACK_SIZE {
            panic!("tyr: Stack overflow");
        }
        self.sp = self.sp + 1;
    }

    /// Decrease the stack pointer by one. Panics if the stack pointer goes
    /// below zero.
    fn decrement_sp(&mut self) {
        if self.sp == 0 {
            panic!("tyr: Stack underflow");
        }
        self.sp = self.sp - 1;
    }

    /// Loads a constant on to the stack.
    ///
    /// By calling:
    ///
    /// LOADC 5
    ///
    /// The stack will look like:
    ///
    /// | 5 | <-- sp
    /// | 0 | <-- bottom of stack
    /// +---+
    fn loadc(&mut self, value: i64) {
        self.increment_sp();
        self.stack[self.sp] = value;
    }

    /// Adds the top two numbers on the stack, and returns the
    /// result on the top of the stack.
    ///
    /// Consider the following sequence of operations:
    ///
    /// LOADC 5
    /// LOADC 5
    /// ADD
    ///
    /// After execution, the stack will look like the following:
    ///
    /// | 10 | <-- sp
    /// |  5 | <-- the first argument is still present at sp-1
    /// |  0 | <-- bottom of stack
    /// +----+
    fn add(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] + self.stack[self.sp-1];
        self.decrement_sp();
    }

    /// Multiplies the top two numbers on the stack, and returns the
    /// result on the top of the stack.
    fn mul(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] * self.stack[self.sp-1];
        self.decrement_sp();
    }

    /// Subtracts the top two numbers on the stack, and returns the
    /// result on the top of the stack.
    fn sub(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] - self.stack[self.sp-1];
        self.decrement_sp();
    }

    /// Divides the top two numbers on the stack, and returns the
    /// result on the top of the stack.
    fn div(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] / self.stack[self.sp-1];
        self.decrement_sp();
    }

    /// Mods the top two numbers on the stack, and returns the
    /// result on the top of the stack.
    fn modq(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] % self.stack[self.sp-1];
        self.decrement_sp();
    }

    /// Performs a bitwise AND on the top two numbers on the stack,
    /// and returns the result on the top of the stack.
    fn and(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] & self.stack[self.sp-1];
        self.decrement_sp();
    }

    /// Performs a bitwise OR on the top two numbers on the stack,
    /// and returns the result on the top of the stack.
    fn or(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] | self.stack[self.sp-1];
        self.decrement_sp();
    }

    /// Negates the top value on the stack, while keeping sp the same.
    ///
    /// Consider the following sequense of operations:
    ///
    /// LOADC 5
    /// NEG
    ///
    /// After exection, the stack will look like the following:
    ///
    /// | -5 | <-- sp
    /// |  0 | <-- bottom of stack
    /// +----+
    fn neg(&mut self) {
        self.stack[self.sp] = -self.stack[self.sp];
    }

    /// Loads an address in stack memory to the top of the stack.
    /// Requires a single argument on top of the stack, corresponding to
    /// the memory address to load. Then, the program contents located at
    /// that address are placed on the top of the stack.
    ///
    /// Consider the following sequence of operations:
    ///
    /// LOADC 5
    /// LOADC 6
    /// LOADC 1 // this is the stack location we want to load.
    ///
    /// The stack now looks like this:
    ///
    /// | 1 | <-- address to load, and sp
    /// | 6 |
    /// | 5 | <-- bottom of stack
    /// +---+
    ///
    /// Now, we can call the load instruction:
    ///
    /// LOAD
    ///
    /// And the stack will look like the following:
    ///
    /// | 5 | <-- loaded value 5 from address 1
    /// | 6 |
    /// | 5 | <-- value at address 1 still remains the same
    /// +---+
    fn load(&mut self) {
        let load_loc = self.maybe_i64_to_usize(self.stack[self.sp])
            .unwrap_or_else(|| panic!("tyr: Attempted to load an illegal value."));

        self.stack[self.sp] = self.stack[load_loc];
    }

    /// Convenience method provided so that we can generate
    /// LOADV i64
    ///
    /// instead of
    ///
    /// LOADC i64
    /// LOAD
    fn loadv(&mut self, val: i64) {
        self.loadc(val);
        self.load();
    }

    /// Stores a value in a specified address on the stack. This function
    /// expects two arguments on the stack: the top value should
    /// be the stack address of where the value will be stored, and the second
    /// value (sp-1) should be the actual number to store. After the number
    /// is stored, we decrement sp, removing the address argument from the stack.
    ///
    /// Consider the following sequence of operations:
    ///
    /// LOADC 5
    /// LOADC 6 // value to store
    /// LOADC 1 // location on stack to put it in
    ///
    /// Now, when we call store, we can put our value in the right spot on the stack:
    ///
    /// STORE
    ///
    /// | 6 | <- sp
    /// | 6 | <- the previous value of 5 has been overwritten by the store call
    /// +---+
    fn store(&mut self) {
        let store_loc = self.maybe_i64_to_usize(self.stack[self.sp])
            .unwrap_or_else(|| panic!("tyr: Attempted to store an illegal value."));

        self.stack[store_loc] = self.stack[self.sp - 1];
        self.decrement_sp();
    }

    /// Convenience method provided so that we can generate
    /// STOREV i64
    ///
    /// instead of
    ///
    /// LOADC i64
    /// STORE
    fn storev(&mut self, val: i64) {
        self.loadc(val);
        self.store();
    }

    /// Jumps to a location on the stack, by moving the program counter to the
    /// correct address. The argument given must be a string that corresponds to
    /// a label provided in the program.
    /// Jmp will panic if the label provided does not exist.
    fn jmp(&mut self, loc: String) {
        let new_pc = self.sym_tab.get(&loc)
            .unwrap_or_else(|| panic!("tyr: Attempted to jump to illegal location"));

        self.pc = *new_pc;
    }

    /// Performs a jmp instruction, if the argument on the top of the stack is
    /// a zero. If the value at sp is not zero, program execution continues and the
    /// top of the stack is popped.
    fn jmpz(&mut self, loc: String) {
        if self.stack[self.sp] == 0 {
            self.jmp(loc);
        }

        self.decrement_sp();
    }

    /// Performs and indexed jump. This function expects a single argument on top
    /// of the stack, an address to jump to. Then, we add the offset provided
    /// to that address and set the program counter.
    fn jmpi(&mut self, offset: i64) {
        let jmp_addr = self.maybe_i64_to_usize(self.stack[self.sp] + offset)
            .unwrap_or_else(|| panic!("tyr: Attempted to calculate an illegal jump offset"));
        self.pc = jmp_addr;

        self.decrement_sp();
    }

    // TODO: Probably shouldn't belong to this struct
    fn maybe_i64_to_usize(&self, num: i64) -> Option<usize> {
        if num < 0 {
            return None;
        }

        Some(num as usize)
    }

    fn peek(&self) -> i64 {
        self.stack[self.sp]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use op::OpCode;
    use sym_tab::SymbolTable;

    #[test]
    fn test_run_loadc() {
        let prog = vec![OpCode::LOADC(5)];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 5);
    }

    #[test]
    fn test_run_add() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::ADD];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 10);
    }

    #[test]
    fn test_run_sub() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(4), OpCode::SUB];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), -1);
    }

    #[test]
    fn test_run_mul() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::MUL];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 25);
    }

    #[test]
    fn test_run_div() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::DIV];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 1);
    }

    #[test]
    fn test_run_div_with_remainder() {
        let prog = vec![OpCode::LOADC(3), OpCode::LOADC(10), OpCode::DIV];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 3);
    }

    #[test]
    fn test_run_modq() {
        let prog = vec![OpCode::LOADC(2), OpCode::LOADC(4), OpCode::MOD];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 0);
    }

    #[test]
    fn test_run_and() {
        let prog = vec![OpCode::LOADC(2), OpCode::LOADC(2), OpCode::AND];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 2);
    }

    #[test]
    fn test_run_or() {
        let prog = vec![OpCode::LOADC(3), OpCode::LOADC(2), OpCode::OR];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 3);
    }

    #[test]
    fn test_run_neg() {
        let prog = vec![OpCode::LOADC(5), OpCode::NEG];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), -5);
    }

    #[test]
    fn test_run_load() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(4), OpCode::LOADC(1), OpCode::LOAD];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 5);
    }

    #[test]
    fn test_run_load_no_contents() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::LOADC(4), OpCode::LOAD];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();
        // Load will return 0 if there are no contents in the stack address attempted to load.
        assert_eq!(vm.peek(), 0);
    }

    #[test]
    #[should_panic(expected = "tyr: Attempted to load an illegal value.")]
    fn test_run_load_illegal_value() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::LOADC(-2), OpCode::LOAD];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();
    }

    #[test]
    fn test_run_loadv() {
        let prog = vec![OpCode::LOADC(3), OpCode::LOADV(1)];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 3);
    }

    #[test]
    fn test_run_store() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(4), OpCode::LOADC(1), OpCode::STORE];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 4);
    }

    #[test]
    #[should_panic(expected = "tyr: Attempted to store an illegal value.")]
    fn test_run_store_illegal_value() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::LOADC(-1), OpCode::STORE];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();
    }

    #[test]
    fn test_run_storev() {
        let prog = vec![OpCode::LOADC(5),  OpCode::STOREV(1)];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 5);
    }

    #[test]
    fn test_run_jmp() {
        let prog = vec![
            OpCode::LABEL("label1".to_string(), 1),
            OpCode::LOADC(5),
            OpCode::LOADC(6),
            OpCode::JMP("halt".to_string()),
            OpCode::LOADC(7),
            OpCode::LABEL("halt".to_string(), 5)
        ];
        let mut sym_tab = SymbolTable::new();
        sym_tab.insert("label1".to_string(), 1);
        sym_tab.insert("halt".to_string(), 5);

        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 6);
    }

    #[test]
    #[should_panic(expected = "tyr: Attempted to jump to illegal location")]
    fn test_run_jmp_no_label() {
        let prog = vec![
            OpCode::LABEL("label1".to_string(), 1),
            OpCode::LOADC(5),
            OpCode::LOADC(6),
            OpCode::JMP("label2".to_string())
        ];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();
    }

    #[test]
    fn test_run_jmpz() {
        let prog = vec![
            OpCode::LABEL("label1".to_string(), 1),
            OpCode::LOADC(5),
            OpCode::LOADC(6),
            OpCode::LOADC(0),
            OpCode::JMPZ("halt".to_string()),
            OpCode::LABEL("halt".to_string(), 5),
            OpCode::LOADC(7)
        ];
        let mut sym_tab = SymbolTable::new();
        sym_tab.insert("label1".to_string(), 1);
        sym_tab.insert("halt".to_string(), 5);

        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 7);
    }

    #[test]
    fn test_run_jmpz_not_zero() {
        let prog = vec![
            OpCode::LABEL("label1".to_string(), 1),
            OpCode::LOADC(5),
            OpCode::LOADC(6),
            OpCode::LOADC(1),
            OpCode::JMPZ("label1".to_string())
        ];
        let sym_tab = SymbolTable::new();
        let mut vm = Vm::new(&prog, &sym_tab);
        vm.run();

        assert_eq!(vm.peek(), 6);
    }
}
