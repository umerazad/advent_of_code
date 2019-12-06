use aoc2019::read_csv_ints;

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn parse(m: i64, index: i64) -> Mode {
        match index {
            0 => Mode::from((m % 1000) / 100),
            1 => Mode::from((m % 10_000) / 1_000),
            2 => Mode::from((m % 100_000) / 10_000),
            x => panic!("Unexpected index for mode: {}", x),
        }
    }
}

impl From<i64> for Mode {
    fn from(v: i64) -> Self {
        match v {
            0 => Mode::Position,
            1 => Mode::Immediate,
            x => panic!("Unexpected mode: {}", x),
        }
    }
}

#[derive(Debug)]
pub struct Operand {
    pub value: i64,
    pub mode: Mode,
}

impl Operand {
    fn new(value: i64, mode: Mode) -> Self {
        Operand { mode, value }
    }
}

impl From<i64> for Opcode {
    fn from(v: i64) -> Self {
        match v % 100 {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            x => panic!("Unexpected opcode: {}", x),
        }
    }
}

impl From<Opcode> for i64 {
    fn from(v: Opcode) -> Self {
        match v {
            Opcode::Add => 1,
            Opcode::Multiply => 2,
            Opcode::Input => 3,
            Opcode::Output => 4,
            Opcode::JumpIfTrue => 5,
            Opcode::JumpIfFalse => 6,
            Opcode::LessThan => 7,
            Opcode::Equals => 8,
            Opcode::Halt => 99,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: i64,
    pub operands: Vec<Operand>,
}

#[derive(Debug)]
struct VM {
    pub bytecode: Vec<i64>,
    pub pc: usize,
    pub input: i64,
    pub outputs: Vec<i64>,
}

impl VM {
    fn new(bytecode: Vec<i64>) -> VM {
        VM {
            bytecode,
            pc: 0,
            input: 1,
            outputs: vec![],
        }
    }

    fn set_input(&mut self, v: i64) {
        self.input = v;
    }

    fn output(&mut self, o: i64) {
        println!("Output: {}", o);
        self.outputs.push(o);
    }

    fn outputs(&self) -> Vec<i64> {
        self.outputs.clone()
    }

    // Executes the VM.
    fn run(&mut self) {
        loop {
            println!("PC: {}", self.pc);
            let inst = self.get_next_instruction();
            println!("Executing: {:?}", inst);
            match Opcode::from(inst.opcode) {
                Opcode::Halt => {
                    // Not technically needed. Just leaving it here to let old
                    // tests pass.
                    self.pc += 1;
                    break;
                }
                Opcode::Add => {
                    let v1 = self.get_value(&inst.operands[0]);
                    let v2 = self.get_value(&inst.operands[1]);

                    // Parameters that an instruction writes to
                    // are always positional.
                    self.bytecode[inst.operands[2].value as usize] = v1 + v2;
                    self.pc += 4;
                }
                Opcode::Multiply => {
                    let v1 = self.get_value(&inst.operands[0]);
                    let v2 = self.get_value(&inst.operands[1]);

                    // Parameters that an instruction writes to
                    // are always positional.
                    self.bytecode[inst.operands[2].value as usize] = v1 * v2;
                    self.pc += 4;
                }
                Opcode::Input => {
                    self.bytecode[inst.operands[0].value as usize] = self.input;
                    self.pc += 2;
                }
                Opcode::Output => {
                    self.output(self.get_value(&inst.operands[0]));
                    self.pc += 2;
                }
                Opcode::JumpIfTrue => {
                    if self.get_value(&inst.operands[0]) != 0 {
                        self.pc = self.get_value(&inst.operands[1]) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Opcode::JumpIfFalse => {
                    if self.get_value(&inst.operands[0]) == 0 {
                        self.pc = self.get_value(&inst.operands[1]) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Opcode::LessThan => {
                    let v1 = self.get_value(&inst.operands[0]);
                    let v2 = self.get_value(&inst.operands[1]);

                    let mut result = 0;
                    if v1 < v2 {
                        result = 1
                    }
                    // Parameters that an instruction writes to
                    // are always positional.
                    self.bytecode[inst.operands[2].value as usize] = result;
                    self.pc += 4;
                }
                Opcode::Equals => {
                    let v1 = self.get_value(&inst.operands[0]);
                    let v2 = self.get_value(&inst.operands[1]);

                    let mut result = 1;
                    if v1 != v2 {
                        result = 0;
                    }
                    // Parameters that an instruction writes to
                    // are always positional.
                    self.bytecode[inst.operands[2].value as usize] = result;
                    self.pc += 4;
                }
            }
            println!("Outputs: {:?}", self.outputs);
        }
    }

    fn get_value(&self, op: &Operand) -> i64 {
        if op.mode == Mode::Immediate {
            op.value
        } else {
            self.bytecode[op.value as usize]
        }
    }

    fn get_next_instruction(&mut self) -> Instruction {
        let mut operands: Vec<Operand> = Vec::new();

        let code = self.bytecode[self.pc];
        let mode = code - (code % 100);
        let opcode = code % 100;

        match Opcode::from(code) {
            Opcode::Add => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 2],
                    Mode::from(Mode::parse(mode, 1)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 3],
                    Mode::from(Mode::parse(mode, 2)),
                ));
            }
            Opcode::Multiply => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 2],
                    Mode::from(Mode::parse(mode, 1)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 3],
                    Mode::from(Mode::parse(mode, 2)),
                ));
            }
            Opcode::Input => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
            }
            Opcode::Output => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
            }
            Opcode::JumpIfTrue => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 2],
                    Mode::from(Mode::parse(mode, 1)),
                ));
            }
            Opcode::JumpIfFalse => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 2],
                    Mode::from(Mode::parse(mode, 1)),
                ));
            }
            Opcode::LessThan => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 2],
                    Mode::from(Mode::parse(mode, 1)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 3],
                    Mode::from(Mode::parse(mode, 2)),
                ));
            }
            Opcode::Equals => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 2],
                    Mode::from(Mode::parse(mode, 1)),
                ));
                operands.push(Operand::new(
                    self.bytecode[self.pc + 3],
                    Mode::from(Mode::parse(mode, 2)),
                ));
            }
            Opcode::Halt => (),
        }

        Instruction { opcode, operands }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mode() {
        /*
        match index {
            0 => Mode::from((m % 1000) / 100),
            1 => Mode::from((m % 10_000) / 1_000),
            2 => Mode::from((m % 100_000) / 10_000),
            x => panic!("Unexpected index for mode: {}", x),
        }
        */

        let m = 1002;
        assert_eq!(Mode::Position, Mode::parse(m, 0));
        assert_eq!(Mode::Immediate, Mode::parse(m, 1));
        assert_eq!(Mode::Position, Mode::parse(m, 2));

        let m = 11100;
        assert_eq!(Mode::Immediate, Mode::parse(m, 0));
        assert_eq!(Mode::Immediate, Mode::parse(m, 1));
        assert_eq!(Mode::Immediate, Mode::parse(m, 2));
    }

    #[test]
    fn test_add() {
        let program = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];

        let mut vm = VM::new(program);
        vm.run();
        println!("{:?}", vm);
        assert_eq!(vm.bytecode, expected);
        assert_eq!(5, vm.pc);
    }

    fn test_multiply() {
        let program = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];

        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.bytecode, expected);
        assert_eq!(5, vm.pc);
    }

    fn test_simple_program() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];

        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.bytecode, expected);
        assert_eq!(5, vm.pc);
    }

    fn test_simple_program2() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.bytecode, expected);
        assert_eq!(9, vm.pc);
    }

    #[test]
    fn test_input_output() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 3, 0, 4, 0, 99];
        let expected = vec![99, 1, 1, 4, 2, 5, 6, 0, 3, 0, 4, 0, 99];
        let mut vm = VM::new(program);
        vm.set_input(99);
        vm.run();
        assert_eq!(vm.bytecode, expected);
        assert_eq!(vm.outputs, vec![99]);
    }

    #[test]
    fn test_jump_instructions() {
        // program that outputs 0 if the input was 0 otherwise 1.
        // It uses position mode for all arguments.
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut vm = VM::new(program.clone());
        vm.set_input(0);
        vm.run();
        assert_eq!(vm.outputs(), vec![0]);

        let mut vm = VM::new(program.clone());
        vm.set_input(9);
        vm.run();
        assert_eq!(vm.outputs(), vec![1]);

        // Same program but uses immediate mode.
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut vm = VM::new(program.clone());
        vm.set_input(0);
        vm.run();
        assert_eq!(vm.outputs(), vec![0]);

        let mut vm = VM::new(program.clone());
        vm.set_input(9);
        vm.run();
        assert_eq!(vm.outputs(), vec![1]);
    }
}

fn main() {
    let program = read_csv_ints("assets/day5_input");
    println!("{:?}", program);
    let mut vm = VM::new(program);
    vm.set_input(5);
    vm.run();
    println!("{:?}", vm.outputs);
}