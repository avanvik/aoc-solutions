use super::*;

#[test]
fn it_calculates_fish_growth() {
    // Part 1
    let mut fishes = get_fish_from_file("test_data.txt");
    simulate_fish_after_days(&mut fishes, 80);
    let amount_of_fish = get_amount_of_fish(fishes);
    assert_eq!(amount_of_fish, 5934);

    let mut fishes = get_fish_from_file("real_data.txt");
    simulate_fish_after_days(&mut fishes, 80);
    let amount_of_fish = get_amount_of_fish(fishes);
    assert_eq!(amount_of_fish, 362740);

    // Part 2
    let mut fishes = get_fish_from_file("test_data.txt");
    simulate_fish_after_days(&mut fishes, 256);
    let amount_of_fish = get_amount_of_fish(fishes);
    assert_eq!(amount_of_fish, 26984457539);

    let mut fishes = get_fish_from_file("real_data.txt");
    simulate_fish_after_days(&mut fishes, 256);
    let amount_of_fish = get_amount_of_fish(fishes);
    assert_eq!(amount_of_fish, 1644874076764);
}
