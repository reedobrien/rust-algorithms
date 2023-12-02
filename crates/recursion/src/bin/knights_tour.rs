use std::{fmt, process, time::Instant};

use anyhow::Result;

const MOVES: [(isize, isize); 8] = [
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
];

const SIZE: usize = 8;
const UNVISITED: isize = -1;

struct Board {
    size: usize,
    cells: Vec<Vec<isize>>,
}

struct CandidateMove {
    cell: Cell,
    paths: usize,
}

impl Board {
    fn new(size: Option<usize>) -> Self {
        Self {
            size: size.unwrap_or(SIZE),
            cells: vec![vec![UNVISITED; size.unwrap_or(SIZE)]; size.unwrap_or(SIZE)],
        }
    }

    fn cell_count(&self) -> usize {
        self.size * self.size
    }

    fn moves(&self, pos: Cell) -> Vec<Cell> {
        MOVES
            .iter()
            .filter(|m| self.valid_move(&pos, m))
            .map(|m| Cell {
                x: pos.x + m.0,
                y: pos.y + m.1,
            })
            .collect()
    }

    // Get the next move using Warnsdorf's rule.
    fn next_move(&self, pos: Cell) -> Option<Cell> {
        let mut candidates: Vec<CandidateMove> = vec![];

        let moves = self.moves(pos);
        if moves.len() < 1 {
            return None;
        }

        for m in moves {
            candidates.push(CandidateMove {
                cell: m,
                paths: self.moves(m).len(),
            });
        }

        candidates.sort_unstable_by_key(|c| c.paths);

        Some(candidates[0].cell)
    }

    fn size(&self) -> usize {
        self.size
    }

    fn max_index(&self) -> isize {
        (self.size - 1) as isize
    }

    fn valid_move(&self, pos: &Cell, r#move: &(isize, isize)) -> bool {
        if pos.x + r#move.0 > self.max_index() || pos.x + r#move.0 < 0 {
            return false;
        }
        if pos.y + r#move.1 > self.max_index() || pos.y + r#move.1 < 0 {
            return false;
        }

        if self.visited(&Cell {
            x: pos.x + r#move.0,
            y: pos.y + r#move.1,
        }) {
            return false;
        }

        true
    }

    fn visit(&mut self, cell: &Cell, val: isize) {
        self.cells[cell.x as usize][cell.y as usize] = val;
    }

    fn visited(&self, cell: &Cell) -> bool {
        self.cells[cell.x as usize][cell.y as usize] != UNVISITED
    }

    fn dump(&self) {
        println!("{}", &self);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for col in row.iter() {
                write!(f, " {:3}", col)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    x: isize,
    y: isize,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cell{{x: {}, y: {}}}", self.x, self.y)
    }
}

fn find_tour(board: &mut Board, cell: Cell, count: isize) -> bool {
    loop {
        board.visit(&cell, count);

        match board.next_move(cell) {
            Some(next_cell) => {
                return find_tour(board, next_cell, count + 1);
            }
            None => {
                if count == board.cell_count() as isize - 1 {
                    return true;
                }
                return false;
            }
        }
    }
}

fn find_tour_backtracking(board: &mut Board, cell: Cell, count: isize) -> bool {
    if count as usize == board.size() * board.size() {
        return true;
    }

    for i in 0..MOVES.len() {
        let m = MOVES[i];
        if board.valid_move(&cell, &(m.0, m.1)) {
            let c = Cell {
                x: &cell.x + m.0,
                y: cell.y + m.1,
            };
            board.visit(&c, count);

            if find_tour_backtracking(board, c, count + 1) {
                return true;
            }

            board.visit(&c, UNVISITED);
        }
    }
    false
}

fn main() -> Result<()> {
    let size: usize = sorting::get_count("What size board?")?;
    let mut board = Board::new(Some(size));
    let start_cell = Cell { x: 0, y: 0 };

    println!("Warnsdorff non-backtracking version");
    let start = Instant::now();
    let success = find_tour(&mut board, start_cell, 0);
    let duration = start.elapsed();

    match success {
        true => {
            println!("Success! in {:?}", duration);
        }
        false => println!("Could not find a tour in {:?}...", duration),
    }

    board.dump();

    println!("recursive backtracking version");
    if size > 8 {
        println!("Skipping board size greater than 8. It takes too long.");
        process::exit(0);
    }
    let mut board = Board::new(Some(size));
    let start = Instant::now();
    board.visit(&start_cell, 0);
    let success = find_tour_backtracking(&mut board, start_cell, 1);
    let duration = start.elapsed();

    match success {
        true => {
            println!("Success! in {:?}", duration);
        }
        false => println!("Could not find a tour in {:?}...", duration),
    }

    board.dump();

    Ok(())
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_board_new() {
        let tut = Board::new(None);
        for row in &tut.cells {
            for col in row.iter() {
                assert_eq!(*col, UNVISITED);
            }
        }
    }

    #[test]
    fn test_board_visit() {
        let mut tut = Board::new(None);
        assert_eq!(tut.visited(&Cell { x: 0, y: 0 }), false);
        tut.visit(&Cell { x: 0, y: 0 }, 1);
        assert_eq!(tut.visited(&Cell { x: 0, y: 0 }), true);
    }

    #[test]
    fn test_moves_from_corner() {
        let tut = Board::new(None);
        let moves = tut.moves(Cell { x: 0, y: 0 });
        assert_eq!(moves.len(), 2);

        let tut = Board::new(None);
        let moves = tut.moves(Cell { x: 7, y: 7 });
        assert_eq!(moves.len(), 2);

        let tut = Board::new(None);
        let moves = tut.moves(Cell { x: 7, y: 0 });
        assert_eq!(moves.len(), 2);

        let tut = Board::new(None);
        let moves = tut.moves(Cell { x: 0, y: 7 });
        assert_eq!(moves.len(), 2);

        let mut tut = Board::new(None);
        tut.visit(&Cell { x: 2, y: 1 }, 1);
        let moves = tut.moves(Cell { x: 0, y: 0 });
        assert_eq!(moves.len(), 1);

        let mut tut = Board::new(None);
        tut.visit(&Cell { x: 5, y: 6 }, 1);
        let moves = tut.moves(Cell { x: 7, y: 7 });
        assert_eq!(moves.len(), 1);

        let mut tut = Board::new(None);
        tut.visit(&Cell { x: 5, y: 6 }, 1);
        tut.visit(&Cell { x: 6, y: 5 }, 2);
        let moves = tut.moves(Cell { x: 7, y: 7 });
        assert!(moves.is_empty());
    }

    #[test]
    fn test_moves_from_middle() {
        let tut = Board::new(None);
        let moves = tut.moves(Cell { x: 4, y: 4 });
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_valid_move_from_corner() {
        let mut tut = Board::new(None);
        let start_cell = &Cell { x: 0, y: 0 };
        tut.visit(start_cell, 0);
        assert_eq!(tut.valid_move(start_cell, &(-1, -2)), false);
        assert_eq!(tut.valid_move(start_cell, &(-1, 2)), false);
        assert_eq!(tut.valid_move(start_cell, &(-2, 1)), false);
        assert_eq!(tut.valid_move(start_cell, &(1, -2)), false);
        assert_eq!(tut.valid_move(start_cell, &(1, 2)), true);
        assert_eq!(tut.valid_move(start_cell, &(2, -1)), false);
        assert_eq!(tut.valid_move(start_cell, &(2, -1)), false);
        assert_eq!(tut.valid_move(start_cell, &(2, 1)), true);
    }

    #[test]
    fn test_valid_move_from_middle() {
        let mut tut = Board::new(None);
        let start_cell = &Cell { x: 3, y: 3 };
        tut.visit(start_cell, 0);
        assert_eq!(tut.valid_move(start_cell, &(-1, -2)), true);
        assert_eq!(tut.valid_move(start_cell, &(-1, 2)), true);
        assert_eq!(tut.valid_move(start_cell, &(-2, 1)), true);
        assert_eq!(tut.valid_move(start_cell, &(1, -2)), true);
        assert_eq!(tut.valid_move(start_cell, &(1, 2)), true);
        assert_eq!(tut.valid_move(start_cell, &(2, -1)), true);
        assert_eq!(tut.valid_move(start_cell, &(2, -1)), true);
        assert_eq!(tut.valid_move(start_cell, &(2, 1)), true);
    }
}
