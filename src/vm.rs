use op::OpCode;
use std::process;
use std::collections::HashMap;

const STACK_SIZE: usize = 50;

pub struct JmpTable {
    table: HashMap<String, usize>
}

impl JmpTable {
    pub fn new() -> JmpTable {
        JmpTable {
            table: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, val: usize) {
        self.table.insert(key, val);
    }

    pub fn get(&self, key: &String) -> Option<&usize> {
        self.table.get(key)
    }

    pub fn is_duplicate(&self, key: &String) -> bool {
        self.table.get(key).is_some()
    }
}

pub struct Vm<'p> {
    prog: &'p Vec<OpCode>,
    pc: usize,
    stack: [i64; STACK_SIZE],
    sp: usize,
    jmp_table: JmpTable
}

impl<'p> Vm<'p> {
    pub fn new(program: &Vec<OpCode>) -> Vm {
        Vm {
            prog: program,
            pc: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            jmp_table: JmpTable::new()
        }
    }

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

    fn execute(&mut self, instr: &OpCode) {
        match instr.clone() {
            OpCode::LOADC(value) => self.loadc(value),
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
            OpCode::PRINT(message) => println!("{}", message),
            OpCode::LABEL(label, pos) => {
                if self.jmp_table.is_duplicate(&label) {
                    panic!("tyr [{:?}]: Duplicate label {:?} found!", pos, label);
                }
                self.jmp_table.insert(label, pos);
            },
            OpCode::NOP => {}
        }
    }

    fn increment_sp(&mut self) {
        if self.sp == STACK_SIZE {
            panic!("tyr: Stack overflow");
        }
        self.sp = self.sp + 1;
    }

    fn decrement_sp(&mut self) {
        if self.sp == 0 {
            panic!("tyr: Stack underflow");
        }
        self.sp = self.sp - 1;
    }

    fn loadc(&mut self, value: i64) {
        self.increment_sp();
        self.stack[self.sp] = value;
    }

    fn add(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] + self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn mul(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] * self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn sub(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] - self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn div(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] / self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn modq(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] % self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn and(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] & self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn or(&mut self) {
        self.stack[self.sp-1] = self.stack[self.sp] | self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn neg(&mut self) {
        self.stack[self.sp] = -self.stack[self.sp];
    }

    /// Expect an address on top of stack
    fn load(&mut self) {
        let load_loc = self.maybe_i64_to_usize(self.stack[self.sp])
            .unwrap_or_else(|| panic!("tyr: Attempted to load an illegal value."));

        self.stack[self.sp] = self.stack[load_loc];
    }

    /// Expect a value and an address on top of stack
    fn store(&mut self) {
        let store_loc = self.maybe_i64_to_usize(self.stack[self.sp])
            .unwrap_or_else(|| panic!("tyr: Attempted to store an illegal value."));

        self.stack[store_loc] = self.stack[self.sp - 1];
        self.decrement_sp();
    }

    fn jmp(&mut self, loc: String) {
        let new_p = self.jmp_table.get(&loc)
            .unwrap_or_else(|| panic!("tyr: Attempted to jump to illegal location"));

        self.sp = *new_p;
    }

    fn jmpz(&mut self, loc: String) {
        if self.stack[self.sp] == 0 {
            self.jmp(loc);
        } else {
            self.decrement_sp();
        }
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

    #[test]
    fn test_jmp_table_is_duplicate() {
        let mut jmp_table = JmpTable::new();
        let key = "test".to_string();
        jmp_table.insert("test".to_string(), 5);

        let result = jmp_table.is_duplicate(&key);

        assert_eq!(result, true);
    }

    #[test]
    fn test_run_loadc() {
        let prog = vec![OpCode::LOADC(5)];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 5);
    }

    #[test]
    fn test_run_add() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::ADD];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 10);
    }

    #[test]
    fn test_run_sub() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(4), OpCode::SUB];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), -1);
    }

    #[test]
    fn test_run_mul() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::MUL];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 25);
    }

    #[test]
    fn test_run_div() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::DIV];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 1);
    }

    #[test]
    fn test_run_div_with_remainder() {
        let prog = vec![OpCode::LOADC(3), OpCode::LOADC(10), OpCode::DIV];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 3);
    }

    #[test]
    fn test_run_modq() {
        let prog = vec![OpCode::LOADC(2), OpCode::LOADC(4), OpCode::MOD];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 0);
    }

    #[test]
    fn test_run_and() {
        let prog = vec![OpCode::LOADC(2), OpCode::LOADC(2), OpCode::AND];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 2);
    }

    #[test]
    fn test_run_or() {
        let prog = vec![OpCode::LOADC(3), OpCode::LOADC(2), OpCode::OR];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 3);
    }

    #[test]
    fn test_run_neg() {
        let prog = vec![OpCode::LOADC(5), OpCode::NEG];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), -5);
    }

    #[test]
    fn test_run_load() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(4), OpCode::LOADC(1), OpCode::LOAD];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 5);
    }

    #[test]
    fn test_run_load_no_contents() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::LOADC(4), OpCode::LOAD];
        let mut vm = Vm::new(&prog);
        vm.run();
        // Load will return 0 if there are no contents in the stack address attempted to load.
        assert_eq!(vm.peek(), 0);
    }

    #[test]
    #[should_panic(expected = "tyr: Attempted to load an illegal value.")]
    fn test_run_load_illegal_value() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::LOADC(-2), OpCode::LOAD];
        let mut vm = Vm::new(&prog);
        vm.run();
    }

    #[test]
    fn test_run_store() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(4), OpCode::LOADC(1), OpCode::STORE];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 4);
    }

    #[test]
    #[should_panic(expected = "tyr: Attempted to store an illegal value.")]
    fn test_run_store_illegal_value() {
        let prog = vec![OpCode::LOADC(5), OpCode::LOADC(5), OpCode::LOADC(-1), OpCode::STORE];
        let mut vm = Vm::new(&prog);
        vm.run();
    }

    #[test]
    fn test_run_jmp() {
        let prog = vec![
            OpCode::LABEL("label1".to_string(), 1),
            OpCode::LOADC(5),
            OpCode::LOADC(6),
            OpCode::LOADC(7),
            OpCode::JMP("label1".to_string())
        ];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 5);
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
        let mut vm = Vm::new(&prog);
        vm.run();
    }

    #[test]
    fn test_run_jmpz() {
        let prog = vec![
            OpCode::LABEL("label1".to_string(), 1),
            OpCode::LOADC(5),
            OpCode::LOADC(6),
            OpCode::LOADC(0),
            OpCode::JMPZ("label1".to_string())
        ];
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 5);
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
        let mut vm = Vm::new(&prog);
        vm.run();

        assert_eq!(vm.peek(), 6);
    }
}
