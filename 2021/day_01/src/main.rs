mod input;

fn main() {
    let increases = count_depth_increases(input::REAL_INPUT, 1);
    let increases_sliding_window = count_depth_increases(input::REAL_INPUT, 3);
    println!("Measured {increases} depth increases");
    println!("Measured {increases_sliding_window} sliding window depth increases");
}

// fn count_depth_increases_fold(measurements: &str, window_size: usize) -> usize {
//     let data = measurements
//         .lines()
//         .map(|d| d.parse::<usize>().expect("Could not parse value"))
//         .into_iter();

//     let sum = data.fold(
//         (Vec::<usize>::new(), 0),
//         |(window, increases), measurement| {
//             if window.len() > window_size {
//                 let prev_sum: usize = window.iter().sum();
//                 let current_sum: usize = new_window.iter().sum();

//                 let mut new_window = window.clone();
//                 new_window.pop();
//                 new_window = vec![current_sum,...new_window];

//                 let new_increase = if prev_sum > current_sum {
//                     increases + 1
//                 } else {
//                     increases
//                 };

//                 (new_window, new_increase)
//             } else {
//                 // Otherwise, add current measurement to window and move on
//                 (window, increases)
//             }
//         },
//     );

//     // let increases = sum.3;
//     // increases

//     sum.1
// }

fn count_depth_increases(measurements: &str, sliding_window_size: usize) -> usize {
    let data: Vec<usize> = measurements
        .lines()
        .map(|d| d.parse::<usize>().expect("Could not parse value"))
        .collect();

    let mut count = 0;
    let mut previous_measurement = 0;

    for i in 1..=(data.len() - sliding_window_size) {
        let mut measurement = 0;

        for j in i..(i + sliding_window_size) {
            measurement += data.get(j).unwrap();
        }

        if measurement > previous_measurement {
            count = count + 1;
        };

        previous_measurement = measurement;
    }
    count
}

// #[cfg(test)]
// mod fold_tests {
//     use crate::count_depth_increases_fold;
//     use crate::input::{TEST_INPUT, TEST_INPUT_SLIDING_WINDOW};

//     #[test]
//     fn it_counts_increases() {
//         assert_eq!(count_depth_increases_fold(TEST_INPUT, 1), 7);
//     }

//     #[test]
//     fn it_sliding_window_counts_increases() {
//         assert_eq!(count_depth_increases_fold(TEST_INPUT_SLIDING_WINDOW, 3), 5);
//     }
// }

#[cfg(test)]
mod tests {
    use crate::count_depth_increases;
    use crate::input::{TEST_INPUT, TEST_INPUT_SLIDING_WINDOW};

    #[test]
    fn it_counts_increases() {
        assert_eq!(count_depth_increases(TEST_INPUT, 1), 7);
    }

    #[test]
    fn it_sliding_window_counts_increases() {
        assert_eq!(count_depth_increases(TEST_INPUT_SLIDING_WINDOW, 3), 5);
    }
}
