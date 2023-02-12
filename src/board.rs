const NUM_ROWS: usize = 8;
const NUM_COLS: usize = 8;
const WHITE: &str = "\u{001b}[107m";
const BLACK: &str = "\u{001b}[40m";

pub struct Board {
    grid: [[char; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[' '; NUM_COLS]; NUM_ROWS],
        }
    }

    /**
     * Prints out a specific tile, with A1 as (0, 0)
     */
    fn show_tile(&self, x: usize, y: usize) {
        let tile = if (x + y) % 2 == 0 { WHITE } else { BLACK };
        print!("{} {} ", tile, self.grid[y][x]);
    }

    /**
     * Prints the chessboard to the console
     */
    pub fn show(&self, white: bool) {
        // clear screen
        print!("\x1b[d");

        for i in 0..NUM_ROWS {
            let y = if white { NUM_ROWS - i - 1 } else { i };

            // print row number
            print!("{} ", y + 1);

            for j in 0..NUM_COLS {
                let x = if white { NUM_COLS - j - 1 } else { j };
                self.show_tile(x, y);
            }

            // clear current background colour
            println!("\u{001b}[0m");
        }

        // print column letter
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
