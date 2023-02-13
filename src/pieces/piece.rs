use crate::Coordinate;

// \u{001b}[38;5;<n>m -> foreground colour for some n
pub const WHITE: &str = "\u{001b}[38;5;242m";
pub const BLACK: &str = "\u{001b}[38;5;232m";

/// Basic properties for each piece
/// * `position` - [Coordinate]
/// * `icon` - unicode with combining characters
/// * `white` - `true` for white, `false` for black
pub struct Piece {
    pub position: Coordinate,
    pub icon: String,
    pub white: bool,
}

impl Piece {
    pub fn new(x: usize, y: usize, icon: &str, white: bool) -> Piece {
        let position = Coordinate { x, y };

        // both black and white pieces use the unicode white pieces
        // because the unicode black pawn is coloured by default in command prompt
        let colour = if white { WHITE } else { BLACK };
        let icon = format!("{}{}\u{fe0e}", colour, icon);

        Piece {
            position,
            icon,
            white,
        }
    }
}
