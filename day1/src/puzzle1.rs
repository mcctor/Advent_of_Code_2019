// Fuel required to launch a given module is based on its mass.
// Specifically, to find the fuel required for a module, take its mass,
// divide by three, round down, and subtract 2.
pub fn launch_fuel(module_mass: f64) -> f64 {
    (module_mass / 3.0).floor() - 2.0
}

#[test]
fn test_launch_fuel() {
    let mass_1 = 12.;
    let mass_2 = 14.;
    let mass_3 = 1969.;

    let expct_answer_1 = 2.;
    let expct_answer_2 = 2.;
    let expct_answer_3 = 654.;

    assert_eq!(launch_fuel(mass_1), expct_answer_1);
    assert_eq!(launch_fuel(mass_2), expct_answer_2);
    assert_eq!(launch_fuel(mass_3), expct_answer_3);
}
