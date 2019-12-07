use aoc2019::read_csv_ints;
use aoc2019::vm::VM;

use itertools::Itertools;

fn calculate_thruster_output(program: Vec<i64>, inputs: &[i64]) -> i64 {
    let mut vms = vec![];

    for i in 0..5 {
        vms.push(VM::new(program.clone()));
        // Set the phase value.
        vms[i].set_inputs(&[inputs[i]]);
    }

    for i in 0..5 {
        let mut signal = 0;

        if i > 0 {
            signal = vms[i - 1].get_last_output();
        }

        vms[i].set_inputs(&[signal]);
        vms[i].run();
    }

    vms[4].get_last_output()
}

#[cfg(test)]
mod tests {
    use super::*;

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
