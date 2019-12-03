mod puzzle1;
mod puzzle2;


fn main() {
    puzzle_1();
}

fn puzzle_1() {
    let line_crossing_points = puzzle1::run_instructions();

    let line1_crossing_points = line_crossing_points.get("line1").unwrap();
    let line2_crossing_points = line_crossing_points.get("line2").unwrap();

    let distances = puzzle1::calc_manhattan_distances(line1_crossing_points);
    println!("Puzzle 1 answer is {:?}", puzzle1::min(&distances));

    println!("Puzzle 2 answer is {}", puzzle2::lowest_number_of_steps(line1_crossing_points, line2_crossing_points));
}