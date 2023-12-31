use std::time::Instant;

use anyhow::Result;

const QUEEN: char = 'Q';
const EMPTY: char = '.';
const SIZE: usize = 4;

fn main() -> Result<()> {
    let size: usize = sorting::get_count("What size board?").unwrap_or(SIZE);
    let mut board = vec![vec![EMPTY; size]; size];

    let start = Instant::now();
    let success = place_queens_1(&mut board, 0, 0);
    let duration = start.elapsed();

    match success {
        true => {
            println!("Solution found in {:?}", duration);
            dump_board(&board);
        }
        false => println!("No solution in {:?}", duration),
    }

    Ok(())
}

fn place_queens_1(board: &mut Vec<Vec<char>>, row: usize, _col: usize) -> bool {
    // _col isn't necessary here unless we want to do extra work.
    let n = board.len();
    // If have processed as many rows as there are in the board, we are done.
    if row == n {
        return true;
    }

    // For each column.
    for col in 0..n {
        if cell_is_safe(board, row, col) {
            // Place a queen if it is safe.
            board[row][col] = QUEEN;
            // And recursively try the next row.
            if place_queens_1(board, row + 1, col) {
                // Woohoo success.
                return true;
            } else {
                // Backtrack.
                board[row][col] = EMPTY;
            }
        }
    }

    false
}

fn cell_is_safe(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // Check if there is a Queen in the column already.
    for (i, _) in board[row].iter().enumerate() {
        if board[i][col] == QUEEN {
            // whoops already a queen in this column, not safe.
            return false;
        }
    }

    // Check the diagonals
    // k rows
    for k in 0..board.len() {
        // l columns
        for l in 0..board.len() {
            // diag down and right || diag up and left
            if k + l == row + col || k - l == row - col {
                /*
                0, 1, 2, 3 ...,
                1, 2, 3, 4 ...,
                2, 3, 4, 5 ...,
                3, 4, 5, 6 ...,
                ..., ..., ...,
                */
                if board[k][l] == QUEEN {
                    // whoops already a queen in the diagonal, not safe.
                    return false;
                }
            }
        }
    }

    true
}

fn dump_board(board: &Vec<Vec<char>>) {
    println!();
    for row in board {
        for col in row.iter() {
            print!("{col} ");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
