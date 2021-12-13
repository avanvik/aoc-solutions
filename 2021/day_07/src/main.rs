/*
    ADVENT of CODE - Day 7 "The Treachery of Whales"

    Part 1: Calculate the position where the crabs use the least amount of fuel to move to
    Part 2: Movement now increases exponentially with distance; calculate cost again
*/

fn main() {
    let crabs = get_crabs_from_file("real_input.txt");
    let (optimal_position, optimal_fuel_use) = calculate_optimal_position(&crabs);
    let (optimal_position_exp, optimal_fuel_use_exp) = calculate_optimal_position_exp(&crabs);

    println!(
        "LIN: Optimal position: {}\nOptimal fuel use: {}\n",
        optimal_position, optimal_fuel_use
    );

    println!(
        "EXP: Optimal position: {}\nOptimal fuel use: {}\n",
        optimal_position_exp, optimal_fuel_use_exp
    );
}

fn calculate_optimal_position(crabs: &Vec<usize>) -> (usize, usize) {
    let _average_pos = crabs.iter().fold(0, |acc, curr| acc + curr) / crabs.len();
    let max_pos = crabs
        .iter()
        .fold(0_usize, |acc, curr| if *curr > acc { *curr } else { acc });

    let mut optimal_pos = 0;
    let mut optimal_fuel_cost = usize::MAX;

    for pos in 0..=max_pos {
        let mut fuel_cost = 0;

        for crab in crabs {
            fuel_cost += if crab > &pos { crab - pos } else { pos - crab };
        }

        if fuel_cost < optimal_fuel_cost {
            optimal_pos = pos;
            optimal_fuel_cost = fuel_cost;
        }
    }

    (optimal_pos, optimal_fuel_cost)
}

fn calculate_optimal_position_exp(crabs: &Vec<usize>) -> (usize, usize) {
    let _average_pos = crabs.iter().fold(0, |acc, curr| acc + curr) / crabs.len();
    let max_pos = crabs
        .iter()
        .fold(0_usize, |acc, curr| if *curr > acc { *curr } else { acc });

    let mut optimal_pos = 0;
    let mut optimal_fuel_cost = usize::MAX;

    for pos in 0..=max_pos {
        let mut total_fuel_cost = 0;

        for crab in crabs {
            let diff = if crab > &pos { crab - pos } else { pos - crab };
            let mut crab_fuel_cost = 0;
            for i in 0..=diff {
                crab_fuel_cost += i;
            }

            total_fuel_cost += crab_fuel_cost;
        }

        if total_fuel_cost < optimal_fuel_cost {
            optimal_pos = pos;
            optimal_fuel_cost = total_fuel_cost;
        }
    }

    (optimal_pos, optimal_fuel_cost)
}

fn get_crabs_from_file(path: &str) -> Vec<usize> {
    let data_str = std::fs::read_to_string(path).expect("Could not read file content");

    data_str
        .split(",")
        .map(|s| s.parse().expect("Could not parse number"))
        .collect()
}

#[cfg(test)]
mod tests;
