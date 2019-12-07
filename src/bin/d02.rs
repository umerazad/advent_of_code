use aoc2019::read_csv_ints;
use aoc2019::vm::VM;

fn main() {
    let mut program = read_csv_ints("assets/day2_input");
    let desired_result = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            program[1] = noun;
            program[2] = verb;
            let mut vm = VM::new(program.clone());
            vm.run();
            if vm.bytecode()[0] == desired_result {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}
