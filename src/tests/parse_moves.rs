use crate::board::Board;
use crate::pieces::Piece;

/// Tests if a move is parsed correctly
fn test_input(
    board: &Board,
    input: &str,
    white: bool,
    expected: Option<(&Piece, usize, usize, Option<char>)>,
) {
    match board.parse_move(input, white) {
        Ok((piece, position, promotion)) => match expected {
            Some((expected_piece, expected_x, expected_y, expected_promotion)) => {
                // check piece properties
                assert_eq!(piece.position.x, expected_piece.position.x);
                assert_eq!(piece.position.y, expected_piece.position.y);
                assert_eq!(piece.icon, expected_piece.icon);
                assert_eq!(piece.white, expected_piece.white);
                // check target position
                assert_eq!(position.x, expected_x);
                assert_eq!(position.y, expected_y);
                // check promotion
                match promotion {
                    Some(promoted_icon) => match expected_promotion {
                        Some(expected_icon) => assert!(promoted_icon == expected_icon),
                        None => assert!(false),
                    },
                    None => assert!(expected_promotion.is_none()),
                }
            }
            None => assert!(false),
        },
        // if invalid, expected should be none
        Err(_) => assert!(expected.is_none()),
    }
}

#[test]
fn bishop() {
    let board = Board::from_vec(&vec![(3, 3, '♗', true)]);
    let piece = board.grid[3][3].as_ref().unwrap();
    test_input(&board, "Bg7", true, Some((piece, 6, 6, None)));
}

#[test]
fn king() {
    let board = Board::from_vec(&vec![(3, 3, '♔', true)]);
    let piece = board.grid[3][3].as_ref().unwrap();
    test_input(&board, "Ke4", true, Some((piece, 4, 3, None)));
}

#[test]
fn knight() {
    let board = Board::from_vec(&vec![(3, 3, '♘', true)]);
    let piece = board.grid[3][3].as_ref().unwrap();
    test_input(&board, "Nf3", true, Some((piece, 5, 2, None)));
}

#[test]
fn pawn() {
    let board = Board::new();
    let piece = board.grid[1][4].as_ref().unwrap();
    test_input(&board, "e4", true, Some((piece, 4, 3, None)));
}

#[test]
fn queen() {
    let board = Board::from_vec(&vec![(3, 3, '♕', true)]);
    let piece = board.grid[3][3].as_ref().unwrap();
    test_input(&board, "Qa4", true, Some((piece, 0, 3, None)));
    test_input(&board, "Qg7", true, Some((piece, 6, 6, None)));
}

#[test]
fn rook() {
    let board = Board::from_vec(&vec![(3, 3, '♖', true)]);
    let piece = board.grid[3][3].as_ref().unwrap();
    test_input(&board, "Ra4", true, Some((piece, 0, 3, None)));
}

#[test]
fn invalid() {
    let board = Board::new();
    test_input(&board, "a6", true, None);
}

#[test]
fn in_check() {
    let mut board = Board::from_vec(&vec![(3, 3, '♔', true), (4, 4, '♕', false)]);
    let piece = board.grid[3][3].as_ref().unwrap();
    test_input(&board, "Kd3", true, Some((piece, 3, 2, None)));
    assert_eq!(board.make_move("Kc3", true), false);
    assert_eq!(board.message, "\u{001b}[31mKing would be in check");
}

#[test]
fn ambiguous() {
    let mut bishop_board = Board::from_vec(&vec![
        (3, 3, '♗', true),
        (3, 5, '♗', true),
        (5, 3, '♗', true),
    ]);
    let bishop_1 = bishop_board.grid[3][3].as_ref().unwrap();
    let bishop_2 = bishop_board.grid[5][3].as_ref().unwrap();
    let bishop_3 = bishop_board.grid[3][5].as_ref().unwrap();
    test_input(&bishop_board, "Be5", true, None);
    test_input(&bishop_board, "B4e5", true, None);
    test_input(&bishop_board, "Bde5", true, None);
    test_input(&bishop_board, "Bd4e5", true, Some((bishop_1, 4, 4, None)));
    test_input(&bishop_board, "Bd6e5", true, Some((bishop_2, 4, 4, None)));
    test_input(&bishop_board, "Bf4e5", true, Some((bishop_3, 4, 4, None)));
    test_input(&bishop_board, "Bde3", true, Some((bishop_1, 4, 2, None)));
    test_input(&bishop_board, "B6c5", true, Some((bishop_2, 2, 4, None)));
    assert_eq!(bishop_board.make_move("B4e5", true), false);
    assert_eq!(bishop_board.message, "\u{001b}[31mB4e5 is ambiguous");

    let mut pawn_board = Board::new();
    pawn_board.place_piece(4, 2, '♙', false);
    let pawn_1 = pawn_board.grid[1][3].as_ref().unwrap();
    let pawn_2 = pawn_board.grid[1][5].as_ref().unwrap();
    test_input(&pawn_board, "dxe3", true, Some((pawn_1, 4, 2, None)));
    test_input(&pawn_board, "fxe3", true, Some((pawn_2, 4, 2, None)));
}

#[test]
fn promotion() {
    let mut board = Board::from_vec(&vec![
        (0, 6, '♙', true),
        (1, 7, '♗', false),
        (1, 1, '♙', false),
        (3, 3, '♙', true),
    ]);
    let pawn_1 = board.grid[6][0].as_ref().unwrap();
    let pawn_2 = board.grid[1][1].as_ref().unwrap();
    test_input(&board, "a8Q", true, Some((pawn_1, 0, 7, Some('♕'))));
    test_input(&board, "axb8=Q", true, Some((pawn_1, 1, 7, Some('♕'))));
    test_input(&board, "b1B", false, Some((pawn_2, 1, 0, Some('♗'))));
    assert_eq!(board.make_move("a8", true), false);
    assert_eq!(board.message, "\u{001b}[31ma8 is not valid because promotion is forced");
    assert_eq!(board.make_move("d5=Q", true), false);
    assert_eq!(board.message, "\u{001b}[31md5=Q is not a valid promotion");
}

#[test]
fn bad_input() {
    let board = Board::new();
    test_input(&board, "NC3", true, None);
    test_input(&board, "asufuihjdlakbhf", true, None);
    test_input(&board, "!@#$%^&*()", true, None);
    test_input(&board, "", true, None);
}
