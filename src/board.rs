use crate::coordinate::Coordinate;
use crate::pieces::{Id, MoveChecker, Piece};
use crate::Error;

pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = 8;

// \u{001b}[38;5;<n>m -> foreground colour for some n
// \u{001b}[48;5;<n>m -> background colour for some value of n
const TILE_COLOURS: [&str; 2] = ["\u{001b}[48;5;250m", "\u{001b}[48;5;240m"];
const WHITE_COLOUR: &str = "\u{001b}[38;5;255m";
const BLACK_COLOUR: &str = "\u{001b}[38;5;232m";
const WARNING_COLOUR: &str = "\u{001b}[31m";

/// Stores the pieces as in a 2D array
/// * `grid` - 2D array of options of [Piece]
/// * `message` - feedback printed on top of move prompt
#[derive(Clone)]
pub struct Board {
    pub grid: [[Option<Piece>; NUM_COLS]; NUM_ROWS],
    pub message: String,
}

impl Board {
    /// Fills board with `None`
    pub fn empty() -> Board {
        Board {
            grid: Default::default(),
            message: String::new(),
        }
    }

    /// Sets up board in starting position
    pub fn new() -> Board {
        let mut board = Board::empty();

        // both black and white pieces use the unicode white pieces
        // because the unicode black pawn is coloured by default in command prompt

        // white pieces
        let rank_1 = ['♖', '♘', '♗', '♔', '♕', '♗', '♘', '♖'];
        let rank_2 = ['♙'; 8];

        // black pieces
        let rank_7 = ['♙'; 8];
        let rank_8 = ['♖', '♘', '♗', '♔', '♕', '♗', '♘', '♖'];

        for x in 0..NUM_COLS {
            board.place_piece(x, 0, rank_1[x], true);
            board.place_piece(x, 1, rank_2[x], true);
            board.place_piece(x, 6, rank_7[x], false);
            board.place_piece(x, 7, rank_8[x], false);
        }

        return board;
    }

    /// Sets up board from a vector of piece data tuples
    /// * Each tuple contains (`x`, `y`, `icon`, `white`), corresponding to the arguments for `place_piece`
    pub fn from_vec(pieces: &Vec<(usize, usize, char, bool)>) -> Board {
        let mut board = Board::empty();
        for (x, y, icon, white) in pieces {
            board.place_piece(*x, *y, *icon, *white);
        }
        return board;
    }

    /// Sets a single piece at (x, y)
    pub fn place_piece(&mut self, x: usize, y: usize, icon: char, white: bool) {
        let piece = Piece::new(x, y, icon, white);
        match piece {
            Ok(piece) => self.grid[y][x] = Some(piece),
            Err(_) => eprintln!("Could not place {} at ({}, {})", icon, x, y),
        }
    }

    /// Prints out a specific tile, with A1 as (0, 0)
    fn show_tile(&self, x: usize, y: usize) {
        let tile = TILE_COLOURS[(x + y) % 2];

        // \u{fe0e} increases the size of the pieces in command prompt
        match &self.grid[y][x] {
            Some(piece) => {
                let colour = if piece.white {
                    WHITE_COLOUR
                } else {
                    BLACK_COLOUR
                };
                print!("{} {}{}\u{fe0e} ", tile, &colour, &piece.icon);
            }
            None => print!("{}  \u{fe0e} ", tile),
        }
    }

    /// Prints the chessboard to the console
    pub fn show(&self, white: bool) {
        // clear screen
        print!("\u{001b}[2J");
        print!("\u{001b}[d");

        for i in 0..NUM_ROWS {
            let y = if white { NUM_ROWS - i - 1 } else { i };

            // print row numbers
            print!("{} ", y + 1);
            for j in 0..NUM_COLS {
                let x = if white { j } else { NUM_COLS - j - 1 };
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
        println!("\n\n{}", self.message);

        // reset colours
        println!("\u{001b}[0m");
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
    pub fn parse_move(&self, input: &str, white: bool) -> Result<(&Piece, Coordinate), Error> {
        // TODO: google en passant
        // TODO: castling
        // TODO: promotion

        if input.len() < 2 {
            return Err(Error::InvalidArgument);
        }

        // remove letter x because it doesn't really matter
        let mut input = input.replace("x", "");

        // last 2 chars of move refers to the destination
        let position = Coordinate::from_alphanumeric(&input[(input.len() - 2)..])?;

        // default to pawn since it has no associated letter
        let mut id = Id::Pawn;

        // only uppercase letters for pieces
        // lowercase b could be confused for uppercase B
        // e.g. bxc5 vs Bxc5
        let mut piece_letter = 'P';
        for letter in ['B', 'N', 'K', 'Q', 'R'] {
            if input.starts_with(letter) {
                piece_letter = letter;
                id = Id::from_char(letter)?;
                break;
            }
        }

        // check for additional identifiers for disambiguation
        let ambiguous = 10;
        let mut x = ambiguous;
        let mut y = ambiguous;

        // pawn taking move
        // max length of any pawn move with the x removed is 3
        if input.len() == 3 && piece_letter == 'P' {
            // first letter identifies the column
            x = input.chars().nth(0).unwrap() as usize - 97;
        }

        // all other disambiguations
        if input.len() > 3 {
            // remove piece char and last 2 chars to get the identifiers
            input = input.replace(piece_letter, "");
            let coordinates: Vec<char> = input[..(input.len() - 2)].chars().collect();

            if coordinates.len() == 1 {
                // decide whether its the column or row identifier
                if coordinates[0].is_alphabetic() && coordinates[0].is_lowercase() {
                    x = coordinates[0] as usize - 97;
                } else {
                    y = coordinates[0] as usize - 49;
                }
            } else {
                x = coordinates[0] as usize - 97;
                y = coordinates[1] as usize - 49;
            }
        }

        // check if there is any ambiguity
        let mut possible_move: Option<(&Piece, Coordinate)> = None;

        // searching every square in an 8 x 8 grid isn't the most efficient way,
        // but given the small size it shouldn't be a significant cost to performance
        let checker = MoveChecker::from_id(&id);
        for row in &self.grid {
            for piece in row {
                match piece {
                    // check the piece's properties before checking the move
                    Some(piece) => {
                        if piece.id == id
                            && piece.white == white
                            && checker.can_move(&self, piece, &position)
                        {
                            // check for ambiguity
                            if (x != ambiguous && piece.position.x != x)
                                || (y != ambiguous && piece.position.y != y)
                            {
                                continue;
                            }

                            // if a move has already been found,
                            // then there shouldn't be another possibility
                            match possible_move {
                                Some(_) => return Err(Error::InvalidArgument),
                                None => possible_move = Some((piece, position)),
                            }
                        }
                    }
                    None => continue,
                }
            }
        }

        // check if a move has been found
        match possible_move {
            Some((piece, position)) => return Ok((piece, position)),
            None => return Err(Error::InvalidArgument),
        }
    }

    /// Move a piece based on `input`
    /// * Returns `true` if the move is valid, `false` if not
    pub fn make_move(&mut self, input: &str, white: bool) -> bool {
        self.message.clear();
        let (original, position) = match self.parse_move(input, white) {
            Ok((piece, position)) => (piece, position),
            Err(_) => {
                self.message = format!("{}{} is not a valid move", WARNING_COLOUR, input);
                return false;
            }
        };

        // check if the move will put the king in check with a test board
        let mut board = self.clone();
        board.grid[original.position.y][original.position.x] = None;
        board.place_piece(position.x, position.y, original.icon, original.white);
        if MoveChecker::in_check(&board, white) {
            self.message = format!("{}King would be in check", WARNING_COLOUR);
            return false;
        }

        // move piece
        let mut piece = original.clone();
        piece.position.x = position.x;
        piece.position.y = position.y;
        self.grid[original.position.y][original.position.x] = None;
        self.grid[position.y][position.x] = Some(piece);
        return true;
    }
}
