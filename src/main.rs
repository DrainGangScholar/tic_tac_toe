use std::fmt::Display;

const ROWS: usize = 3;
const COLS: usize = 3;
const X_WIN: [char; 3] = ['X', 'X', 'X'];
const O_WIN: [char; 3] = ['O', 'O', 'O'];
fn main() {
    let mut board = Board::empty();
    match board.make_move(1, 2, 'X') {
        Ok(_) => println!("{}", &board),
        Err(_) => println!("MRs"),
    };
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;
    board.alpha_beta(9, &mut alpha, &mut beta, false);
    match board.make_move(alpha as usize, beta as usize, '0') {
        Ok(_) => println!("{}", &board),
        Err(_) => println!("MRs"),
    };
}

struct Board {
    board: [[char; COLS]; ROWS],
}
impl Board {
    fn empty() -> Self {
        Board {
            board: [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']],
        }
    }
    fn make_move(&mut self, row: usize, col: usize, choice: char) -> Result<(), String> {
        //malo vise razmisli o Resutl<>
        if row >= ROWS || col >= COLS {
            return Err("Index out of bounds".to_string());
        }
        if self.board[row][col] != ' ' {
            return Err("Occupied!".to_string());
        }
        self.board[row][col] = choice;
        Ok(())
    }
    fn alpha_beta(
        &mut self,
        depth: i32,
        alpha: &mut i32,
        beta: &mut i32,
        is_maximizing_player: bool,
    ) -> Result<(), i32> {
        if depth == 0 || self.evaluate() != 0 {
            return self.evaluate();
        }
        if is_maximizing_player {
            let mut value = std::i32::MIN;
            for (row, col) in self.generate_moves() {
                self.make_move(row, col, 'X').unwrap();
                value = value.min(self.alpha_beta(depth - 1, alpha, beta, false));
                self.board[row][col] = ' '; //undo
                *alpha = *(alpha).max(&mut value);
                if alpha >= beta {
                    break;
                }
            }
            value
        } else {
            let mut value = std::i32::MAX;
            for (row, col) in self.generate_moves() {
                self.make_move(row, col, 'O').unwrap();
                value = value.max(self.alpha_beta(depth - 1, alpha, beta, true));
                self.board[row][col] = ' '; //undo
                *beta = *(beta).min(&mut value);
                if alpha >= beta {
                    break;
                }
            }
            value
        }
    }
    fn generate_moves(&mut self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                if self.board[i][j] == ' ' {
                    moves.push((i, j));
                }
            }
        }
        moves
    }
    fn evaluate(&mut self) -> i32 {
        // 1 x pobednik, -1 o pobednik, 0 nereseno/nema pobednika jos?? treba i o tome da se razmislja...
        for i in 0..ROWS {
            match self.board[i] {
                X_WIN => {
                    //x pobedio
                    return 1;
                }
                O_WIN => {
                    //y pobedio
                    return -1;
                }
                _ => {
                    //proveram kolonu
                    let column = [self.board[0][i], self.board[1][i], self.board[2][i]];
                    match column {
                        X_WIN => {
                            //x pobedio
                            return 1;
                        }
                        O_WIN => {
                            //y pobedio
                            return -1;
                        }
                        _ => (), //nema pobednika za sad :DD:
                    };
                }
            };
            let diagonal_left = [self.board[0][0], self.board[1][1], self.board[2][2]];
            let diagonal_right = [self.board[0][2], self.board[1][1], self.board[2][0]];
            if diagonal_left == X_WIN || diagonal_right == X_WIN {
                return 1;
            } else if diagonal_left == O_WIN || diagonal_right == O_WIN {
                return -1;
            }
        }
        0
    }
    fn is_board_full(&mut self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell != ' '))
    }
    fn is_game_over(&mut self) -> bool {
        self.evaluate() != 0 || self.is_board_full()
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+---+---+---+")?;
        for row in &self.board {
            write!(f, "|")?;
            for cell in row {
                write!(f, " {} |", cell)?;
            }
            writeln!(f)?;
            writeln!(f, "+---+---+---+")?;
        }
        Ok(())
    }
}
