pub struct Piece {
    pub icon: String,
}

impl Piece {
    pub fn new(icon: &str) -> Piece {
        // \u{fe0e} increases the size of the pieces in command prompt
        Piece {
            icon: format!("{}{}", icon, "\u{fe0e}"),
        }
    }
}
