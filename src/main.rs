use lib::{Computer, Game, Player};

mod lib;
fn main() {
    let player = Player::default();
    let computer = Computer::default();
    let mut Game = Game::new(player, computer);
}
