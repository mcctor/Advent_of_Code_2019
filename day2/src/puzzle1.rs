use std::fs::File;
use std::io::Read;

const NEXT_POS: usize = 1;

// OPCODES DECLARATIONS
const EXIT_OPCODE: u32 = 99;
const MULT_OPCODE: u32 = 2;
const ADD_OPCODE: u32 = 1;

enum Computation {
    Success,
    Completed,
    Failed,
}

pub fn intcode_machine_input(intcode: &mut [u32], noun: u32, verb: u32) {
    let frst_pos = 1;
    let snd_pos = 2;

    // replace values at positions with noun and verb
    intcode[frst_pos] = noun;
    intcode[snd_pos] = verb;
}

pub fn restore_gravity_assist(list: &mut [u32]) {
    intcode_machine_input(list, 12, 2);
}

pub fn run_computation(list: &mut [u32]) {
    let mut cur_pos = 0;
    loop {
        match opcode_computation(list, cur_pos) {
            Some(Computation::Completed) => break,
            Some(Computation::Failed) => panic!("Opcode not recognized"),
            Some(Computation::Success) => {
                cur_pos += 4
            }
            _ => {
                panic!("Pretty much sure something went wrong here.");
            }
        }
    }
}

fn opcode_computation(list: &mut [u32], cur_pos: usize) -> Option<Computation> {
    // Operand positions
    let first_operand_pos = cur_pos + NEXT_POS;
    let second_operand_pos = first_operand_pos + NEXT_POS;
    let result_index = second_operand_pos + NEXT_POS;

    let opcode = match list.get(cur_pos) {
        Some(num) => num,
        None => panic!("Couldn't find number")
    };

    match *opcode {
        EXIT_OPCODE => {
            return Some(Computation::Completed);
        }
        MULT_OPCODE => {
            list[(list[result_index]) as usize] =
                list[list[first_operand_pos] as usize] * list[list[second_operand_pos] as usize];
        }
        ADD_OPCODE => {
            list[(list[result_index]) as usize] =
                list[list[first_operand_pos] as usize] + list[list[second_operand_pos] as usize];
        }
        _ => {
            return Some(Computation::Failed);
        }
    }
    Some(Computation::Success)
}

pub fn get_operation_code_list() -> Vec<u32> {
    let contents = read_input_from_file();
    parse_input_to_vec(contents)
}

fn read_input_from_file() -> String {
    let mut file = match File::open("opcode_computer_input.txt") {
        Ok(file) => file,
        Err(_) => panic!("failed to find file")
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn parse_input_to_vec(input: String) -> Vec<u32> {
    let str_vec: Vec<&str> = input.trim_end().split(',').collect();
    let mut int_vec: Vec<u32> = Vec::new();

    for &elem in str_vec.iter() {
        int_vec.push(elem.parse().expect("failed to convert &str to u32"));
    }
    int_vec
}

#[test]
fn test_opcode_computation() {
    let mut input_1 = vec![1, 0, 0, 0, 99];
    let mut input_2 = vec![2, 3, 0, 3, 99];
    let mut input_3 = vec![2, 4, 4, 5, 99, 0];

    run_computation(&mut input_1);
    run_computation(&mut input_2);
    run_computation(&mut input_3);

    assert_eq!(input_1, vec![2, 0, 0, 0, 99]);
    assert_eq!(input_2, vec![2, 3, 0, 6, 99]);
    assert_eq!(input_3, vec![2, 4, 4, 5, 99, 9801]);
}