use op::OpCode;
use std::process;

pub struct Vm<'p> {
    prog: &'p Vec<OpCode>,
    pc: usize
}

impl<'p> Vm<'p> {
    pub fn new(program: &Vec<OpCode>) -> Vm {
        Vm {
            prog: program,
            pc: 0
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

    fn execute(&self, instr: &OpCode) {
        // TODO: Make these actually do something other than print eventually
        match instr.clone() {
            OpCode::HALT => {
                println!("tyr: Halt instruction detected, exiting...");
                process::exit(0);
            },
            OpCode::PRINT(message) => {
                println!("{}", message);
            },
            OpCode::ADD((x, y)) => {
                println!("{}", x + y);
            },
            OpCode::SUB((x, y)) => {
                println!("{}", x - y);
            },
            OpCode::MUL((x, y)) => {
                println!("{}", x * y);
            },
            OpCode::DIV((x, y)) => {
                println!("{}", x / y);
            },
            OpCode::NOP => {},
        }
    }
}

#[cfg(test)]
mod tests {

}
