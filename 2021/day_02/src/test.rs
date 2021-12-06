use super::*;

#[test]
fn it_parses_instructions() {
    let test_input = read_file_to_string("test_input.txt");
    let instructions = vec![
        Direction::Forward(5),
        Direction::Down(5),
        Direction::Forward(8),
        Direction::Up(3),
        Direction::Down(8),
        Direction::Forward(2),
    ];

    let parsed_instructions = parse_instructions(test_input);

    assert_eq!(
        format!("{:?}", parsed_instructions),
        format!("{:?}", instructions)
    );
}

#[test]
fn it_navigates_to_position() {
    let instructions = vec![
        Direction::Forward(5),
        Direction::Down(5),
        Direction::Forward(8),
        Direction::Up(3),
        Direction::Down(8),
        Direction::Forward(2),
    ];

    let position = navigate(&instructions);
    let product = position.x * position.y;

    assert_eq!(product, 150);
}

#[test]
fn it_navigates_to_position_by_aim() {
    let instructions = vec![
        Direction::Forward(5),
        Direction::Down(5),
        Direction::Forward(8),
        Direction::Up(3),
        Direction::Down(8),
        Direction::Forward(2),
    ];

    let position = navigate_with_aim(&instructions);
    let product = position.x * position.y;

    assert_eq!(product, 900);
}

#[test]
fn it_calculates_correct_product() {
    let input = read_file_to_string("input.txt");
    let instructions = parse_instructions(input);
    let pos = navigate(&instructions);

    assert_eq!(2019945, pos.x * pos.y);
}
