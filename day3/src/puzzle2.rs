pub fn lowest_number_of_steps(line1: &[(i32, i32, i32)], line2: &[(i32, i32, i32)]) -> i32 {
    let mut lowest_sum = line1[1].2 + line2[1].2;
    for (i, _) in line1.iter().enumerate().skip(1) {
        let cur_sum = line1[i].2 + line2[i].2;
        if cur_sum < lowest_sum {
            lowest_sum = cur_sum;
        }
    }

    lowest_sum
}