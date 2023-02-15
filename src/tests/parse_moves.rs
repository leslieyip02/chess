use crate::board::Board;
use crate::pieces::Piece;

/// Tests if a move is parsed correctly
fn test_input(board: &Board, input: &str, white: bool, expected: Option<(&Piece, usize, usize)>) {
    match board.parse_move(input, white) {
        Ok((piece, position)) => match expected {
            Some((expected_piece, expected_x, expected_y)) => {
                // check piece properties
                assert_eq!(piece.position.x, expected_piece.position.x);
                assert_eq!(piece.position.y, expected_piece.position.y);
                assert_eq!(piece.icon, expected_piece.icon);
                assert_eq!(piece.white, expected_piece.white);
                // check target position
                assert_eq!(position.x, expected_x);
                assert_eq!(position.y, expected_y);
            }
            None => assert!(false),
        },
        // if invalid, expected should be none
        Err(_) => assert!(expected.is_none()),
    }
}

#[test]
fn pawn() {
    let board = Board::new();
    let piece = board.grid[1][4].as_ref().unwrap();
    test_input(&board, "e4", true, Some((piece, 4, 3)));
}

#[test]
fn invalid() {
    let board = Board::new();
    test_input(&board, "a6", true, None);
}

#[test]
fn ambiguous() {
    let bishop_board = Board::from_vec(&vec![(3, 3, "♗", true), (3, 5, "♗", true)]);
    let bishop_1 = bishop_board.grid[3][3].as_ref().unwrap();
    test_input(&bishop_board, "B4e5", true, Some((bishop_1, 4, 4)));
    let bishop_2 = bishop_board.grid[5][3].as_ref().unwrap();
    test_input(&bishop_board, "B6e5", true, Some((bishop_2, 4, 4)));

    let mut pawn_board = Board::new();
    pawn_board.place_piece(4, 2, "♙", false);
    let pawn_1 = pawn_board.grid[1][3].as_ref().unwrap();
    test_input(&pawn_board, "dxe3", true, Some((pawn_1, 4, 2)));
    let pawn_2 = pawn_board.grid[1][5].as_ref().unwrap();
    test_input(&pawn_board, "fxe3", true, Some((pawn_2, 4, 2)));
}
