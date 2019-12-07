use aoc2019::read_csv_ints;
use aoc2019::vm::VM;

fn main() {
    let program = read_csv_ints("assets/day5_input");
    println!("{:?}", program);
    let mut vm = VM::new(program);
    vm.set_inputs(&[5]);
    vm.run();
    println!("{:?}", vm.outputs());
}
