pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = 8;

/// (x, y) coordinate with A1 as (0, 0)
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

pub mod board {
    pub mod board;
    pub use board::Board;
}

pub mod pieces {
    pub mod moves;
    pub use moves::MoveChecker;

    pub mod piece;
    pub use piece::Piece;
    pub use piece::Id;
}

pub mod tests {
    #[cfg(test)]
    pub mod bishop_moves;
    
    #[cfg(test)]
    pub mod pawn_moves;
}