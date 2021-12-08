use core::fmt;

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec<Vec<BoardCell>>,
    row_marks: Vec<usize>,
    col_marks: Vec<usize>,
}

impl Board {
    pub fn new(cells: Vec<Vec<BoardCell>>) -> Self {
        Board {
            cells: cells.clone(),
            row_marks: vec![0; cells.len()],
            col_marks: vec![0; cells.get(0).unwrap().len()],
        }
    }

    pub fn get_marked(&self) -> (usize, usize) {
        (
            self.row_marks
                .clone()
                .into_iter()
                .fold(0, |acc, m| acc.max(m)),
            self.col_marks
                .clone()
                .into_iter()
                .fold(0, |acc, m| acc.max(m)),
        )
    }

    // Returns (marked rows, marked cols) result
    pub fn mark_cell_with_number(&mut self, num: usize) {
        let board = &mut self.cells;
        board.into_iter().enumerate().for_each(|(i, row)| {
            row.into_iter().enumerate().for_each(|(j, mut cell)| {
                if cell.value == num {
                    cell.marked = true;
                    self.row_marks[i] += 1;
                    self.col_marks[j] += 1;
                }
            })
        });
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&BoardCell> {
        if let Some(row_arr) = self.cells.get(row) {
            row_arr.get(col)
        } else {
            None
        }
    }

    pub fn get_board_score(&self, last_number: usize) -> usize {
        let sum_of_unmarked = self.cells.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .filter(|c| !c.marked)
                .fold(0, |row_acc, cell| row_acc + cell.value)
        });

        let product = sum_of_unmarked * last_number;

        product
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoardCell {
    pub marked: bool,
    pub value: usize,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = &self.cells;
        // let mut sum_str = "".to_string();

        let sum_str = arr.into_iter().fold("".to_string(), |acc, row| {
            format!("{acc}{}\n", {
                row.iter().fold("".to_string(), |row_acc, cell| {
                    format!("{}", {
                        if cell.marked {
                            format!(
                                "{row_acc}  ({})",
                                if cell.value < 10 {
                                    format!(" {}", cell.value)
                                } else {
                                    format!("{}", cell.value)
                                }
                            )
                        } else {
                            format!(
                                "{row_acc}   {} ",
                                if cell.value < 10 {
                                    format!(" {}", cell.value)
                                } else {
                                    format!("{}", cell.value)
                                }
                            )
                        }
                    })
                })
            })
        });

        write!(f, "\n{}\n", sum_str)
    }
}
