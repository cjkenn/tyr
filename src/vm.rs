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
                // Insert pos+1 because we actually want to jump the line
                // after the label, not the label itself
                self.jmp_table.insert(label, pos+1);
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
        self.stack[self.sp] = self.stack[self.sp] + self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn mul(&mut self) {
        self.stack[self.sp] = self.stack[self.sp] * self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn sub(&mut self) {
        self.stack[self.sp] = self.stack[self.sp] - self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn div(&mut self) {
        // TODO: This might not work?
        self.stack[self.sp] = self.stack[self.sp] / self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn modq(&mut self) {
        self.stack[self.sp] = self.stack[self.sp] % self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn and(&mut self) {
        self.stack[self.sp] = self.stack[self.sp] & self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn or(&mut self) {
        self.stack[self.sp] = self.stack[self.sp] | self.stack[self.sp-1];
        self.decrement_sp();
    }

    fn neg(&mut self) {
        self.stack[self.sp] = -self.stack[self.sp];
    }

    fn load(&mut self) {
        let load_val = self.maybe_i64_to_usize(self.stack[self.sp])
            .unwrap_or_else(|| panic!("tyr: Attempted to load an illegal value."));

        self.stack[self.sp] = self.stack[load_val];
    }

    fn store(&mut self) {
        let store_val = self.maybe_i64_to_usize(self.stack[self.sp])
            .unwrap_or_else(|| panic!("tyr: Attempted to store and illegal value."));

        self.stack[store_val] = self.stack[self.sp - 1];
        self.decrement_sp();
    }

    fn jmp(&mut self, loc: String) {
        let new_p = self.jmp_table.get(&loc)
            .unwrap_or_else(|| panic!("tyr: Attempted to jump to illegal location"));

        self.sp = *new_p;
    }

    fn jmpz(&mut self, loc: String) {

    }

    // TODO: Probably shouldn't belong to this struct
    fn maybe_i64_to_usize(&self, num: i64) -> Option<usize> {
        if num < 0 {
            return None;
        }

        Some(num as usize)
    }
}

#[cfg(test)]
mod tests {

}
