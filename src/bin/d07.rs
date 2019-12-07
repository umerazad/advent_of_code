use aoc2019::read_csv_ints;
use itertools::Itertools;
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
    pub inputs: LinkedList<i64>,
    pub outputs: Vec<i64>,
}

impl VM {
    fn new(bytecode: Vec<i64>) -> VM {
        VM {
            bytecode,
            pc: 0,
            inputs: LinkedList::new(),
            outputs: vec![],
        }
    }

    fn set_inputs(&mut self, v: &[i64]) {
        for &i in v {
            self.inputs.push_back(i);
        }
    }

    fn output(&mut self, o: i64) {
        self.outputs.push(o);
    }

    fn outputs(&self) -> Vec<i64> {
        self.outputs.clone()
    }

    fn get_last_output(&self) -> i64 {
        *self.outputs.last().unwrap()
    }

    // Executes the VM.
    fn run(&mut self) {
        loop {
            let inst = self.get_next_instruction();
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
                    let inp = self.inputs.pop_front().unwrap();
                    println!("Supplying input: {}", inp);
                    self.bytecode[inst.operands[0].value as usize] = inp;
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

fn calculate_thruster_output(program: Vec<i64>, inputs: &[i64]) -> i64 {
    let mut a1 = VM::new(program.clone());
    a1.set_inputs(&[inputs[0], 0]);
    a1.run();

    let mut a2 = VM::new(program.clone());
    a2.set_inputs(&[inputs[1], a1.get_last_output()]);
    a2.run();

    let mut a3 = VM::new(program.clone());
    a3.set_inputs(&[inputs[2], a2.get_last_output()]);
    a3.run();

    let mut a4 = VM::new(program.clone());
    a4.set_inputs(&[inputs[3], a3.get_last_output()]);
    a4.run();

    let mut a5 = VM::new(program.clone());
    a5.set_inputs(&[inputs[4], a4.get_last_output()]);
    a5.run();

    a5.get_last_output()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_amplification_circuit() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let inputs = [4, 3, 2, 1, 0];

        assert_eq!(43210, calculate_thruster_output(program, &inputs));
    }

    #[test]
    fn test_amplification_circuit2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let inputs = [0, 1, 2, 3, 4];

        assert_eq!(54321, calculate_thruster_output(program, &inputs));
    }

    #[test]
    fn test_amplification_circuit3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        let inputs = [1, 0, 4, 3, 2];

        assert_eq!(65210, calculate_thruster_output(program, &inputs));
    }
}

fn main() {
    let program = read_csv_ints("assets/day7_input");
    let perms = (0..5).permutations(5);
    let mut max_thrust = 0;
    for x in perms {
        let thrust = calculate_thruster_output(program.clone(), &x);
        if thrust > max_thrust {
            max_thrust = thrust;
        }
    }

    println!("Max thrust: {}", max_thrust);
}
