use aoc2019::read_csv_ints;
use aoc2019::vm::VM;

fn main() {
    let program = read_csv_ints("assets/day9_input");
    let mut vm = VM::new(program);
    vm.set_inputs(&[1]);
    vm.run();
    println!("Outputs: {:?}", vm.outputs());
}
