use std::{
    cmp::{max, min},
    fmt::Display,
};

use super::Cell;
#[derive()]
pub struct Board {
    cells: [Option<Cell>; 9],
}
impl Board {
    // fn make_move(&mut self, row: u8, col: u8) -> Result<T, E> {
    //     if row >= 3 || row <= 0 || col >= 3 || col <= 0 {
    //         return Err("Index out of bounds...".to_string());
    //     }
    // }
    // fn evaluate_state() {}
    // fn is_full() {}
    fn alpha_beta(
        &mut self,
        depth: u32,
        alpha: &mut i32,
        beta: &mut i32,
        maximizing_player: bool,
    ) -> (i32, Option<usize>) {
        if depth == 0 || self.game_over() {
            return (self.score(), None);
        }
        if maximizing_player {
            let mut max_eval = std::i32::MIN;
            let mut best_move = None;
            for m in self.possible_moves() {
                self.cells[m] = Some(Cell::O); //do move
                let (eval, _) = self.alpha_beta(depth - 1, alpha, beta, false);
                self.cells[m] = None; //undo move
                if eval > max_eval {
                    max_eval = eval;
                    best_move = Some(m);
                }
                *alpha = max(*alpha, eval);
                if *alpha >= *beta {
                    break;
                }
            }
            (max_eval, best_move)
        } else {
            let mut min_eval = std::i32::MIN;
            let mut best_move = None;
            for m in self.possible_moves() {
                self.cells[m] = Some(Cell::O); //do move
                let (eval, _) = self.alpha_beta(depth - 1, alpha, beta, false);
                self.cells[m] = None; //undo move
                if eval < min_eval {
                    min_eval = eval;
                    best_move = Some(m);
                }
                *alpha = min(*alpha, eval);
                if *alpha >= *beta {
                    break;
                }
            }
            (min_eval, best_move)
        }
    }
    fn possible_moves(&mut self) -> Vec<usize> {
        let mut moves = Vec::new();
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_none() {
                moves.push(i);
            }
        }
        moves
    }
    fn score(&mut self) -> i32 {
        -5
    }
    fn game_over(&mut self) -> bool {
        true
    }
}
impl Default for Board {
    fn default() -> Self {
        Self { cells: [None; 9] }
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            if i % 3 != 0 {
                write!(f, " | ")?;
            }
            match cell {
                Some(Cell::X) => write!(f, "X")?,
                Some(Cell::O) => write!(f, "O")?,
                None => write!(f, " ")?,
            }
            if i % 3 == 2 && i != self.cells.len() - 1 {
                writeln!(f)?;
                writeln!(f, "- - - - -")?;
            }
        }
        Ok(())
    }
}
