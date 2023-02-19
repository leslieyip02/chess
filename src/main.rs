use chess::board::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut line = String::new();

    // select game type
    println!("\u{001b}[5mGame Select: \u{001b}[0m");
    println!("1. Chess");
    println!("2. Chess960");
    stdin.read_line(&mut line)?;
    let choice = line.trim();
    let mut board = match choice {
        "quit" => return Ok(()),
        "2" => Board::new_random(),
        _ => Board::new(),
    };
    line.clear();

    // keep track of turns
    let mut white = true;

    loop {
        // display board
        board.show(white);

        // wait for input
        println!(
            "Move (\u{001b}[4m{}\u{001b}[24m): ",
            if white { "white" } else { "black" }
        );
        stdin.read_line(&mut line)?;
        let input = line.trim();

        if input == "quit" {
            break;
        }

        if board.make_move(input, white) {
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
