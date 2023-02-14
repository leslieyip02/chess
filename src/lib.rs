pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = 8;

pub enum Error {
    InvalidArgument,
    IndexOutOfRange,
}

/// (x, y) coordinate with A1 as (0, 0)
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Result<Coordinate, Error> {
        if x >= NUM_COLS || y >= NUM_ROWS {
            return Err(Error::IndexOutOfRange);
        }

        Ok(Coordinate { x, y })
    }

    /// Convert alphanumeric grid coordinate to 0-indexed coordinates
    /// * `position` - lowercase letter from 'a' to 'h' with a number within [0, 8)
    /// * e.g. A1 => (0, 0)
    /// * e.g. E4 => (4, 3)
    fn from_alphanumeric(position: &str) -> Result<Coordinate, Error> {
        if position.len() != 2 {
            return Err(Error::InvalidArgument);
        }

        let coordinates: Vec<char> = position.chars().collect();
        let x = coordinates[0] as usize - 97; // a is 97
        let y = coordinates[1] as usize - 49; // 0 is 48, minus 49 to be 0-indexed
        Coordinate::new(x, y)
    }
}

pub mod board {
    pub mod board;
    pub use board::Board;
}

pub mod pieces {
    pub mod moves;
    pub use moves::MoveChecker;

    pub mod piece;
    pub use piece::Id;
    pub use piece::Piece;
}

mod tests {
    #[cfg(test)]
    mod bishop_moves;

    #[cfg(test)]
    mod pawn_moves;

    #[cfg(test)]
    mod parse_moves;
}
