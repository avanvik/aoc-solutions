mod input;

/*
    ADVENT of CODE - Day 1

    Part 1: Count each depth increase
    Part 2: Count each sliding window depth increase
*/

fn main() {
    let increases = count_depth_increases(input::REAL_INPUT, 1);
    let increases_sliding_window = count_depth_increases(input::REAL_INPUT, 3);
    println!("Measured {increases} depth increases");
    println!("Measured {increases_sliding_window} sliding window depth increases");
}

fn count_depth_increases(measurements: &str, window_size: usize) -> usize {
    let data: Vec<usize> = measurements
        .lines()
        .map(|d| d.parse::<usize>().expect("Could not parse value"))
        .collect();

    let (_, sum): (usize, usize) = data
        .iter()
        .as_slice()
        .windows(window_size)
        .map(|slc| slc.iter().sum())
        .fold((usize::MAX, 0), |(last_val, inc), curr_val| {
            let incrementation = if curr_val > last_val { 1 } else { 0 };
            (curr_val, inc + incrementation)
        });

    sum
}

#[cfg(test)]
mod tests {
    use crate::count_depth_increases;
    use crate::input::{REAL_INPUT, TEST_INPUT, TEST_INPUT_SLIDING_WINDOW};

    #[test]
    fn it_counts_increases() {
        assert_eq!(count_depth_increases(TEST_INPUT, 1), 7);
    }

    #[test]
    fn it_counts_increases_with_real_data() {
        assert_eq!(count_depth_increases(REAL_INPUT, 1), 1557);
    }

    #[test]
    fn it_sliding_window_counts_increases_with_real_data() {
        assert_eq!(count_depth_increases(TEST_INPUT_SLIDING_WINDOW, 3), 5);
    }

    #[test]
    fn it_sliding_window_counts_increases() {
        assert_eq!(count_depth_increases(REAL_INPUT, 3), 1608);
    }
}
