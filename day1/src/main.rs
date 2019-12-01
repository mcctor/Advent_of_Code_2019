mod puzzle1;
mod puzzle2;

use std::fs::File;
use std::io::Read;

fn main() {
    let module_masses = get_module_masses();
    let puzzle1_fuel = calculate_sum_of_fuel(&module_masses, &puzzle1::launch_fuel);
    let puzzle2_fuel = calculate_sum_of_fuel(&module_masses, &puzzle2::launch_fuel);

    println!("Puzzle 1 fuel required is {:?}", puzzle1_fuel);
    println!("Puzzle 2 fuel required is {:?}", puzzle2_fuel)
}

fn calculate_sum_of_fuel(module_masses: &[f64], f: &dyn Fn(f64) -> f64) -> f64 {
    let mut sum = 0.0;
    for mass in module_masses.iter() {
        sum += f(*mass);
    }
    sum
}

fn get_module_masses() -> Vec<f64> {
    let file_contents = read_puzzle_input_file();

    convert_input_to_vector(file_contents)
}

fn read_puzzle_input_file() -> String {
    let mut fd = File::open("puzzle_input.txt").expect("cannot find `puzzle_input.txt`");
    let mut contents = String::new();

    fd.read_to_string(&mut contents)
        .expect("failed to read file.");
    contents
}

fn convert_input_to_vector(file_contents: String) -> Vec<f64> {
    let str_vec: Vec<&str> = file_contents.split('\n').collect();
    let mut num_vec = Vec::<f64>::new();

    for number in str_vec.iter() {
        num_vec.push(
            number
                .parse::<f64>()
                .expect("failed to convert &str to f64"),
        );
    }

    num_vec
}
