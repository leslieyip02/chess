use crate::{board::Board, pieces::Piece, Coordinate};

/// sets up a default board for all tests
fn test_board() -> Board {
    let mut board = Board::empty();
    board.place_piece(3, 3, "♗", true);
    board.place_piece(3, 5, "♗", true);
    return board;
}

/// Tests if a move is parsed correctly
fn test_move(board: &Board, action: &str, white: bool, expected: Option<(&Piece, &Coordinate)>) {
    match board.parse_move(action, white) {
        Ok((piece, position)) => match expected {
            Some((expected_piece, expected_position)) => {
                // check piece properties
                assert_eq!(piece.position.x, expected_piece.position.x);
                assert_eq!(piece.position.y, expected_piece.position.y);
                assert_eq!(piece.icon, expected_piece.icon);
                assert_eq!(piece.white, expected_piece.white);
                // check target position
                assert_eq!(position.x, expected_position.x);
                assert_eq!(position.y, expected_position.y);
            }
            None => assert!(false),
        },
        // if invalid, expected should be none
        Err(_) => {
            match expected {
                Some(_) => assert!(false),
                None => assert!(true),
            };
        }
    }
}

#[test]
fn ambiguous_bishop_move() {
    let board = test_board();

    match &board.grid[3][3] {
        Some(piece) => {
            let position = Coordinate { x: 4, y: 4 };
            test_move(&board, "B4e5", true, Some((&piece, &position)));
        }
        None => assert!(false),
    }

    match &board.grid[5][3] {
        Some(piece) => {
            let position = Coordinate { x: 4, y: 4 };
            test_move(&board, "B6e5", true, Some((&piece, &position)));
        }
        None => assert!(false),
    }
}

#[test]
fn pawn_move() {
    let board = Board::new();
    match &board.grid[1][4] {
        Some(piece) => {
            let position = Coordinate { x: 4, y: 3 };
            test_move(&board, "e4", true, Some((&piece, &position)));
        }
        None => assert!(false),
    }
}

#[test]
fn invalid_pawn_move() {
    let board = Board::new();
    test_move(&board, "a6", true, None);
}

#[test]
fn ambiguous_pawn_move() {
    let mut board = Board::new();
    board.place_piece(4, 2, "♙", false);

    match &board.grid[1][3] {
        Some(piece) => {
            let position = Coordinate { x: 4, y: 2 };
            test_move(&board, "dxe3", true, Some((&piece, &position)));
        }
        None => assert!(false),
    }

    match &board.grid[1][5] {
        Some(piece) => {
            let position = Coordinate { x: 4, y: 2 };
            test_move(&board, "fxe3", true, Some((&piece, &position)));
        }
        None => assert!(false),
    }
}
