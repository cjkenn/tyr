use op::OpCode;
use std::process;

const STACK_SIZE: usize = 50;

pub struct Vm<'p> {
    prog: &'p Vec<OpCode>,
    pc: usize,
    stack: [i64; STACK_SIZE],
    sp: usize
}

impl<'p> Vm<'p> {
    pub fn new(program: &Vec<OpCode>) -> Vm {
        Vm {
            prog: program,
            pc: 0,
            stack: [0; STACK_SIZE],
            sp: 0
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
            OpCode::HALT => process::exit(0),
            OpCode::PRINT(message) => println!("{}", message),
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
}

#[cfg(test)]
mod tests {

}
