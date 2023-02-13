use chess::board::*;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut line = String::new();

    let board = Board::new();
    
    // clear entire screen
    print!("\x1b[2J");
    
    loop {        
        // display board
        board.show(true);
        
        // wait for input
        println!("\nMove: ");
        stdin.read_line(&mut line)?;
        let action = line.trim();
        
        if action == "quit" {
            break;
        }
        
        // clear current line
        print!("\u{001b}[2J");
        line.clear();
    }

    Ok(())
}
