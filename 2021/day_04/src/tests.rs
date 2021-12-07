use super::*;

#[test]
fn it_marks_cell_in_board() {
    let mut boards = read_boards_from_file("test_data_boards.txt");

    assert_eq!(boards[0].get_cell(0, 2).unwrap().marked, false);
    boards[0].mark_cell_with_number(17);
    assert_eq!(boards[0].get_cell(0, 2).unwrap().value, 17);
    assert_eq!(boards[0].get_cell(0, 2).unwrap().marked, true);
}

#[test]
fn it_calculates_winner_board_score() {
    let mut boards = read_boards_from_file("test_data_boards.txt");
    let numbers = read_numbers_from_file("test_data_numbers.txt");
    let highscores = calculate_highscore_list(&mut boards, numbers);

    assert_eq!(highscores[0], 4512);

    let mut boards = read_boards_from_file("real_data_boards.txt");
    let numbers = read_numbers_from_file("real_data_numbers.txt");
    let highscores = calculate_highscore_list(&mut boards, numbers);

    assert_eq!(highscores[0], 35711);
}

#[test]
fn it_calculates_loser_board_score() {
    let mut boards = read_boards_from_file("test_data_boards.txt");
    let numbers = read_numbers_from_file("test_data_numbers.txt");
    let highscores = calculate_highscore_list(&mut boards, numbers);

    let last_score_entry = *highscores.last().unwrap();
    assert_eq!(last_score_entry, 1924);
}
