use super::puzzle1;

pub fn find_pair_for(intcode: &[u32], result: u32) -> (i32, i32) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode_instance = intcode.to_vec();
            puzzle1::intcode_machine_input(&mut intcode_instance, noun, verb);
            puzzle1::run_computation(&mut intcode_instance);
            if intcode_instance[0] == result {
                return (intcode_instance[1] as i32, intcode_instance[2] as i32);
            }
        }
    }
    (-1, -1)
}
