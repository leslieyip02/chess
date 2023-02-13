use super::super::pieces::piece::Piece;

const NUM_ROWS: usize = 8;
const NUM_COLS: usize = 8;
const WHITE: &str = "\u{001b}[100;1m";
const BLACK: &str = "\u{001b}[40;1m";

pub struct Board {
    grid: [[Option<Piece>; NUM_COLS]; NUM_ROWS],
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            grid: Default::default(),
        };

        board.reset();
        return board;
    }

    /**
     * Resets to starting position
     */
    pub fn reset(&mut self) {
        // white pieces
        self.grid[0] = ["♖", "♘", "♗", "♔", "♕", "♗", "♘", "♖"].map(|c| Some(Piece::new(c)));
        self.grid[1] = ["♙"; 8].map(|c| Some(Piece::new(c)));

        // black pieces
        self.grid[6] = ["♟"; 8].map(|c| Some(Piece::new(c)));
        self.grid[7] = ["♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜"].map(|c| Some(Piece::new(c)));
    }

    /**
     * Prints out a specific tile, with A1 as (0, 0)
     */
    fn show_tile(&self, x: usize, y: usize) {
        let tile = if (x + y) % 2 == 0 { WHITE } else { BLACK };
        let piece = match &self.grid[y][x] {
            Some(piece) => &piece.icon,
            None => " \u{fe0e}", // \u{fe0e} to match tiles with pieces
        };

        print!("{} {} ", tile, piece);
    }

    /**
     * Prints the chessboard to the console
     */
    pub fn show(&self, white: bool) {
        // clear screen
        print!("\x1b[d");

        for i in 0..NUM_ROWS {
            let y = if white { NUM_ROWS - i - 1 } else { i };

            // print row numbers
            print!("{} ", y + 1);

            for j in 0..NUM_COLS {
                let x = if white { NUM_COLS - j - 1 } else { j };
                self.show_tile(x, y);
            }

            // clear current background colour
            println!("\u{001b}[0m");
        }

        // print column letters
        print!("  ");
        for i in 0..NUM_COLS {
            let keycode = if white { i + 65 } else { NUM_COLS - i + 64 } as u32;
            match char::from_u32(keycode) {
                Some(c) => print!(" {} ", c),
                None => print!("   "),
            };
        }
    }
}
