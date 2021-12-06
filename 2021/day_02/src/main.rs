/*
    ADVENT of CODE - Day 2 "Dive!"

    Part 1: Parse navigation instructions and calculate position product
    Part 2: Calculate position with updated navigation logic
*/

fn main() {
    let input = read_file_to_string("input.txt");
    let instructions = parse_instructions(input);
    let pos = navigate(&instructions);
    let pos_by_aim = navigate_with_aim(&instructions);

    println!("Product: {}", pos.x * pos.y);
    println!("Product with aim: {}", pos_by_aim.x * pos_by_aim.y);
}

fn parse_instructions(input: String) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            if let [cmd, val] = line
                .split(" ")
                .into_iter()
                .collect::<Vec<&str>>()
                .as_slice()
            {
                let val: usize = val.parse().expect("Could not parse value");
                match *cmd {
                    "forward" => Direction::Forward(val),
                    "back" => Direction::Back(val),
                    "up" => Direction::Up(val),
                    "down" => Direction::Down(val),
                    _ => Direction::None,
                }
            } else {
                Direction::None
            }
        })
        .collect()
}

fn navigate(instructions: &Vec<Direction>) -> Position {
    instructions.iter().fold(
        Position { x: 0, y: 0, aim: 0 },
        |pos, instruction| match instruction {
            Direction::Forward(val) => Position {
                x: pos.x + val,
                y: pos.y,
                aim: 0,
            },
            Direction::Back(val) => Position {
                x: pos.x - val,
                y: pos.y,
                aim: 0,
            },
            Direction::Up(val) => Position {
                x: pos.x,
                y: pos.y - val,
                aim: 0,
            },
            Direction::Down(val) => Position {
                x: pos.x,
                y: pos.y + val,
                aim: 0,
            },
            _ => pos,
        },
    )
}

fn navigate_with_aim(instructions: &Vec<Direction>) -> Position {
    instructions.iter().fold(
        Position { x: 0, y: 0, aim: 0 },
        |pos, instruction| match instruction {
            Direction::Forward(val) => Position {
                x: pos.x + val,
                y: pos.y + val * pos.aim,
                aim: pos.aim,
            },
            Direction::Back(val) => Position {
                x: pos.x - val,
                y: pos.y - val * pos.aim,
                aim: pos.aim,
            },
            Direction::Up(val) => Position {
                x: pos.x,
                y: pos.y,
                aim: pos.aim - val,
            },
            Direction::Down(val) => Position {
                x: pos.x,
                y: pos.y,
                aim: pos.aim + val,
            },
            _ => pos,
        },
    )
}
fn read_file_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Could not read file content")
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    aim: usize,
}

#[derive(Debug)]
enum Direction {
    Up(usize),
    Down(usize),
    Forward(usize),
    Back(usize),
    None,
}

#[cfg(test)]
mod test;
