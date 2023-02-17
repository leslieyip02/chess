use crate::board::*;
use crate::coordinate::Coordinate;
use crate::pieces::{Id, Piece};

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

    /// Checks if any pieces blocking along the 2 coorindates
    fn blocked(board: &Board, from: &Coordinate, to: &Coordinate, white: bool) -> bool {
        // convert to signed so dx and dy can take negative values
        let mut x1 = from.x as i8;
        let mut y1 = from.y as i8;
        let x2 = to.x as i8;
        let y2 = to.y as i8;

        // normalise dx and dy to be 1, 0 or -1
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        while x1 != x2 || y1 != y2 {
            x1 += dx;
            y1 += dy;

            match &board.grid[y1 as usize][x1 as usize] {
                Some(piece) => {
                    // if reached target position,
                    // check if it's a capture or it's blocked
                    if x1 == x2 && y1 == y2 {
                        return piece.white == white;
                    }

                    return true;
                }
                None => (),
            }
        }
        return false;
    }

    /// Checks if a piece can move to a position
    pub fn can_move(&self, board: &Board, piece: &Piece, position: &Coordinate) -> bool {
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

        // unsigned distance along x and y axes
        let dx = piece.position.x.abs_diff(position.x);
        let dy = piece.position.y.abs_diff(position.y);

        // check if the move is actually a move
        if dx == 0 && dy == 0 {
            return false;
        }

        match self {
            Self::Bishop => {
                // check if diagonal
                if !(dx == dy) {
                    return false;
                }

                return !Self::blocked(board, &piece.position, position, piece.white);
            }
            Self::King => {
                // 1 squre in cardinal and ordinal directions
                if !((dx + dy) == 1 || (dx == 1 && dy == 1)) {
                    return false;
                }
                
                return !Self::blocked(board, &piece.position, position, piece.white);
            }
            Self::Knight => {
                // L-shape, don't need to check if blocked
                return (dx == 1 && dy == 2) || (dx == 2 && dy == 1);
            }
            Self::Pawn => {
                // TODO: google en passant

                // check if the pawn is moving in the correct direction,
                // and whether the position is within range
                if (position.y > piece.position.y) != piece.white || dy > 2 {
                    return false;
                }

                // if diagonal, check if a piece exists there
                if dx != 0 {
                    // check if the distance is correct
                    if dx != 1 || dy != 1 {
                        return false;
                    }

                    // can only move diagonal when taking
                    return match &board.grid[position.y][position.x] {
                        Some(_) => true,
                        None => false,
                    };
                }

                // if the piece has not moved, can move 2
                let starting_rank = if piece.white { 1 } else { 6 };
                if dy == 2 && piece.position.y != starting_rank {
                    return false;
                }

                // check if a piece occupies the target position
                // this requires a different check because pawns can only capture along a diagonal
                match &board.grid[position.y][position.x] {
                    Some(_) => {
                        return false;
                    }
                    None => (),
                }

                return !Self::blocked(board, &piece.position, position, piece.white);
            }
            Self::Queen => {
                // check if diagonal or vertical or horizontal
                if !(dx == dy || dx == 0 || dy == 0) {
                    return false;
                }

                return !Self::blocked(board, &piece.position, position, piece.white);
            }
            Self::Rook => {
                // check if vertical or horizontal
                if !(dx == 0 || dy == 0) {
                    return false;
                }

                return !Self::blocked(board, &piece.position, position, piece.white);
            }
        };
    }

    pub fn in_check(board: &Board, white: bool) -> bool {
        let mut position: Option<&Coordinate> = None;
        for row in &board.grid {
            for piece in row {
                match piece {
                    Some(piece) => {
                        // find the king
                        if piece.white == white && piece.id == Id::King {
                            position = Some(&piece.position);
                        }
                    }
                    None => continue,
                }
            }
        }
        
        // there might not be a king for custom boards, so just return false
        let position = match position {
            Some(position) => position,
            None => return false,
        };
        
        for row in &board.grid {
            for piece in row {
                match piece {
                    // check if any pieces can attack the king
                    Some(piece) => {
                        if piece.white != white {
                            let checker = MoveChecker::from_id(&piece.id);
                            if checker.can_move(&board, &piece, &position) {
                                return true;
                            }
                        }
                    }
                    None => continue,
                }
            }
        }

        return false;
    }
}
