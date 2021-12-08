use super::*;

#[test]
fn it_calculates_overlaps_of_non_slanted_lines() {
    let lines = build_lines_from_file("test_input.txt");
    let non_slanted_lines: Vec<&Line> = lines
        .iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        .collect();

    let (max_x, max_y) = get_maxima(&lines);
    let mut grid = Grid::new(max_y + 1, max_x + 1);

    for line in non_slanted_lines {
        grid.draw_line(&line);
    }

    let overlaps = grid.get_overlaps();

    assert_eq!(overlaps, 5);
}

#[test]
fn it_calculates_overlaps_of_all_lines() {
    let lines = build_lines_from_file("test_input.txt");
    let (max_x, max_y) = get_maxima(&lines);
    let mut grid = Grid::new(max_y + 1, max_x + 1);

    for line in lines {
        grid.draw_line(&line);
    }

    let overlaps = grid.get_overlaps();

    assert_eq!(overlaps, 12);
}
