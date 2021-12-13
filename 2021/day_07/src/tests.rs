use super::*;

#[test]
fn it_calculates_optimal_position() {
    let crabs = get_crabs_from_file("test_input.txt");
    let (_optimal_position, optimal_fuel_cost) = calculate_optimal_position(&crabs);

    assert_eq!(optimal_fuel_cost, 37);
}

#[test]
fn it_calculates_optimal_position_exp() {
    let crabs = get_crabs_from_file("test_input.txt");
    let (_optimal_position, optimal_fuel_cost) = calculate_optimal_position_exp(&crabs);

    assert_eq!(optimal_fuel_cost, 168);
}
