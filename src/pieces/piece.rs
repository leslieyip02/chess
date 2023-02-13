// \u{001b}[38;5;<n>m -> foreground colour for some n
pub const WHITE: &str = "\u{001b}[38;5;242m";
pub const BLACK: &str = "\u{001b}[38;5;232m";

pub struct Piece {
    pub icon: String,
    pub white: bool,
}

impl Piece {
    pub fn new(icon: &str, white: bool) -> Piece {
        let colour = if white { WHITE } else { BLACK };
        // \u{fe0e} increases the size of the pieces in command prompt
        Piece {
            icon: format!("{}{}\u{fe0e}", colour, icon),
            white,
        }
    }
}
