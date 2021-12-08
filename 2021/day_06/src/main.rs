/*
    ADVENT of CODE - Day 6 "Lanternfish"

    Part 1:
    Part 2:
*/

fn main() {
    let mut fishes = get_fish_from_file("real_data.txt");
    simulate_fish_after_days(&mut fishes, 256);

    let amount_of_fish = get_amount_of_fish(fishes);

    println!("{}", amount_of_fish);
}

struct Fish {
    amount: usize,
    age: usize,
}

fn get_fish_from_file(path: &str) -> Vec<Fish> {
    let data_str = std::fs::read_to_string(path).expect("Could not read file content");

    data_str
        .split(",")
        .map(|s| Fish {
            amount: 1,
            age: s.parse().expect("Could not parse number"),
        })
        .collect()
}

fn simulate_fish_after_days(fishes: &mut Vec<Fish>, duration: usize) -> &Vec<Fish> {
    for _day in 0..duration {
        let mut newborn = 0;
        for i in 0..fishes.len() {
            if fishes[i].age == 0 {
                fishes[i].age = 7;
                newborn += fishes[i].amount;
            }
            fishes[i].age -= 1;
        }

        fishes.push(Fish {
            age: 8,
            amount: newborn,
        });
    }

    fishes
}

fn get_amount_of_fish(fish: Vec<Fish>) -> usize {
    let mut amount = 0;
    for f in fish {
        amount += f.amount;
    }
    amount
}

#[cfg(test)]
mod tests;
