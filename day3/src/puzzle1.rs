use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// MOVEMENT BYTES DECLARATION
const UP_INSTR: u8 = b'U';
const DOWN_INSTR: u8 = b'D';
const LEFT_INSTR: u8 = b'L';
const RIGHT_INSTR: u8 = b'R';

#[derive(Debug)]
pub struct CrossingPoint {
    pub x: i32,
    pub y: i32,
    pub steps_to: i32,
}

#[derive(Debug)]
pub struct Position {
    row: i32,
    column: i32,
    history: Vec<Box<CrossingPoint>>,
    tot_steps: i32,
}

impl Position {
    pub fn new() -> Position {
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
        let crossing_point = CrossingPoint {
            x: coordinates.0,
            y: coordinates.1,
            steps_to: coordinates.2,
        };
        self.history.push(Box::new(crossing_point));
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


pub fn run_instructions<'a>(line1: &'a mut Position, line2: &'a mut Position) -> HashMap<String, Vec<&'a Box<CrossingPoint>>> {
    let instructions = read_mov_instr_from_file();
    let (line1_instr, line2_instr) = mov_instr_to_vec(&instructions);

    process_instr_for_line(line1, line1_instr);
    process_instr_for_line(line2, line2_instr);

    find_crossing_points(line1, line2)
}

fn find_crossing_points<'a>(line1: &'a mut Position, line2: &'a mut Position) -> HashMap<String, Vec<&'a Box<CrossingPoint>>> {
    let mut crossing_points = HashMap::new();
    let mut crossing_points_line1 = Vec::new();
    let mut crossing_points_line2 = Vec::new();
    for line1_coord in &line1.history {
        for line2_coord in line2.history.iter() {
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

fn is_crossing_point_for(line1: &Box<CrossingPoint>, line2: &Box<CrossingPoint>) -> bool {
    (line1.x == line2.x) && (line1.y == line2.y)
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

pub fn calc_manhattan_distances(points: &[&Box<CrossingPoint>]) -> Vec<f64> {
    let origin = points.get(0).unwrap();

    let mut manhattan_distances = vec![];
    for &point in points.iter() {
        let distance = {
            ((point.x as f64).abs() - (origin.x as f64).abs()) +
                ((point.y as f64).abs() - (origin.y as f64).abs())
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
