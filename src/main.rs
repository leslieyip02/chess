use chess::board::*;

fn main() {
    // clear entire screen
    print!("\x1b[2J");

    let x = Board::new();
    x.show(true);
}
