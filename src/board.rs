use crate::coordinate::Coordinate;
use crate::pieces::{Id, MoveChecker, Piece};
use crate::Error;

pub const NUM_COLS: usize = 8;
pub const NUM_ROWS: usize = 8;
pub const KINGSIDE_CASTLE: [usize; 2] = [6, 5];
pub const QUEENSIDE_CASTLE: [usize; 2] = [2, 3];

// \u{001b}[38;5;<n>m -> foreground colour for some n
// \u{001b}[48;5;<n>m -> background colour for some value of n
const TILE_COLOURS: [&str; 2] = ["\u{001b}[48;5;250m", "\u{001b}[48;5;240m"];
const WHITE_COLOUR: &str = "\u{001b}[38;5;255m";
const BLACK_COLOUR: &str = "\u{001b}[38;5;232m";
const WARNING_COLOUR: &str = "\u{001b}[31m";

// inaccessible coordinate used to test for ambiguity
const AMBIGUOUS: usize = NUM_COLS + 10;

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
        let mut board = Self::empty();

        // both black and white pieces use the unicode white pieces
        // because the unicode black pawn is coloured by default in command prompt

        // white pieces
        let rank_1 = ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'];
        let rank_2 = ['♙'; 8];

        // black pieces
        let rank_7 = ['♙'; 8];
        let rank_8 = ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'];

        for x in 0..NUM_COLS {
            board.place_piece(x, 0, rank_1[x], true, 0);
            board.place_piece(x, 1, rank_2[x], true, 0);
            board.place_piece(x, 6, rank_7[x], false, 0);
            board.place_piece(x, 7, rank_8[x], false, 0);
        }

        return board;
    }

    /// Sets up board from a vector of piece data tuples
    /// * Each tuple contains (`x`, `y`, `icon`, `white`), corresponding to the arguments for `place_piece`
    pub fn from_vec(pieces: &Vec<(usize, usize, char, bool)>) -> Board {
        let mut board = Board::empty();
        for (x, y, icon, white) in pieces {
            board.place_piece(*x, *y, *icon, *white, 0);
        }
        return board;
    }

    /// Sets a single piece at (x, y)
    pub fn place_piece(&mut self, x: usize, y: usize, icon: char, white: bool, moves: usize) {
        let piece = Piece::new(x, y, icon, white, moves);
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
                let colour = match piece.white {
                    true => WHITE_COLOUR,
                    false => BLACK_COLOUR,
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

    /// Standardises the input string
    fn sanitise_input(input: &str) -> String {
        // these characters don't convery any additional information
        // x, : for captures (e.g. Bxe5, B:e5 or Be5:)
        // =, (), / for promotion (e.g. e8=Q, e8(Q), e8/Q)
        // + for checks, # for checkmates
        let input = input.replace(&['x', ':', '=', '(', ')', '/', '+', '#'][..], "");

        // remove optional en passant notation
        let input = input.replace("e.p.", "");

        // change '0's to 'O's for castling
        let input = input.replace('0', "O");

        // remove whitespace
        let input = String::from(input.trim());

        return input;
    }

    /// Checks what the move promotes to
    /// * returns `None` if it's not a promotion move
    fn promote_to(input: &str) -> Option<char> {
        for (letter, icon) in [('B', '♗'), ('N', '♘'), ('Q', '♕'), ('R', '♖')] {
            if input.ends_with(letter) {
                return Some(icon);
            }
        }

        return None;
    }

    /// Identifies the type of piece being moved
    fn piece_id(input: &str) -> Result<Id, Error> {
        // only uppercase letters for pieces
        // lowercase b could be confused for uppercase B
        // e.g. bxc5 vs Bxc5
        for letter in ['B', 'N', 'K', 'Q', 'R'] {
            if input.starts_with(letter) {
                let id = Id::from_char(letter)?;
                return Ok(id);
            }
        }

        // default to pawn since it has no associated letter
        Ok(Id::Pawn)
    }

    /// Converts a coordinate from alphanumeric grid to 0-indexed coordinates
    fn target_position(input: &str) -> Result<Coordinate, Error> {
        if input.len() < 2 {
            return Err(Error::IndexOutOfRange);
        }

        // last 2 chars of move refers to the destination
        let index = input.len() - 2;
        let target = Coordinate::from_alphanumeric(&input[index..])?;
        Ok(target)
    }

    // Returns the piece to move, the position to move to, and if the move is a promotion
    fn process_input(input: &str, white: bool) -> Result<(Id, Coordinate, Option<char>), Error> {
        let promotion = Self::promote_to(&input);
        let id = Self::piece_id(&input)?;

        // if the move is a promotion,
        // remove the last letter so that the target position is the last 2 characters
        let mut end_index = input.len();
        if promotion.is_some() {
            end_index -= 1;
        }
        let position = Self::target_position(&input[..end_index])?;

        // only allow promotion if its a pawn move to the correct rank
        let promotion_rank = if white { 7 } else { 0 };
        if promotion.is_some() && (id != Id::Pawn || position.y != promotion_rank) {
            return Err(Error::InvalidMove {
                message: String::from("is not a valid promotion"),
            });
        }

        // must promote when on promotion rank
        if promotion.is_none() && id == Id::Pawn && position.y == promotion_rank {
            return Err(Error::InvalidMove {
                message: String::from("is not valid because promotion is forced"),
            });
        }

        Ok((id, position, promotion))
    }

    /// Checks for additional positional identifiers for disambiguation
    fn disambiguate(input: &str, id: &Id) -> Result<(usize, usize), Error> {
        let mut x = AMBIGUOUS;
        let mut y = AMBIGUOUS;

        // pawn taking move
        // ignore pawn moves that are of length 2
        if id == &Id::Pawn && input.len() > 2 {
            // first letter identifies the column
            x = input.chars().nth(0).unwrap() as usize - 97;
        }

        // all other disambiguations for other pieces
        if id != &Id::Pawn && input.len() > 3 {
            // skip the piece letter to get the identifiers
            let coordinates: Vec<char> = input[1..(input.len() - 2)].chars().collect();

            if coordinates.len() == 1 {
                // decide whether its the column or row identifier
                if coordinates[0].is_alphabetic() && coordinates[0].is_lowercase() {
                    x = coordinates[0] as usize - 97;
                } else {
                    y = coordinates[0] as usize - 49;
                }
            } else if coordinates.len() == 2 {
                x = coordinates[0] as usize - 97;
                y = coordinates[1] as usize - 49;
            } else {
                // there shouldn't be more than 2 identifiers
                return Err(Error::InvalidArgument);
            }
        }

        Ok((x, y))
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
    pub fn parse_move(
        &self,
        input: &str,
        white: bool,
    ) -> Result<(&Piece, Coordinate, Option<char>), Error> {
        // TODO: google en passant
        // TODO: castling
        if input.len() < 2 {
            return Err(Error::InvalidArgument);
        }

        let (id, position, promotion) = Self::process_input(&input, white)?;
        let (x, y) = Self::disambiguate(&input, &id)?;

        // check if there is any remaining ambiguity
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
                            if (x != AMBIGUOUS && piece.position.x != x)
                                || (y != AMBIGUOUS && piece.position.y != y)
                            {
                                continue;
                            }

                            // if a move has already been found,
                            // then there shouldn't be another possibility
                            match possible_move {
                                Some(_) => {
                                    return Err(Error::InvalidMove {
                                        message: String::from("is ambiguous"),
                                    })
                                }
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
            Some((piece, position)) => return Ok((piece, position, promotion)),
            None => return Err(Error::InvalidArgument),
        }
    }

    /// Castling is handled separately because it's the only move that moves 2 pieces at once
    /// * supports chess960 castling
    pub fn castle(&mut self, input: &str, white: bool) -> bool {
        let kingside = input == "O-O";

        let rank = if white { 0 } else { 7 };
        let mut king: Option<&Piece> = None;
        for piece in &self.grid[rank] {
            match piece {
                Some(piece) => {
                    // find the king
                    if piece.id == Id::King && piece.white == white {
                        king = Some(piece);
                        break;
                    }
                }
                None => continue,
            }
        }

        if king.is_none() {
            return false;
        }

        let king = king.unwrap();
        let range = match kingside {
            true => (king.position.x + 1)..NUM_COLS,
            false => 0..king.position.x,
        };

        let mut rook: Option<&Piece> = None;
        for i in range {
            match &self.grid[rank][i] {
                Some(piece) => {
                    // find  the rook
                    if piece.id == Id::Rook && piece.white == white {
                        rook = Some(piece);
                        break;
                    }
                }
                None => continue,
            }
        }

        if rook.is_none() {
            return false;
        }

        let rook = rook.unwrap();
        if MoveChecker::can_castle(&self, king, rook, kingside, white) {
            let rank = if white { 0 } else { 7 };
            let files = match kingside {
                true => &KINGSIDE_CASTLE,
                false => &QUEENSIDE_CASTLE,
            };
            let king_target = files[0];
            let rook_target = files[1];
            let king_x = king.position.x;
            let rook_x = rook.position.y;

            // move pieces
            self.grid[rank][king_x] = None;
            self.place_piece(king_target, rank, '♔', white, 1);
            self.grid[rank][rook_x] = None;
            self.place_piece(rook_target, rank, '♖', white, 1);

            return true;
        }

        return false;
    }

    /// Moves a piece based on `input`
    /// * Returns `true` if the move is valid, `false` if not
    pub fn make_move(&mut self, input: &str, white: bool) -> bool {
        self.message.clear();
        let sanitised_input = &Self::sanitise_input(input)[..];

        // check if the move is castling
        match sanitised_input {
            "O-O" | "O-O-O" => {
                if self.castle(sanitised_input, white) {
                    return true;
                } else {
                    self.message = format!("{}Cannot castle {}", WARNING_COLOUR, input);
                    return false;
                }
            }
            _ => (),
        }

        // check if move is valid first
        let (original, position, promotion) = match self.parse_move(sanitised_input, white) {
            Ok((piece, position, promotion)) => (piece, position, promotion),
            Err(error) => {
                let error_message = match error {
                    Error::InvalidMove { message } => message,
                    _ => String::from("is not a valid move"),
                };
                self.message = format!("{}{} {}", WARNING_COLOUR, input, error_message);
                return false;
            }
        };

        // properties of moved piece
        let x = position.x;
        let y = position.y;
        let icon = match promotion {
            Some(icon) => icon,
            None => original.icon,
        };
        let white = original.white;
        let moves = original.moves + 1;

        // check if the move will put the king in check with a test board
        let mut test_board = self.clone();
        test_board.grid[original.position.y][original.position.x] = None;
        test_board.place_piece(x, y, icon, white, moves);
        if MoveChecker::in_check(&test_board, white) {
            self.message = format!("{}King would be in check", WARNING_COLOUR);
            return false;
        }

        // move piece
        self.grid[original.position.y][original.position.x] = None;
        self.place_piece(x, y, icon, white, moves);
        return true;
    }
}
