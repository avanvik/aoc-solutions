use super::*;

#[test]
fn it_parses_string_to_bitvec() {
    let bits = bitvec_from_str("10110");

    assert_eq!(format!("{}", bitvec_to_string(&bits)), "10110");
}

#[test]
fn it_sums_bitvecs() {
    // With test data
    let data = read_file_to_string("test_input.txt");
    let bitvecs = bitvecs_from_string(&data);
    let sum = sum_bitvecs(&bitvecs);

    assert_eq!(bitvec_to_string(&sum), "10110");

    // With real data
    let data = read_file_to_string("real_input.txt");
    let bitvecs = bitvecs_from_string(&data);
    let sum = sum_bitvecs(&bitvecs);

    assert_eq!(bitvec_to_string(&sum), "101001100111")
}

#[test]
fn it_calculates_gamma() {
    // With test data
    let data = read_file_to_string("test_input.txt");
    let bitvecs = bitvecs_from_string(&data);
    let sum = sum_bitvecs(&bitvecs);
    let gamma = calculate_gamma(&sum);

    assert_eq!(gamma, 22);

    // With real data
    let data = read_file_to_string("real_input.txt");
    let bitvecs = bitvecs_from_string(&data);
    let sum = sum_bitvecs(&bitvecs);
    let gamma = calculate_gamma(&sum);

    assert_eq!(gamma, 2663);
}

#[test]
fn it_calculates_epsilon() {
    // With test data
    let data = read_file_to_string("test_input.txt");
    let bitvecs = bitvecs_from_string(&data);
    let sum = sum_bitvecs(&bitvecs);
    let epsilon = calculate_epsilon(&sum);

    assert_eq!(epsilon, 9);

    // With real data
    let data = read_file_to_string("real_input.txt");
    let bitvecs = bitvecs_from_string(&data);
    let sum = sum_bitvecs(&bitvecs);
    let epsilon = calculate_epsilon(&sum);

    assert_eq!(epsilon, 1432);
}

#[test]
fn it_inverts_bitvec() {
    // With test data
    let bitvec = bitvec_from_str("10110");
    let bitvec_inv = invert_bitvec(&bitvec);

    assert_eq!(bitvec_to_string(&bitvec_inv), "01001");

    // With real data
    let bitvec = bitvec_from_str("101001100111");
    let bitvec_inv = invert_bitvec(&bitvec);

    assert_eq!(bitvec_to_string(&bitvec_inv), "010110011000");
}
