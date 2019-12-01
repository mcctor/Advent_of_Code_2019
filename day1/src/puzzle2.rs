// So, for each module mass, calculate its fuel and add it to the total.
// Then, treat the fuel amount you just calculated as the input mass and repeat the process,
// continuing until a fuel requirement is zero or negative
use super::puzzle1;

pub fn launch_fuel(module_mass: f64) -> f64 {
    let mut new_mod_mass = module_mass;
    let mut total_fuel = 0.0;

    loop {
        new_mod_mass = puzzle1::launch_fuel(new_mod_mass);
        if new_mod_mass < 0.0 {
            break;
        }
        total_fuel += new_mod_mass;
    }
    total_fuel
}

#[test]
fn test_launch_fuel() {
    let mass_1 = 14.;
    let mass_2 = 100756.;
    let mass_3 = 1969.;

    let expct_answer_1 = 2.;
    let expct_answer_2 = 50346.;
    let expct_answer_3 = 966.;

    assert_eq!(launch_fuel(mass_1), expct_answer_1);
    assert_eq!(launch_fuel(mass_2), expct_answer_2);
    assert_eq!(launch_fuel(mass_3), expct_answer_3);
}
