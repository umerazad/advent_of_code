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
    pub done: bool,
}

impl VM {
    fn new(bytecode: Vec<i64>) -> VM {
        VM {
            bytecode,
            pc: 0,
            inputs: LinkedList::new(),
            outputs: vec![],
            done: false,
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
                    self.bytecode[inst.operands[0].value as usize] = inp;
                    self.pc += 2;
                }
                Opcode::Output => {
                    self.output(self.get_value(&inst.operands[0]));
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

fn calculate_feedback_loop_thruster_output(program: Vec<i64>, inputs: &[i64]) -> i64 {
    let mut vms = vec![];

    for i in 0..5 {
        vms.push(VM::new(program.clone()));
        // Set the phase setting for each VM.
        vms[i].set_inputs(&[inputs[i]]);
    }

    // Initial signal is 0.
    let mut signal = 0;

    let mut index = 0;
    // Loop until the last VM halts.
    while !vms[4].done {
        vms[index].set_inputs(&[signal]);
        vms[index].run();
        signal = vms[index].get_last_output();
        index = (index + 1) % 5;
    }

    signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_loop1() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let inputs = [9, 8, 7, 6, 5];

        assert_eq!(
            139629729,
            calculate_feedback_loop_thruster_output(program, &inputs)
        );
    }

    #[test]
    fn test_feedback_loop2() {
        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let inputs = [9, 7, 8, 5, 6];

        assert_eq!(
            18216,
            calculate_feedback_loop_thruster_output(program, &inputs)
        );
    }
}

fn main() {
    let program = read_csv_ints("assets/day7_input");
    let perms = (5..10).permutations(5);
    let mut max_thrust = 0;
    for x in perms {
        let thrust = calculate_feedback_loop_thruster_output(program.clone(), &x);
        if thrust > max_thrust {
            max_thrust = thrust;
        }
    }

    println!("Max thrust: {}", max_thrust);
}
