use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// MOVEMENT BYTES DECLARATION
const UP_INSTR: u8 = b'U';
const DOWN_INSTR: u8 = b'D';
const LEFT_INSTR: u8 = b'L';
const RIGHT_INSTR: u8 = b'R';

#[derive(Debug)]
struct Position {
    row: i32,
    column: i32,
    pub history: Vec<(i32, i32, i32)>,
    tot_steps: i32
}

impl Position {
    fn new() -> Position {
        Position {
            row: 0,
            column: 0,
            tot_steps: 0,
            history: Vec::with_capacity(100000000),
        }
    }

    fn process_instruction(&mut self, instr: &str) {
        let (instr_char, steps) = instr.split_at(1);
        let instr_char = instr_char.as_bytes()[0];
        let steps = steps.parse::<i32>().unwrap();

        match instr_char {
            UP_INSTR => {
                self.move_up(steps);
            }
            DOWN_INSTR => {
                self.move_down(steps);
            }
            LEFT_INSTR => {
                self.move_left(steps);
            }
            RIGHT_INSTR => {
                self.move_right(steps);
            }
            _ => {}
        }
    }

    fn record_history(&mut self, coordinates: (i32, i32, i32)) {
        self.history.push(coordinates);
    }

    fn move_left(&mut self, steps: i32) -> (i32, i32) {
        for _step in 0..steps {
            self.record_history((self.row, self.column, self.tot_steps));
            self.tot_steps += 1;
            self.column -= 1;
        }
        (self.row, self.column)
    }

    fn move_right(&mut self, steps: i32) -> (i32, i32) {
        for _step in 0..steps {
            self.record_history((self.row, self.column, self.tot_steps));
            self.tot_steps += 1;
            self.column += 1;
        }
        (self.row, self.column)
    }

    fn move_up(&mut self, steps: i32) -> (i32, i32) {
        for _step in 0..steps {
            self.record_history((self.row, self.column, self.tot_steps));
            self.tot_steps += 1;
            self.row += 1;
        }
        (self.row, self.column)
    }

    fn move_down(&mut self, steps: i32) -> (i32, i32) {
        for _step in 0..steps {
            self.record_history((self.row, self.column, self.tot_steps));
            self.tot_steps += 1;
            self.row -= 1;
        }
        (self.row, self.column)
    }
}


pub fn run_instructions() -> HashMap<String, Vec<(i32, i32, i32)>> {
    let mut line1 = Position::new();
    let mut line2 = Position::new();

    let instructions = read_mov_instr_from_file();
    let (line1_instr, line2_instr) = mov_instr_to_vec(&instructions);

    process_instr_for_line(&mut line1, line1_instr);
    process_instr_for_line(&mut line2, line2_instr);

    find_crossing_points(&mut line1, &mut line2)
}

fn find_crossing_points(line1: &mut Position, line2: &mut Position) -> HashMap<String, Vec<(i32, i32, i32)>> {
    let mut crossing_points = HashMap::new();
    let mut crossing_points_line1: Vec<(i32, i32, i32)> = Vec::new();
    let mut crossing_points_line2: Vec<(i32, i32, i32)> = Vec::new();
    for &line1_coord in line1.history.iter() {
        for &line2_coord in line2.history.iter() {
            if is_crossing_point_for(line1_coord, line2_coord) {
                crossing_points_line1.push(line1_coord);
                crossing_points_line2.push(line2_coord);
            }
        }
    }
    crossing_points.insert("line1".to_string(), crossing_points_line1);
    crossing_points.insert("line2".to_string(), crossing_points_line2);
    crossing_points
}

fn is_crossing_point_for(line1: (i32, i32, i32), line2: (i32, i32, i32)) -> bool {
    (line1.0 == line2.0) && (line1.1 == line2.1)
}

fn process_instr_for_line(line: &mut Position, instr: Vec<&str>) {
    for &each_instr in instr.iter() {
        line.process_instruction(each_instr);
    }
}

fn mov_instr_to_vec(file_contents: &String) -> (Vec<&str>, Vec<&str>) {
    let instr_vec: Vec<&str> = file_contents.split("\n").collect();

    let frst_line_vec = instr_vec[0].split(",").collect::<Vec<&str>>();
    let snd_line_vec = instr_vec[1].split(",").collect::<Vec<&str>>();

    (frst_line_vec, snd_line_vec)
}

fn read_mov_instr_from_file() -> String {
    let mut file = File::open("movement_instructions.txt")
        .expect("Could not open/find file.");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    file_contents
}

pub fn calc_manhattan_distances(points: &[(i32, i32, i32)]) -> Vec<f64> {
    let origin = points[0];

    let mut manhattan_distances = vec![];
    for &point in points.iter() {
        let distance = {
            ((point.0 as f64).abs() - (origin.0 as f64).abs()) +
                ((point.1 as f64).abs() - (origin.1 as f64).abs())
        };
        manhattan_distances.push(distance);
    }

    manhattan_distances
}

pub fn min(list: &[f64]) -> f64 {
    let mut smallest = list[1];

    for &elem in list.iter().skip(1) {
        if elem < smallest {
            smallest = elem
        }
    }
    smallest
}
