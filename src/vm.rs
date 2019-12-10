use std::collections::LinkedList;

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
    AdjustRelativeBase,
    Halt,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
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
            2 => Mode::Relative,
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
            9 => Opcode::AdjustRelativeBase,
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
            Opcode::AdjustRelativeBase => 9,
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
pub struct VM {
    bytecode: Vec<i64>,
    pc: usize,
    inputs: LinkedList<i64>,
    outputs: Vec<i64>,
    done: bool,
    relative_base: i64,
}

impl VM {
    pub fn new(bytecode: Vec<i64>) -> VM {
        VM {
            bytecode,
            pc: 0,
            inputs: LinkedList::new(),
            outputs: vec![],
            done: false,
            relative_base: 0,
        }
    }

    pub fn set_inputs(&mut self, v: &[i64]) {
        for &i in v {
            self.inputs.push_back(i);
        }
    }

    fn output(&mut self, o: i64) {
        self.outputs.push(o);
    }

    pub fn outputs(&self) -> Vec<i64> {
        self.outputs.clone()
    }

    pub fn get_last_output(&self) -> i64 {
        *self.outputs.last().unwrap()
    }

    pub fn run(&mut self) {
        while !self.done {
            self.run_till_output();
        }
    }

    // Executes the VM.
    pub fn run_till_output(&mut self) {
        if self.done {
            return;
        }

        loop {
            let inst = self.get_next_instruction();
            match Opcode::from(inst.opcode) {
                Opcode::Halt => {
                    self.pc += 1;
                    self.done = true;
                    break;
                }
                Opcode::AdjustRelativeBase => {
                    let value = self.get_value(&inst.operands[0]);
                    self.relative_base += value;
                    self.pc += 2;
                }
                Opcode::Add => {
                    let v1 = self.get_value(&inst.operands[0]);
                    let v2 = self.get_value(&inst.operands[1]);

                    // Parameters that an instruction writes to
                    // are always positional.
                    let dest = self.get_absolute_address(&inst.operands[2]);
                    self.set_mem(dest, v1 + v2);
                    self.pc += 4;
                }
                Opcode::Multiply => {
                    let v1 = self.get_value(&inst.operands[0]);
                    let v2 = self.get_value(&inst.operands[1]);

                    // Parameters that an instruction writes to
                    // are always positional.
                    let dest = self.get_absolute_address(&inst.operands[2]);
                    self.set_mem(dest, v1 * v2);
                    self.pc += 4;
                }
                Opcode::Input => {
                    let inp = self.inputs.pop_front().unwrap();
                    // In case of input, we only care about the address where to
                    // store the value.
                    let mut address = inst.operands[0].value;
                    if inst.operands[0].mode == Mode::Relative {
                        address += self.relative_base;
                    }
                    self.set_mem(address as usize, inp);
                    self.pc += 2;
                }
                Opcode::Output => {
                    let value = self.get_value(&inst.operands[0]);
                    self.output(value);
                    self.pc += 2;
                    // We break out to let the caller consume output for
                    // the feedback loop.
                    break;
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
                    let address = self.get_absolute_address(&inst.operands[2]);
                    self.set_mem(address, result);
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
                    let address = self.get_absolute_address(&inst.operands[2]);
                    self.set_mem(address, result);
                    self.pc += 4;
                }
            }
        }
    }

    fn get_absolute_address(&self, op: &Operand) -> usize {
        match op.mode {
            Mode::Position => op.value as usize,
            Mode::Relative => (op.value + self.relative_base) as usize,
            Mode::Immediate => panic!("Invalid mode for operand: {:?}", op),
        }
    }

    fn set_mem(&mut self, address: usize, v: i64) {
        self.ensure_mem_availability(address);
        self.bytecode[address] = v;
    }

    fn get_value(&mut self, op: &Operand) -> i64 {
        match op.mode {
            Mode::Immediate => op.value,
            Mode::Position => {
                let address = op.value as usize;
                self.ensure_mem_availability(address);
                self.bytecode[address]
            }
            Mode::Relative => {
                let address = op.value + self.relative_base;
                self.ensure_mem_availability(address as usize);
                self.bytecode[address as usize]
            }
        }
    }

    fn ensure_mem_availability(&mut self, mem_size: usize) {
        if mem_size > self.bytecode.len() {
            // double the memory
            self.bytecode.resize(mem_size * 2, 0);
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
            Opcode::AdjustRelativeBase => {
                operands.push(Operand::new(
                    self.bytecode[self.pc + 1],
                    Mode::from(Mode::parse(mode, 0)),
                ));
            }
            Opcode::Halt => (),
        }

        Instruction { opcode, operands }
    }

    pub fn bytecode(&self) -> Vec<i64> {
        self.bytecode.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mode() {
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
        vm.set_inputs(&[99]);
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
        vm.set_inputs(&[0]);
        vm.run();
        assert_eq!(vm.outputs(), vec![0]);

        let mut vm = VM::new(program.clone());
        vm.set_inputs(&[9]);
        vm.run();
        assert_eq!(vm.outputs(), vec![1]);

        // Same program but uses immediate mode.
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut vm = VM::new(program.clone());
        vm.set_inputs(&[0]);
        vm.run();
        assert_eq!(vm.outputs(), vec![0]);

        let mut vm = VM::new(program.clone());
        vm.set_inputs(&[9]);
        vm.run();
        assert_eq!(vm.outputs(), vec![1]);
    }

    #[test]
    fn test_large_numbers() {
        let large_number = 1125899906842624i64;
        let program = vec![104, large_number, 99];
        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.get_last_output(), large_number);

        // This program should output a 16 digit number.
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.get_last_output().to_string().len(), 16);
    }

    #[test]
    fn test_quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let expected_outputs = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut vm = VM::new(program);
        vm.run();
        assert_eq!(vm.outputs(), expected_outputs);
    }

    #[test]
    fn test_relative_mode_input() {
        let program = vec![
            109, 100, // set relative base to 100
            109, 25, // Increment relative base by 25
            109, -20, // Decrement relative base by 20
            203, 50, // store first input at relative_base + 50 i.e. 105 + 50
            103, 50, // store second input at 50
            99,
        ]; // halt
        let mut vm = VM::new(program);
        vm.set_inputs(&[111, 55]);
        vm.run();
        assert_eq!(vm.bytecode()[155], 111);
        assert_eq!(vm.bytecode()[50], 55);
    }
}
