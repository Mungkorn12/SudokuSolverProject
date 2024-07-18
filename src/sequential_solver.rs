use std::fmt;

pub struct SudokuSolver {
    board: [[i32; 9]; 9],
}

impl SudokuSolver {
    pub fn new(board_str: &str) -> Self {
        let mut board = [[0; 9]; 9];
        for (i, c) in board_str.chars().enumerate() {
            if c != '.' {
                board[i / 9][i % 9] = c.to_digit(10).unwrap() as i32;
            }
        }
        SudokuSolver { board }
    }

    pub fn print_board(&self) {
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                println!("-----------");
            }
            for (j, &cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }
                print!("{} ", if cell == 0 { '.' } else { std::char::from_digit(cell as u32, 10).unwrap() });
            }
            println!();
        }
    }

    fn is_safe(&self, row: usize, col: usize, num: i32) -> bool {
        for i in 0..9 {
            if self.board[row][i] == num || self.board[i][col] == num {
                return false;
            }
        }

        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        let mut empty_found = false;
        let mut row = 0;
        let mut col = 0;

        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    row = i;
                    col = j;
                    empty_found = true;
                    break;
                }
            }
            if empty_found {
                break;
            }
        }

        if !empty_found {
            return true;
        }

        for num in 1..=9 {
            if self.is_safe(row, col, num) {
                self.board[row][col] = num;
                if self.solve() {
                    return true;
                }
                self.board[row][col] = 0;
            }
        }

        false
    }
}

impl fmt::Display for SudokuSolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                writeln!(f, "------+-------+------")?;
            }
            for (j, &cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", if cell == 0 { '.' } else { std::char::from_digit(cell as u32, 10).unwrap() })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let board_str = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    let mut solver = SudokuSolver::new(board_str);

    println!("Initial board:");
    solver.print_board();

    if solver.solve() {
        println!("Solution:\n{}", solver);
    } else {
        println!("No solution exists");
    }
    let elapsed = now.elapsed();
    println!("Elapsed 2 : {:.2?}", elapsed);
}
