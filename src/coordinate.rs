use crate::board::{NUM_COLS, NUM_ROWS};
use crate::Error;

/// (x, y) coordinate with A1 as (0, 0)
#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Result<Coordinate, Error> {
        if x >= NUM_ROWS || y >= NUM_COLS {
            return Err(Error::IndexOutOfRange);
        }

        Ok(Coordinate { x, y })
    }

    /// Convert alphanumeric grid coordinate to 0-indexed coordinates
    /// * `position` - lowercase letter from 'a' to 'h' with a number within [0, 8)
    /// * e.g. A1 => (0, 0)
    /// * e.g. E4 => (4, 3)
    pub fn from_alphanumeric(position: &str) -> Result<Coordinate, Error> {
        if position.len() != 2 {
            return Err(Error::InvalidArgument);
        }

        let coordinates: Vec<char> = position.chars().collect();
        if !coordinates[0].is_lowercase() || !coordinates[1].is_numeric() {
            return Err(Error::InvalidArgument);
        }

        let x = coordinates[0] as usize - 97; // a is 97
        let y = coordinates[1] as usize - 49; // 0 is 48, minus 49 to be 0-indexed
        Coordinate::new(x, y)
    }
}
