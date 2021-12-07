mod board;
use board::{Board, BoardCell};

/*
    ADVENT of CODE - Day 4 "Giant Squid"

    Part 1:
        - Pull numbers from list
        - Find matches in boards
        - Mark board as winner if a board gets a full row or column
        - Sum all unmarked numbers from board
        - Multiply board sum with winning number

    Part 2:
        - Get score of last winning board (the guaranteed-to-lose board)
*/

fn main() {
    let mut boards = read_boards_from_file("real_data_boards.txt");
    let numbers: Vec<usize> = read_numbers_from_file("real_data_numbers.txt");
    let highscores = calculate_highscore_list(&mut boards, numbers);

    println!("Winner score: {}", highscores[0]);
    println!("Loser score: {}", highscores.last().unwrap());
}

fn split_string_to_board_strings(data_string: &str) -> Vec<&str> {
    data_string.split("\n\n").collect() // split string by empty lines
}

fn parse_string_to_board(board_string: &str) -> Board {
    let lines: Vec<Vec<BoardCell>> = board_string
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|s| *s != "")
                .map(|number| BoardCell {
                    value: number.parse().unwrap(),
                    marked: false,
                })
                .collect()
        })
        .collect();

    Board::new(lines)
}

fn read_file_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Could not read file content")
}

fn read_boards_from_file(path: &str) -> Vec<Board> {
    let data_string = read_file_to_string(path);
    let board_strings = split_string_to_board_strings(&data_string);
    let boards = board_strings
        .iter()
        .map(|s| parse_string_to_board(s))
        .collect();

    boards
}

fn read_numbers_from_file(path: &str) -> Vec<usize> {
    read_file_to_string(path)
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calculate_highscore_list(boards: &mut Vec<Board>, numbers: Vec<usize>) -> Vec<usize> {
    let boards_len = boards.len();
    let mut highscores: Vec<usize> = vec![];

    'rounds: for number in numbers {
        for board in boards.iter_mut() {
            let (old_row, old_col) = board.get_marked().clone();
            board.mark_cell_with_number(number);
            let (row, col) = board.get_marked();

            if (old_col < 5 && old_row < 5 && row == 5) || (old_row < 5 && old_col < 5 && col == 5)
            {
                highscores.push(board.get_board_score(number));
                if highscores.len() == boards_len {
                    break 'rounds;
                }
            };
        }
    }
    highscores
}

#[cfg(test)]
mod tests;
