pub enum Error {
    InvalidArgument,
    IndexOutOfRange,
}

pub mod board;
pub mod coordinate;
pub mod pieces {
    pub mod moves;
    pub use moves::MoveChecker;

    pub mod piece;
    pub use piece::Id;
    pub use piece::Piece;
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::coordinate::Coordinate;
    use crate::pieces::MoveChecker;

    /// Tests if a piece at (x1, y1) can move to (x2, y2)
    fn test_move(board: &Board, x1: usize, y1: usize, x2: usize, y2: usize, expected: bool) {
        let position = Coordinate { x: x2, y: y2 };
        match &board.grid[y1][x1] {
            Some(piece) => {
                let moves = MoveChecker::from_id(&piece.id);
                assert_eq!(moves.can_move(&piece, &position, &board), expected);
            }
            None => assert!(false),
        }
    }

    mod bishop_moves;
    mod knight_moves;
    mod pawn_moves;
    mod rook_moves;

    mod parse_moves;
}
