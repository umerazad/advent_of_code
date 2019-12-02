use aoc2019::read_csv_ints;

enum Opcode {
    Add,
    Multiply,
    Halt,
}

impl From<usize> for Opcode {
    fn from(v: usize) -> Self {
        match v {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Halt,
            x => panic!("Unexpected opcode: {}", x),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: usize,
    pub op1: usize,
    pub op2: usize,
    pub op3: usize,
}

#[derive(Debug)]
struct VM {
    pub bytecode: Vec<usize>,
    pub pc: usize,
}

impl VM {
    fn new(bytecode: Vec<usize>) -> VM {
        VM { bytecode, pc: 0 }
    }

    // Executes the VM.
    fn run(&mut self) {
        loop {
            let inst = self.get_next_instruction();
            match Opcode::from(inst.opcode) {
                Opcode::Halt => break,
                Opcode::Add => {
                    let v1 = self.bytecode[inst.op1];
                    let v2 = self.bytecode[inst.op2];
                    self.bytecode[inst.op3] = v1 + v2;
                }
                Opcode::Multiply => {
                    let v1 = self.bytecode[inst.op1];
                    let v2 = self.bytecode[inst.op2];
                    self.bytecode[inst.op3] = v1 * v2;
                }
            }
        }
    }

    fn get_next_instruction(&mut self) -> Instruction {
        let mut op1 = 0;
        let mut op2 = 0;
        let mut op3 = 0;
        let opcode = self.bytecode[self.pc];

        if opcode != 99 {
            op1 = self.bytecode[self.pc + 1];
            op2 = self.bytecode[self.pc + 2];
            op3 = self.bytecode[self.pc + 3];
            self.pc += 4;
        }

        Instruction {
            opcode,
            op1,
            op2,
            op3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let program = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];

        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.bytecode, expected);
        assert_eq!(4, vm.pc);
    }

    fn test_multiply() {
        let program = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];

        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.bytecode, expected);
        assert_eq!(4, vm.pc);
    }

    fn test_simple_program() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];

        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.bytecode, expected);
        assert_eq!(4, vm.pc);
    }

    fn test_simple_program2() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.bytecode, expected);
        assert_eq!(8, vm.pc);
    }
}

fn main() {
    let mut program = read_csv_ints("assets/day2_input");
    let desired_result = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            program[1] = noun;
            program[2] = verb;
            let mut vm = VM::new(program.clone());
            vm.run();
            if vm.bytecode[0] == desired_result {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}
