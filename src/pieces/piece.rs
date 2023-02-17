use crate::Error;
use crate::coordinate::Coordinate;

#[derive(Clone, PartialEq)]
pub enum Id {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

impl Id {
    pub fn from_char(icon: char) -> Result<Id, Error> {
        match icon {
            'B' | '♗' => Ok(Self::Bishop),
            'K' | '♔' => Ok(Self::King),
            'N' | '♘' => Ok(Self::Knight),
            'P' | '♙' => Ok(Self::Pawn),
            'Q' | '♕' => Ok(Self::Queen),
            'R' | '♖' => Ok(Self::Rook),
            _ => Err(Error::InvalidArgument),
        }
    }
}

/// Basic properties for each piece
/// * `position` - [Coordinate]
/// * `id` - [Id]
/// * `icon` - unicode with combining characters
/// * `white` - `true` for white, `false` for black
#[derive(Clone)]
pub struct Piece {
    pub position: Coordinate,
    pub id: Id,
    pub icon: char,
    pub white: bool,
}

impl Piece {
    pub fn new(x: usize, y: usize, icon: char, white: bool) -> Result<Piece, Error> {
        let position = Coordinate::new(x, y)?;
        let id = Id::from_char(icon)?;

        Ok(Piece {
            position,
            id,
            icon,
            white,
        })
    }
}
