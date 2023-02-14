use crate::pieces::{Id, MoveChecker, Piece};
use crate::{Coordinate, Error, NUM_COLS, NUM_ROWS};

// \u{001b}[48;5;<n>m -> background colour for some value of n
const TILE_COLOURS: [&str; 2] = ["\u{001b}[48;5;229m", "\u{001b}[48;5;106m"];

/// Stores the pieces as options in a 2D array
/// * pieces can be dereferenced by calling `as_ref()`
/// * pieces can also be referenced inside `match` blocks
pub struct Board {
    pub grid: [[Option<Piece>; NUM_COLS]; NUM_ROWS],
}

impl Board {
    /// Fills board with `None`
    pub fn empty() -> Board {
        Board {
            grid: Default::default(),
        }
    }

    /// Sets up board in starting position
    pub fn new() -> Board {
        let mut board = Board::empty();

        board.reset();
        return board;
    }

    /// Sets a single piece at (x, y)
    pub fn place_piece(&mut self, x: usize, y: usize, icon: &str, white: bool) {
        let piece = Piece::new(x, y, icon, white);
        match piece {
            Ok(piece) => self.grid[y][x] = Some(piece),
            Err(_) => println!("Could not place {} at ({}, {})", icon, x, y),
        }
    }

    /// Resets to starting position
    pub fn reset(&mut self) {
        // white pieces
        let rank_1 = ["♖", "♘", "♗", "♔", "♕", "♗", "♘", "♖"];
        let rank_2 = ["♙"; 8];

        // black pieces
        let rank_7 = ["♙"; 8];
        let rank_8 = ["♖", "♘", "♗", "♔", "♕", "♗", "♘", "♖"];

        for x in 0..NUM_COLS {
            self.place_piece(x, 0, rank_1[x], true);
            self.place_piece(x, 1, rank_2[x], true);
            self.place_piece(x, 6, rank_7[x], false);
            self.place_piece(x, 7, rank_8[x], false);
        }
    }

    /// Prints out a specific tile, with A1 as (0, 0)
    fn show_tile(&self, x: usize, y: usize) {
        let tile = TILE_COLOURS[(x + y) % 2];
        let piece = match &self.grid[y][x] {
            Some(piece) => &piece.icon,
            None => " ",
        };

        // \u{fe0e} increases the size of the pieces in command prompt
        print!("{} {}\u{fe0e} ", tile, piece);
    }

    /// Prints the chessboard to the console
    pub fn show(&self, white: bool) {
        // clear screen
        print!("\u{001b}[d");

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
                Some(c) => print!(" {}\u{fe0e} ", c),
                None => print!("   \u{fe0e}"),
            };
        }
        println!();
    }

    /// Parses a move given in algebraic notation
    ///
    /// * Each piece is denoted by an uppercase letter, except for pawns
    ///     - B for bishop
    ///     - K for king
    ///     - N for knight
    ///     - Q for queen
    ///     - R for rook
    ///
    /// * Returns a `Piece` and a `Coordinate` to move to
    pub fn parse_move(&self, action: &str, white: bool) -> Result<(&Piece, Coordinate), Error> {
        // TODO: google en passant
        // TODO: castling
        // TODO: promotion

        // remove letter x because it doesn't really matter
        let mut action = action.replace("x", "");

        // last 2 chars of move refers to the destination
        let position = Coordinate::from_alphanumeric(&action[(action.len() - 2)..])?;

        // default to pawn since it has no associated letter
        let mut id = Id::Pawn;

        // only uppercase letters for pieces
        // lowercase b could be confused for uppercase B
        // e.g. bxc5 vs Bxc5
        let mut piece_letter = "";
        for letter in ["B", "N", "K", "Q", "R"] {
            if action.starts_with(letter) {
                piece_letter = letter;
                id = Id::from_str(letter)?;
                break;
            }
        }

        // check for additional identifiers for disambiguation
        let ambiguous = 10;
        let mut x = ambiguous;
        let mut y = ambiguous;

        // pawn taking move
        // max length of any pawn move with the x removed is 3
        if action.len() == 3 && piece_letter.is_empty() {
            // first letter identifies the column
            x = action.chars().nth(0).unwrap() as usize - 97;
        }

        if action.len() > 3 {
            // remove piece char and last 2 chars to get the identifiers
            action = action.replace(piece_letter, "");
            let coordinates: Vec<char> = action[..(action.len() - 2)].chars().collect();

            if coordinates.len() == 1 {
                // decide whether its the column or row identifier
                if coordinates[0].is_alphabetic() {
                    x = coordinates[0] as usize - 97;
                } else {
                    y = coordinates[0] as usize - 49;
                }
            } else {
                x = coordinates[0] as usize - 97;
                y = coordinates[1] as usize - 49;
            }
        }

        let checker = MoveChecker::from_id(&id);
        for row in &self.grid {
            for piece in row {
                match piece {
                    // check the piece's properties before checking the move
                    Some(piece) => {
                        if piece.id == id
                            && piece.white == white
                            && checker.can_move(piece, &position, &self)
                        {
                            // check for ambiguity
                            if x != ambiguous && piece.position.x != x {
                                continue;
                            }

                            if y != ambiguous && piece.position.y != y {
                                continue;
                            }

                            return Ok((piece, position));
                        }
                    }
                    None => continue,
                }
            }
        }

        Err(Error::InvalidArgument)
    }
}
