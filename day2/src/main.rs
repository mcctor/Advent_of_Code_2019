mod puzzle1;
mod puzzle2;

fn main() {
    let mut op_code_list = puzzle1::get_operation_code_list();
    puzzle1::restore_gravity_assist(&mut op_code_list);
    puzzle1::run_computation(&mut op_code_list);
    println!("Puzzle1 => Value at position 0 is: {}", op_code_list.get(0).unwrap());

    let mut op_code_list = puzzle1::get_operation_code_list();
    let (noun, verb) = puzzle2::find_pair_for(&mut op_code_list, 19690720);
    println!("Puzzle2 => Result is: {}", ((100 * noun) + verb));
}
