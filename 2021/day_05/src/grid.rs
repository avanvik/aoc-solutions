use crate::line::{Line, Point};

pub struct Grid {
    pub cells: Vec<Vec<GridCell>>,
}

impl Grid {
    pub fn new(y_size: usize, x_size: usize) -> Self {
        let cells = vec![vec![GridCell { overlaps: 0 }; x_size]; y_size];

        Grid { cells }
    }

    pub fn draw_line(&mut self, line: &Line) {
        let rasterized_line = line.to_rasterized();
        for point in rasterized_line {
            let Point { x, y } = point;
            self.set_cell(y, x, self.get_cell(y, x).overlaps + 1);
        }
    }

    pub fn get_overlaps(&self) -> usize {
        let mut overlaps = 0;
        for row in &self.cells {
            for cell in row {
                if cell.overlaps > 1 {
                    overlaps += 1;
                }
            }
        }
        overlaps
    }

    fn get_cell(&self, y: usize, x: usize) -> GridCell {
        self.cells[y][x]
    }

    fn set_cell(&mut self, y: usize, x: usize, new_value: isize) {
        self.cells[y][x].overlaps = new_value;
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut final_string = "".to_string();
        final_string.push_str("––––––---------------\n");

        for col in &self.cells {
            for cell in col {
                final_string.push_str(
                    format!(
                        " {}",
                        if cell.overlaps > 0 {
                            format!("{}", cell.overlaps)
                        } else {
                            ".".to_string()
                        }
                    )
                    .as_str(),
                );
            }
            final_string.push_str("\n");
        }

        final_string.push_str("––––––---------------");

        write!(f, "{}", final_string)
    }
}

#[derive(Clone, Copy)]
pub struct GridCell {
    pub overlaps: isize,
}
