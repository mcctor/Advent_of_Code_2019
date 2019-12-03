mod puzzle1;

fn main() {
    let crossing_points = puzzle1::run_instructions();
    println!("{:?}", crossing_points);

    let distances = puzzle1::calc_manhattan_distances(&crossing_points);
    println!("The answer is {:?}", puzzle1::min(&distances));
}
