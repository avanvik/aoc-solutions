use std::collections::HashMap;

/*
    ADVENT of CODE - Day 8 "Seven Segment Search"

    Part 1:
    Part 2:
*/

fn main() {
    let digits = parse_digits_from_file("real_data.txt");
    // let mut count = 0;
    // let parsed_digits: Vec<Vec<usize>> = digits
    //     .iter()
    //     .map(|digit| {
    //         digit
    //             .output
    //             .iter()
    //             .map(|s| {
    //                 if let Some(num) = deduce_num(s) {
    //                     count += 1;
    //                     num
    //                 } else {
    //                     0
    //                 }
    //             })
    //             .collect()
    //     })
    //     .collect();

    // println!("Count: {}", count);

    for line in &digits {
        let map = map_char_to_char_for_display(&line.input);

        // TODO: Print map in alphabetic order
        let mut sorted_map: Vec<_> = map.iter().collect();
        sorted_map.sort_by_key(|v| v.0);

        println!("{:?}", sorted_map);
    }
}

fn _str_to_num(s: &str) -> Option<usize> {
    let mut word: Vec<char> = String::from(s).chars().collect();
    word.sort();

    match String::from_iter(word).as_str() {
        "abcefg" => Some(0),
        "cf" => Some(1),
        "acdeg" => Some(2),
        "acdfg" => Some(3),
        "bcdf" => Some(4),
        "abdfg" => Some(5),
        "abdefg" => Some(6),
        "acf" => Some(7),
        "abcdefg" => Some(8),
        "abcdfg" => Some(0),
        _ => None,
    }
}

/// Output: HashMap(a => c, b => d, c => f, ...)
fn map_char_to_char_for_display(display_inputs: &Vec<String>) -> HashMap<char, char> {
    /*
        num 0  (1)  2   3  (4)  5   6  (7) (8)  9
        len 6   2   5   5   4   5   6   3   7   6

        Segments
        ch  occ rules
        a   8   "7" not "1"
        b   6   (occurs 6 times)
        c   8   (occurs 8 times) and (not e)
        d   7   (in "7" and not in "4")
        e   4   (occurs 4 times) or ("8" not "9")
        f   9   (occurs 9 times) or (2 not (3 and e))
        g   7   (occurs 7 times) and (not d)
    */

    let mut char_map = HashMap::new();

    let segments_in_1: Vec<char> = display_inputs
        .iter()
        .filter(|s| s.len() == 2)
        .flat_map(|s| s.chars())
        .collect();
    let segments_in_4: Vec<char> = display_inputs
        .iter()
        .filter(|s| s.len() == 4)
        .flat_map(|s| s.chars())
        .collect();
    let segments_in_7: Vec<char> = display_inputs
        .iter()
        .filter(|s| s.len() == 3)
        .flat_map(|s| s.chars())
        .collect();
    let _segments_in_8: Vec<char> = display_inputs
        .iter()
        .filter(|s| s.len() == 7)
        .flat_map(|s| s.chars())
        .collect();

    let mut char_occurrences: HashMap<char, usize> = HashMap::new();
    for word in display_inputs {
        for character in word.chars() {
            let val = *char_occurrences.get(&character).unwrap_or(&0);
            char_occurrences.insert(character, val + 1);
        }
    }

    // ch  occ rules
    // a   8   "7" not "1"
    let (seg_a, _) = &char_occurrences
        .clone()
        .into_iter()
        .find(|(c, occ)| occ == &8 && segments_in_7.contains(c) && !segments_in_1.contains(c))
        .expect("Could not find segment A");

    println!(
        "Finding segment A: {:?}",
        &char_occurrences
            .clone()
            .into_iter()
            .filter(|(c, occ)| occ == &8 && segments_in_7.contains(c) && !segments_in_1.contains(c))
            .collect::<Vec<_>>()
    );

    // ch  occ rules
    // b   6   (occurs 6 times)
    let (seg_b, _) = &char_occurrences
        .clone()
        .into_iter()
        .find(|(_, occ)| occ == &6)
        .expect("Could not find segment with 6 occurrences");

    println!(
        "Finding segment B: {:?}",
        &char_occurrences
            .clone()
            .into_iter()
            .filter(|(_, occ)| occ == &6)
            .collect::<Vec<_>>()
    );

    // ch  occ rules
    // e   4   (occurs 4 times) or ("8" not "9")
    let (seg_e, _) = &char_occurrences
        .clone()
        .into_iter()
        .find(|(_, occ)| occ == &4)
        .expect("Could not find segment with 4 occurrences");

    println!(
        "Finding segment E: {:?}",
        &char_occurrences
            .clone()
            .into_iter()
            .filter(|(_, occ)| occ == &4)
            .collect::<Vec<_>>()
    );

    // ch  occ rules
    // c   8   (occurs 8 times) and (not a)
    let (seg_c, _) = &char_occurrences
        .clone()
        .into_iter()
        .filter(|(c, occ)| occ == &8 && *c != *seg_a)
        .last()
        .expect("Could not find segment for 'c'");

    println!(
        "Finding segment C: {:?}",
        &char_occurrences
            .clone()
            .into_iter()
            .filter(|(_c, occ)| occ == &8)
            .filter(|(c, _occ)| *c != *seg_a)
            .collect::<Vec<_>>()
    );

    // ch  occ rules
    // f   9   (occurs 9 times) or (2 not (3 and e))
    let (seg_f, _) = &char_occurrences
        .clone()
        .into_iter()
        .find(|(_, occ)| occ == &9)
        .expect("Could not find segment with 9 occurrences");

    println!(
        "Finding segment F: {:?}",
        &char_occurrences
            .clone()
            .into_iter()
            .filter(|(_, occ)| occ == &9)
            .collect::<Vec<_>>()
    );

    // ch  occ rules
    // d   7   (in "7" and not in "4")
    let (seg_d, _) = &char_occurrences
        .clone()
        .into_iter()
        .find(|(c, occ)| occ == &7 && !segments_in_7.contains(c) && segments_in_4.contains(c))
        .expect("Could not find segment D");

    println!(
        "Finding segment D: {:?}",
        &char_occurrences
            .clone()
            .into_iter()
            .filter(|(c, occ)| occ == &7 && !segments_in_7.contains(c) && segments_in_4.contains(c))
            .collect::<Vec<_>>()
    );

    // ch  occ rules
    // g   7   (occurs 7 times) and (not c)
    let (seg_g, _) = &char_occurrences
        .clone()
        .into_iter()
        .find(|(c, occ)| occ == &7 && *c != *seg_d)
        .expect("Could not find segment with ---");

    println!(
        "Finding segment G: {:?}",
        &char_occurrences
            .clone()
            .into_iter()
            .filter(|(c, occ)| occ == &7 && c != seg_d)
            .collect::<Vec<_>>()
    );

    char_map.insert('a', *seg_a);
    char_map.insert('b', *seg_b);
    char_map.insert('c', *seg_c);
    char_map.insert('d', *seg_d);
    char_map.insert('e', *seg_e);
    char_map.insert('f', *seg_f);
    char_map.insert('g', *seg_g);

    char_map
}

fn _num_from_len(s: &str) -> Option<usize> {
    match s.len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

fn parse_digits_from_file(path: &str) -> Vec<Digit> {
    let data_str = std::fs::read_to_string(path).expect("Could not read file content");

    data_str
        .lines()
        .map(|line| {
            let data: Vec<&str> = line.split(" | ").collect();

            Digit {
                input: data[0].split(" ").map(|d| String::from(d)).collect(),
                _output: data[1].split(" ").map(|d| String::from(d)).collect(),
            }
        })
        .collect()
}

struct Digit {
    input: Vec<String>,
    _output: Vec<String>,
}

#[cfg(test)]
mod tests;
