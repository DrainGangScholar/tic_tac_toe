use lib::board::Board;

mod lib;
fn main() {
    let mut board = Board::default();
    print!("{}", &mut board);
}
