use crate::{board::Board, pieces::Piece, Coordinate};

/// sets up a default board for all tests
fn test_board() -> Board {
    let board = Board::new();
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
fn pawn_move() {
    let board = test_board();
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
    let board = test_board();
    test_move(&board, "a6", true, None);
}
