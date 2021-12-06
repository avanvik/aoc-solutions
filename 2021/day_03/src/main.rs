mod bit;
use bit::*;

/*
    ADVENT of CODE - Day 3 "Binary Diagnostic"

    Part 1:
    Part 2:
*/

fn main() {
    let data = read_file_to_string("real_input.txt");
    let bitvecs = bitvecs_from_string(&data);
    let sum = sum_bitvecs(&bitvecs);
    let gamma = calculate_gamma(&sum);
    let epsilon = calculate_epsilon(&sum);

    for bitvec in &bitvecs {
        println!("| {} |", bitvec_to_string(&bitvec));
    }

    println!("\nSUM");
    println!("| {} |\n", bitvec_to_string(&sum));

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);

    println!("Product: {}", gamma * epsilon);

    println!("\nLIFE SUPPORT");

    let o2_rating = get_oxygen_rating(&bitvecs);
    println!(
        "Oxygen rating: {} | Decimal: {}",
        bitvec_to_string(&o2_rating),
        bitvec_to_decimal(&o2_rating)
    );

    let co2_rating = get_co2_rating(&bitvecs);
    println!(
        "CO2 rating: {} | Decimal: {}",
        bitvec_to_string(&co2_rating),
        bitvec_to_decimal(&co2_rating)
    );

    println!(
        "CO2 * O2 = {}",
        bitvec_to_decimal(&o2_rating) * bitvec_to_decimal(&co2_rating)
    )
}

fn read_file_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Could not read file content")
}

// Returns bitvec with dominant values
fn sum_bitvecs(bitvecs: &Vec<Bitvec>) -> Bitvec {
    let mut sum_vec: Vec<isize> = vec![];

    for pos in 0..bitvecs[0].len() {
        let mut sum: isize = 0;
        for bitvec in bitvecs {
            match bitvec[pos] {
                Bit::One => sum += 1,
                Bit::Zero => sum -= 1,
            }
        }
        sum_vec.push(sum);
    }

    sum_vec
        .into_iter()
        .map(|sum| if sum > 0 { Bit::One } else { Bit::Zero })
        .collect()
}

fn bitvecs_from_string(datastring: &String) -> Vec<Bitvec> {
    datastring
        .lines()
        .map(|line| bitvec_from_str(&line))
        .collect()
}

fn bitvec_from_str(bit_string: &str) -> Bitvec {
    bit_string
        .chars()
        .map(|c| {
            let c = &c.to_string()[..];
            match c {
                "1" => Bit::One,
                "0" => Bit::Zero,
                _ => Bit::Zero,
            }
        })
        .collect()
}

fn bitvec_to_string(bits: &Bitvec) -> String {
    bits.iter()
        .fold("".to_string(), |acc, b| format!("{}{}", acc, b))
}

fn bitvec_to_decimal(bitvec: &Bitvec) -> usize {
    let mut decimal_value = 0;
    for (i, bit) in bitvec.iter().rev().enumerate() {
        if let Bit::One = bit {
            let val = 2_usize.pow(i.try_into().unwrap());
            decimal_value += val;
        }
    }
    decimal_value
}

fn invert_bitvec(bitvec: &Bitvec) -> Bitvec {
    bitvec
        .iter()
        .map(|bit| match bit {
            Bit::One => Bit::Zero,
            Bit::Zero => Bit::One,
        })
        .collect()
}

fn calculate_epsilon(bitvec: &Bitvec) -> usize {
    bitvec_to_decimal(&invert_bitvec(&bitvec))
}
fn calculate_gamma(bitvec: &Bitvec) -> usize {
    bitvec_to_decimal(&bitvec)
}

fn get_dominant_bit_at_index(bitvecs: &Vec<Bitvec>, index: usize) -> Bit {
    let mut sum = 0;
    for bitvec in bitvecs {
        match bitvec.get(index).unwrap() {
            Bit::One => sum += 1,
            Bit::Zero => sum -= 1,
        }
    }

    if sum >= 0 {
        Bit::One
    } else {
        Bit::Zero
    }
}

fn get_matching_bitvecs(bitvecs: &Vec<Bitvec>, with_value: Bit, at_position: usize) -> Vec<Bitvec> {
    let mut matches = vec![];

    for bitvec in bitvecs {
        let val = bitvec
            .get(at_position)
            .expect("Could not get bit at position");

        if format!("{:?}", val) == format!("{:?}", with_value) {
            matches.push(bitvec.clone());
        }
    }

    matches
}

fn get_oxygen_rating(bitvecs: &Vec<Bitvec>) -> Bitvec {
    let mut matches = bitvecs.clone();
    let bitvec_length = bitvecs
        .get(0)
        .expect("Could not get bitvec at index 0")
        .len();

    // For each bitvec position, find dominant bit.
    // Then filter list of bitvecs for next iteration.
    for i in 0..bitvec_length {
        let dominant_bit = get_dominant_bit_at_index(&matches, i);
        let new_matches = get_matching_bitvecs(&matches, dominant_bit, i);
        matches = new_matches;

        if matches.len() == 1 {
            break;
        }
    }
    let final_bitvec = matches.get(0).unwrap();
    final_bitvec.clone()
}

fn get_co2_rating(bitvecs: &Vec<Bitvec>) -> Bitvec {
    let mut matches = bitvecs.clone();
    let bitvec_length = bitvecs
        .get(0)
        .expect("Could not get bitvec at index 0")
        .len();

    // For each bitvec position, find dominant bit.
    // Then filter list of bitvecs for next iteration.
    for i in 0..bitvec_length {
        let dominant_bit = get_dominant_bit_at_index(&matches, i).flip();
        let new_matches = get_matching_bitvecs(&matches, dominant_bit, i);
        matches = new_matches;

        if matches.len() == 1 {
            break;
        }
    }
    let final_bitvec = matches.get(0).unwrap();
    final_bitvec.clone()
}

#[cfg(test)]
mod test;
