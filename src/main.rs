use std::fmt::Display;

const ROWS: usize = 3;
const COLS: usize = 3;
const X_WIN: [char; 3] = ['X', 'X', 'X'];
const O_WIN: [char; 3] = ['O', 'O', 'O'];
fn main() {
    // let mut board = Board::empty();
    // println!("{}", &board);
    // loop {
    //     match board.make_move(1, 2, 'X') {
    //         Ok(_) => println!("{}", &board),
    //         Err(_) => continue,
    //     }
    // }
    let mut board = Board::default();
    loop {
        //treba da bude neka alternacija izmedju algoritma i pravog igraca D::D
        match board.make_move(7, 4, 'X') {
            Ok(_) => println!("{}", &board),
            Err(err) => println!("{}", err)(),
        };
    }
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
    fn _evaluate(&mut self) -> i32 {
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
}
impl Default for Board {
    fn default() -> Self {
        Board {
            board: [['X', 'O', 'X'], ['O', 'X', 'O'], ['O', 'X', 'O']],
        }
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
