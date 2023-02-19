use chess::board::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut line = String::new();

    // keep track of turns
    let mut white = true;
    let mut board = Board::new();

    loop {
        // display board
        board.show(white);

        // wait for input
        println!(
            "Move (\u{001b}[4m{}\u{001b}[24m): ",
            if white { "white" } else { "black" }
        );
        stdin.read_line(&mut line)?;
        let action = line.trim();

        if action == "quit" {
            break;
        }

        if board.make_move(action, white) {
            board.show(white);
            board.show_loading_bar();

            if board.game_over(white) {
                break;
            }

            // only change to the other player after a valid move is made
            white = !white;
        }

        line.clear();
    }

    Ok(())
}
