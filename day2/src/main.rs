use std::time::Instant;

mod puzzle1;
mod puzzle2;

fn main() {
    puzzle_1();
    puzzle_2();
}

fn puzzle_1() {
    let mut op_code_list = puzzle1::get_operation_code_list();
    puzzle1::restore_gravity_assist(&mut op_code_list);
    puzzle1::run_computation(&mut op_code_list);
    println!("Puzzle1 => Value at position 0 is: {}", op_code_list.get(0).unwrap());
}

fn puzzle_2() {
    let expected_result = 19690720;
    let puzzle2_solution = |noun, verb| (100 * noun) + verb;
    let mut op_code_list = puzzle1::get_operation_code_list();

    let now = Instant::now();
    let (noun, verb) = puzzle2::find_pair_for(&mut op_code_list, expected_result);
    println!("Puzzle2 executed in {} ms => Result is: {}",
             now.elapsed().as_millis(),
             puzzle2_solution(noun, verb)
    );
}
