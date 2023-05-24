use super::computer::Computer;
use super::player::Player;
pub struct Game {
    board: Vec<Vec<char>>,
    player: Player,
    computer: Computer,
}
impl Game {
    pub(crate) fn new(player: Player, computer: Computer) -> Self {
        Game {
            board: vec![
                vec![' ', ' ', ' '],
                vec![' ', ' ', ' '],
                vec![' ', ' ', ' '],
            ],
            player,
            computer,
        }
    }
    fn player_move(&mut self, row: usize, col: usize) {
        self.board[row][col] = self.player.symbol;
    }
    fn computer_move(&mut self) {
        let (_, row, col) = self.negamax(0, self.computer.symbol);
        self.board[row][col] = self.computer.symbol;
    }
    fn negamax(&mut self, depth: i32, player: char) -> (i32, usize, usize) {
        if self.win(self.computer.symbol) {
            return (1, 0, 0);
        }
        if self.win(self.player.symbol) {
            return (-1, 0, 0);
        }
        if self.draw() {
            return (0, 0, 0);
        }

        let mut best_score = std::i32::MIN;
        let mut best_move = (0, 0);

        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == ' ' {
                    let mut temp_board = self.board.clone();
                    temp_board[row][col] = player;

                    let (score, _, _) = self.negamax_helper(
                        temp_board,
                        depth + 1,
                        if player == self.player.symbol {
                            self.computer.symbol
                        } else {
                            self.player.symbol
                        },
                    );

                    let score = -score;

                    if score > best_score {
                        best_score = score;
                        best_move = (row, col);
                    }
                }
            }
        }
        (best_score, best_move.0, best_move.1)
    }
    fn negamax_helper(
        &self,
        board: Vec<Vec<char>>,
        depth: i32,
        player: char,
    ) -> (i32, usize, usize) {
        let mut temp_game = Game {
            board: board.clone(),
            player: self.player.clone(),
            computer: self.computer.clone(),
        };

        temp_game.negamax(depth, player)
    }
}
