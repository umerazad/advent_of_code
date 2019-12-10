use aoc2019::read_csv_ints;
use aoc2019::vm::VM;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day9_part1() {
        let program = read_csv_ints("assets/day9_input");
        let mut vm = VM::new(program);
        vm.set_inputs(&[1]);
        vm.run();
        assert_eq!(vm.get_last_output(), 3598076521);
    }

    fn test_day9_part2() {
        let program = read_csv_ints("assets/day9_input");
        let mut vm = VM::new(program);
        vm.set_inputs(&[1]);
        vm.run();
        assert_eq!(vm.get_last_output(), 90722);
    }
}

fn main() {
    let program = read_csv_ints("assets/day9_input");
    let mut vm = VM::new(program);
    vm.set_inputs(&[2]);
    vm.run();
    println!("Outputs: {:?}", vm.outputs());
}
