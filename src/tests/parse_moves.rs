use crate::board::*;
use crate::pieces::moves::MoveType;
use crate::pieces::Piece;

/// Tests if a move is parsed correctly
fn test_normal_input(
    board: &Board,
    input: &str,
    white: bool,
    expected: Option<(&Piece, usize, usize, Option<char>)>,
) {
    match board.parse_move(input, white) {
        Ok(move_type) => match move_type {
            MoveType::Normal {
                piece,
                target,
                promotion,
            } => match expected {
                Some((expected_piece, expected_x, expected_y, expected_promotion)) => {
                    // check piece properties
                    assert_eq!(piece.position.x, expected_piece.position.x);
                    assert_eq!(piece.position.y, expected_piece.position.y);
                    assert_eq!(piece.icon, expected_piece.icon);
                    assert_eq!(piece.white, expected_piece.white);
                    // check target position
                    assert_eq!(target.x, expected_x);
                    assert_eq!(target.y, expected_y);
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
            _ => assert!(false),
        },
        // if invalid, expected should be none
        Err(_) => assert!(expected.is_none()),
    }
}

#[test]
fn bishop() {
    let board = Board::from_vec(&vec![(3, 3, '♗', true)]);
    let bishop = board.grid[3][3].as_ref().unwrap();
    test_normal_input(&board, "Bg7", true, Some((bishop, 6, 6, None)));
}

#[test]
fn king() {
    let board = Board::from_vec(&vec![(3, 3, '♔', true)]);
    let king = board.grid[3][3].as_ref().unwrap();
    test_normal_input(&board, "Ke4", true, Some((king, 4, 3, None)));
}

#[test]
fn knight() {
    let board = Board::from_vec(&vec![(3, 3, '♘', true)]);
    let knight = board.grid[3][3].as_ref().unwrap();
    test_normal_input(&board, "Nf3", true, Some((knight, 5, 2, None)));
}

#[test]
fn pawn() {
    let board = Board::new();
    let pawn = board.grid[1][4].as_ref().unwrap();
    test_normal_input(&board, "e4", true, Some((pawn, 4, 3, None)));
}

#[test]
fn queen() {
    let board = Board::from_vec(&vec![(3, 3, '♕', true)]);
    let queen = board.grid[3][3].as_ref().unwrap();
    test_normal_input(&board, "Qa4", true, Some((queen, 0, 3, None)));
    test_normal_input(&board, "Qg7", true, Some((queen, 6, 6, None)));
}

#[test]
fn rook() {
    let board = Board::from_vec(&vec![(3, 3, '♖', true)]);
    let rook = board.grid[3][3].as_ref().unwrap();
    test_normal_input(&board, "Ra4", true, Some((rook, 0, 3, None)));
}

#[test]
fn invalid() {
    let board = Board::new();
    test_normal_input(&board, "a6", true, None);
}

#[test]
fn in_check() {
    let mut board = Board::from_vec(&vec![(3, 3, '♔', true), (4, 4, '♕', false)]);
    let king = board.grid[3][3].as_ref().unwrap();
    test_normal_input(&board, "Kd3", true, Some((king, 3, 2, None)));
    assert!(!board.make_move("Kc3", true));
    assert_eq!(board.message, "\u{001b}[31mKc3 puts the king in check");
}

#[test]
fn ambiguous() {
    let mut board = Board::from_vec(&vec![
        (3, 3, '♗', true),
        (3, 5, '♗', true),
        (5, 3, '♗', true),
    ]);
    let bishop_1 = board.grid[3][3].as_ref().unwrap();
    let bishop_2 = board.grid[5][3].as_ref().unwrap();
    let bishop_3 = board.grid[3][5].as_ref().unwrap();
    test_normal_input(&board, "Be5", true, None);
    test_normal_input(&board, "B4e5", true, None);
    test_normal_input(&board, "Bde5", true, None);
    test_normal_input(&board, "Bd4e5", true, Some((bishop_1, 4, 4, None)));
    test_normal_input(&board, "Bd6e5", true, Some((bishop_2, 4, 4, None)));
    test_normal_input(&board, "Bf4e5", true, Some((bishop_3, 4, 4, None)));
    test_normal_input(&board, "Bde3", true, Some((bishop_1, 4, 2, None)));
    test_normal_input(&board, "B6c5", true, Some((bishop_2, 2, 4, None)));
    assert!(!board.make_move("B4e5", true));
    assert_eq!(board.message, "\u{001b}[31mB4e5 is ambiguous");
}

#[test]
fn ambiguous_pawn() {
    let mut board = Board::new();
    board.place_piece(4, 2, '♙', false, 0);
    let pawn_1 = board.grid[1][3].as_ref().unwrap();
    let pawn_2 = board.grid[1][5].as_ref().unwrap();
    test_normal_input(&board, "dxe3", true, Some((pawn_1, 4, 2, None)));
    test_normal_input(&board, "fxe3", true, Some((pawn_2, 4, 2, None)));
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
    test_normal_input(&board, "a8Q", true, Some((pawn_1, 0, 7, Some('♕'))));
    test_normal_input(&board, "axb8Q", true, Some((pawn_1, 1, 7, Some('♕'))));
    test_normal_input(&board, "b1B", false, Some((pawn_2, 1, 0, Some('♗'))));
    assert!(!board.make_move("a8", true));
    assert_eq!(
        board.message,
        "\u{001b}[31ma8 is not valid because promotion is forced"
    );
    assert!(!board.make_move("d5=Q", true));
    assert_eq!(board.message, "\u{001b}[31md5=Q is not a valid promotion");
}

#[test]
fn castle() {
    let mut board = Board::from_vec(&vec![
        (4, 0, '♔', true),
        (0, 0, '♖', true),
        (7, 0, '♖', true),
    ]);
    assert!(board.make_move("O-O", true));
    assert!(board.grid[0][KINGSIDE_CASTLE[0]].as_ref().unwrap().icon == '♔');
    assert!(board.grid[0][KINGSIDE_CASTLE[1]].as_ref().unwrap().icon == '♖');
}

#[test]
fn en_passant() {
    let mut board = Board::from_vec(&vec![
        (1, 4, '♙', true),
        (0, 6, '♙', false),
        (2, 6, '♙', false),
    ]);
    board.make_move("c6", false);
    board.make_move("c5", false);
    assert!(!board.make_move("bxc6", true));
    board.make_move("a5", false);
    assert!(board.make_move("bxa6", true));
    assert!(board.grid[5][0].as_ref().unwrap().icon == '♙');
    assert!(board.grid[5][0].as_ref().unwrap().white);
    assert!(board.grid[4][0].is_none());
}

#[test]
fn bad_input() {
    let board = Board::new();
    test_normal_input(&board, "NC3", true, None);
    test_normal_input(&board, "asufuihjdlakbhf", true, None);
    test_normal_input(&board, "!@#$%^&*()", true, None);
    test_normal_input(&board, "", true, None);
}
