use super::puzzle1;

pub fn find_pair_for(intcode: &[u32], result: u32) -> (i32, i32) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode_copy = intcode.to_vec();
            puzzle1::intcode_machine_input(&mut intcode_copy, noun, verb);
            puzzle1::run_computation(&mut intcode_copy);
            if intcode_copy[0] == result {
                return (intcode_copy[1] as i32, intcode_copy[2] as i32);
            }
        }
    }
    (-1, -1)
}