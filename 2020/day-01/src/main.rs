mod input;

use input::INPUT;

fn main() {
    let mut all_numbers: Vec<i32> = vec![];
    let input_vec: Vec<&str> = INPUT.lines().collect();

    for line in &input_vec {
        match line.parse::<i32>() {
            Ok(number) => all_numbers.push(number),
            Err(_) => (),
        }
    }

    let mut correct_answer: Option<(i32, i32)> = None;

    // Loop throught all numbers
    for number_a in &all_numbers {
        // If we now have a solution
        if let Some((number_a, number_b)) = correct_answer {
            println!(
                "CORRECT ANSWER IS {} * {} = {}",
                &number_a,
                &number_b,
                &number_a * &number_b
            );
            break;
        }

        // Check current number against all other numbers
        for number_b in &all_numbers {
            if number_a + number_b == 2020 {
                correct_answer = Some((*number_a, *number_b));
            }
        }
    }
}
