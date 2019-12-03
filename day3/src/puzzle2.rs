use super::puzzle1::CrossingPoint;

pub fn lowest_number_of_steps(line1: &[&Box<CrossingPoint>], line2: &[&Box<CrossingPoint>]) -> i32 {
    let mut lowest_sum = line1.get(1).unwrap().steps_to + line2.get(1).unwrap().steps_to;
    for (i, _) in line1.iter().enumerate().skip(1) {
        let cur_sum = line1[i].steps_to + line2[i].steps_to;
        if cur_sum < lowest_sum {
            lowest_sum = cur_sum;
        }
    }

    lowest_sum
}