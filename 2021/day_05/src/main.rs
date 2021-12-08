mod line;
use grid::Grid;
use line::{Line, Point};
mod grid;

/*
    ADVENT of CODE - Day 5 "Hydrothermal Venture"

    Part 1:
        - Only consider horizontal and vertical lines
        - Determine the number of points where at least two lines overlap

    Part 2:
        - Do the same, but also for queer lines

*/

fn main() {
    // Get lines from file
    let lines = build_lines_from_file("real_input.txt");
    let non_slanted_lines: Vec<&Line> = lines
        .iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        .collect();

    // Generate grid based on line sizes
    let (max_x, max_y) = get_maxima(&lines);
    let mut grid = Grid::new(max_y + 1, max_x + 1);

    // Part 1
    for line in non_slanted_lines {
        grid.draw_line(&line);
    }
    let overlaps = grid.get_overlaps();
    println!("PART 1: Overlaps {}", overlaps);

    // Part 2
    let mut grid = Grid::new(max_y + 1, max_x + 1);
    for line in lines {
        grid.draw_line(&line);
    }

    let overlaps = grid.get_overlaps();
    println!("PART 2: Overlaps {}", overlaps);
}

fn build_lines_from_file(path: &str) -> Vec<Line> {
    let data_str = std::fs::read_to_string(path).expect("Could not read file content");

    let lines = data_str
        .lines()
        .map(|data_line| {
            let line_str: Vec<&str> = data_line.split(" -> ").collect();
            let start_coords: Vec<usize> =
                line_str[0].split(",").map(|s| s.parse().unwrap()).collect();
            let end_coords: Vec<usize> =
                line_str[1].split(",").map(|s| s.parse().unwrap()).collect();

            Line {
                start: Point {
                    x: start_coords[0],
                    y: start_coords[1],
                },
                end: Point {
                    x: end_coords[0],
                    y: end_coords[1],
                },
            }
        })
        .collect();

    lines
}

fn get_maxima(lines: &Vec<Line>) -> (usize, usize) {
    let mut max = (0, 0);

    for line in lines {
        let max_x = line.start.x.max(line.end.x);
        let max_y = line.start.y.max(line.end.y);

        if max_x >= max.0 {
            max.0 = max_x;
        }

        if max_y >= max.1 {
            max.1 = max_y;
        }
    }

    max
}

#[cfg(test)]
mod tests;
