mod puzzle1;

fn main() {
    let mut op_code_list = puzzle1::get_operation_code_list();
    puzzle1::restore_gravity_assist(&mut op_code_list);
    puzzle1::run_computation(&mut op_code_list);

    println!("Value at position 0 is: {}", op_code_list.get(0).unwrap());
}
