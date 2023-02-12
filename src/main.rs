mod board;
use board::Board;

fn main() {
    // clear entire screen
    print!("\x1b[2J");

    let x = Board::new();
    x.show(true);
}
