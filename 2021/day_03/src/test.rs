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

#[test]
fn it_gets_oxygen_rating() {
    // With test data
    let data = read_file_to_string("test_input.txt");
    let bitvecs = bitvecs_from_string(&data);

    assert_eq!(bitvec_to_string(&get_oxygen_rating(&bitvecs)), "10111");
}

#[test]
fn it_gets_co2_rating() {
    // With test data
    let data = read_file_to_string("test_input.txt");
    let bitvecs = bitvecs_from_string(&data);

    assert_eq!(bitvec_to_string(&get_co2_rating(&bitvecs)), "01010");
}

#[test]
fn it_gets_gets_dominant_bit_at_index() {
    // With test data
    let data = read_file_to_string("test_input.txt");
    let bitvecs = bitvecs_from_string(&data);

    let dominant_bitvec = get_dominant_bit_at_index(&bitvecs, 0);
    assert_eq!(format!("{:?}", dominant_bitvec), format!("{:?}", Bit::One));

    let dominant_bitvec = get_dominant_bit_at_index(&bitvecs, 1);
    assert_eq!(format!("{:?}", dominant_bitvec), format!("{:?}", Bit::Zero));

    let dominant_bitvec = get_dominant_bit_at_index(&bitvecs, 2);
    assert_eq!(format!("{:?}", dominant_bitvec), format!("{:?}", Bit::One));

    let dominant_bitvec = get_dominant_bit_at_index(&bitvecs, 3);
    assert_eq!(format!("{:?}", dominant_bitvec), format!("{:?}", Bit::One));

    let dominant_bitvec = get_dominant_bit_at_index(&bitvecs, 4);
    assert_eq!(format!("{:?}", dominant_bitvec), format!("{:?}", Bit::Zero));
}

#[test]
fn it_gets_filters_bitvecs() {
    // With test data
    let data = read_file_to_string("test_input.txt");
    let bitvecs = bitvecs_from_string(&data);

    let matching_bitvecs = get_matching_bitvecs(&bitvecs, Bit::Zero, 0);
    assert_eq!(format!("{:?}", matching_bitvecs), "[[Zero, Zero, One, Zero, Zero], [Zero, One, One, One, One], [Zero, Zero, One, One, One], [Zero, Zero, Zero, One, Zero], [Zero, One, Zero, One, Zero]]");

    let matching_bitvecs = get_matching_bitvecs(&bitvecs, Bit::One, 1);
    assert_eq!(format!("{:?}", matching_bitvecs), "[[One, One, One, One, Zero], [Zero, One, One, One, One], [One, One, One, Zero, Zero], [One, One, Zero, Zero, One], [Zero, One, Zero, One, Zero]]");
}
