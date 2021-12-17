use std::collections::HashMap;

/*
    ADVENT of CODE - Day 8 "Seven Segment Search"

    Part 1:
    Part 2:
*/

const DEBUG: bool = false;

fn main() {
    let digits = parse_digits_from_file("real_data.txt");
    let mut sum = 0;

    for digit in &digits {
        let mut built_num: Vec<String> = vec![];

        let char_map = map_chars_for_display(&digit.input);

        if DEBUG {
            println!("{:?} | {:?}", &digit.input, &digit.output);
            println!("{:?}", char_map);
        }

        for num in &digit.output {
            let decoded_str = convert_str_with_map(&char_map, &num);
            let decoded_num = str_to_num(&decoded_str).expect("Could parse segments as number");

            built_num.push(decoded_num.to_string());
        }

        let segment_num = built_num
            .iter()
            .flat_map(|c| c.chars())
            .collect::<String>()
            .parse::<usize>()
            .expect("Could not parse number");

        sum += segment_num;
    }

    println!("SUM {}", sum);
}

fn str_to_num(s: &str) -> Option<usize> {
    let mut word: Vec<char> = String::from(s).chars().collect();
    word.sort();

    if DEBUG {
        println!("Sorted: {:?}", word);
    }

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
        "abcdfg" => Some(9),
        _ => None,
    }
}

/// Output: HashMap(a => c, b => d, c => f, ...)
fn map_chars_for_display(display_inputs: &Vec<String>) -> HashMap<char, char> {
    /*
        num 0  (1)  2   3  (4)  5   6  (7) (8)  9
        len 6   2   5   5   4   5   6   3   7   6

        Segments
        ch  occ rules
        a   8   in "7" and not in "1"
        b  (6)  (occurs 6 times)
        c   8   (occurs 8 times) and (not e)
        d   7   in "7" and not in "4"
        e  (4)  (occurs 4 times) or ("8" not "9")
        f  (9)  (occurs 9 times) or (2 not (3 and e))
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

    let mut char_occurrences: HashMap<char, usize> = HashMap::new();

    for word in display_inputs {
        for character in word.chars() {
            let val = *char_occurrences.get(&character).unwrap_or(&0);
            char_occurrences.insert(character, val + 1);
        }
    }

    let seg_a = &char_occurrences
        .clone()
        .into_iter()
        .filter(|(c, occ)| occ == &8 && segments_in_7.contains(c) && !segments_in_1.contains(c))
        .collect::<Vec<_>>();

    if DEBUG {
        println!("SEG A: {:?}", seg_a);
    }
    let seg_a = seg_a.last().expect("Could not find segment A");

    let seg_b = &char_occurrences
        .clone()
        .into_iter()
        .filter(|(_, occ)| occ == &6)
        .collect::<Vec<_>>();

    if DEBUG {
        println!("SEG B: {:?}", seg_b)
    };
    let seg_b = seg_b.last().expect("Could not find segment B");

    let seg_e = &char_occurrences
        .clone()
        .into_iter()
        .filter(|(_, occ)| occ == &4)
        .collect::<Vec<_>>();

    if DEBUG {
        println!("SEG E: {:?}", seg_e);
    }
    let seg_e = seg_e.last().expect("Could not find segment E");

    let seg_c = &char_occurrences
        .clone()
        .into_iter()
        .filter(|(c, occ)| occ == &8 && c != &seg_a.0)
        .collect::<Vec<_>>();

    if DEBUG {
        println!("SEG C: {:?}", seg_c);
    }
    let seg_c = seg_c.last().expect("Could not find segment C");

    let seg_f = &char_occurrences
        .clone()
        .into_iter()
        .filter(|(_, occ)| occ == &9)
        .collect::<Vec<_>>();

    if DEBUG {
        println!("SEG F: {:?}", seg_f);
    }
    let seg_f = seg_f.last().expect("Could not find segment F");

    let seg_d = &char_occurrences
        .clone()
        .into_iter()
        .filter(|(c, occ)| occ == &7 && !segments_in_7.contains(c) && segments_in_4.contains(c))
        .collect::<Vec<_>>();

    if DEBUG {
        println!("SEG D: {:?}", seg_d);
    }
    let seg_d = seg_d.last().expect("Could not find segment D");

    let seg_g = &char_occurrences
        .clone()
        .into_iter()
        .filter(|(c, occ)| occ == &7 && c != &seg_d.0)
        .collect::<Vec<_>>();

    if DEBUG {
        println!("SEG G: {:?}", seg_g);
    }
    let seg_g = seg_g.last().expect("Could not find segment G");

    char_map.insert(seg_a.0, 'a');
    char_map.insert(seg_b.0, 'b');
    char_map.insert(seg_c.0, 'c');
    char_map.insert(seg_d.0, 'd');
    char_map.insert(seg_e.0, 'e');
    char_map.insert(seg_f.0, 'f');
    char_map.insert(seg_g.0, 'g');

    char_map
}

fn convert_str_with_map(char_map: &HashMap<char, char>, input: &String) -> String {
    input
        .clone()
        .chars()
        .into_iter()
        .map(|c| char_map.get(&c).unwrap())
        .collect()
}

fn parse_digits_from_file(path: &str) -> Vec<Digit> {
    let data_str = std::fs::read_to_string(path).expect("Could not read file content");

    data_str
        .lines()
        .map(|line| {
            let data: Vec<&str> = line.split(" | ").collect();

            Digit {
                input: data[0].split(" ").map(|d| String::from(d)).collect(),
                output: data[1].split(" ").map(|d| String::from(d)).collect(),
            }
        })
        .collect()
}

struct Digit {
    input: Vec<String>,
    output: Vec<String>,
}

#[cfg(test)]
mod tests;
