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

#[cfg(test)]
mod test;
