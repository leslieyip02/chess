use crate::board::*;
use crate::pieces::{Id, Piece};
use crate::coordinate::Coordinate;

pub enum MoveChecker {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

impl MoveChecker {
    pub fn from_id(id: &Id) -> Self {
        match id {
            Id::Bishop => Self::Bishop,
            Id::King => Self::King,
            Id::Knight => Self::Knight,
            Id::Queen => Self::Queen,
            Id::Rook => Self::Rook,
            Id::Pawn => Self::Pawn,
        }
    }

    /// Checks if a piece can move to a position
    pub fn can_move(&self, piece: &Piece, position: &Coordinate, board: &Board) -> bool {
        // check if move is in within the board
        // don't need to check negative since x and y are unsigned
        if position.x >= NUM_COLS || position.y >= NUM_ROWS {
            return false;
        }

        // check if the target position contains a friendly piece
        match &board.grid[position.y][position.x] {
            Some(target) => {
                if piece.white == target.white {
                    return false;
                }
            }
            None => (),
        }

        // TODO: check if in check
        // TODO: check if piece is pinned

        // convert to signed to check direction
        let mut x1 = piece.position.x as i8;
        let mut y1 = piece.position.y as i8;
        let x2 = position.x as i8;
        let y2 = position.y as i8;

        let dx = x2 - x1;
        let dy = y2 - y1;

        // check if the move is actually a move
        if dx == 0 && dy == 0 {
            return false;
        }

        return match self {
            Self::Bishop => {
                // check if diagonal
                if dx.abs() != dy.abs() {
                    return false;
                }

                // check if any pieces blocking
                let mut blocked = false;
                x1 += dx.signum();
                y1 += dy.signum();
                while x1 != x2 && y1 != y2 {
                    match &board.grid[y1 as usize][x1 as usize] {
                        Some(_) => {
                            blocked = true;
                            break;
                        }
                        None => {
                            x1 += dx.signum();
                            y1 += dy.signum();
                        }
                    }
                }

                return !blocked;
            }
            // 1 squre in cardinal and ordinal directions
            Self::King => {
                // TODO: check if move will put king in check
                (dx + dy) == 1 || (dx == 1 && dy == 1)
            }
            // L-shape, don't need to check if blocked
            Self::Knight => (dx == 1 && dy == 2) || (dx == 2 && dy == 1),
            Self::Pawn => {
                // TODO: google en passant

                // check if the pawn is moving in the correct direction
                if (dy > 0) != piece.white {
                    return false;
                }

                // if diagonal, check if a piece exists there
                if dx != 0 {
                    // check if the distance is correct
                    if dx.abs() != 1 || dy.abs() != 1 {
                        return false;
                    }

                    // can only move diagonal when taking
                    return match &board.grid[position.y][position.x] {
                        Some(_) => true,
                        None => false,
                    };
                }

                // check if the position is within reach
                if dy.abs() > 2 {
                    return false;
                }

                // if the piece has not moved, can move 2
                let starting_rank = if piece.white { 1 } else { 6 };
                if dy.abs() == 2 && y1 != starting_rank {
                    return false;
                }

                let mut blocked = false;
                for _ in 0..dy.abs() {
                    y1 += dy.signum();
                    match board.grid[y1 as usize][x1 as usize] {
                        Some(_) => {
                            blocked = true;
                            break;
                        }
                        None => continue,
                    }
                }

                return !blocked;
            }
            // unlimited squares in cardinal and ordinal directions
            Self::Queen => dx == dy || dx == 0 || dy == 0,
            // horizontals and verticals
            Self::Rook => dx == 0 || dy == 0,
        };
    }
}
