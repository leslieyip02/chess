use super::test_move;
use crate::{board::Board, pieces::MoveChecker};

#[test]
fn straight() {
    let board = Board::from_vec(&vec![(3, 3, '♔', true)]);
    test_move(&board, 3, 3, 2, 3, true);
    test_move(&board, 3, 3, 4, 3, true);
    test_move(&board, 3, 3, 3, 2, true);
    test_move(&board, 3, 3, 3, 4, true);
}

#[test]
fn diagonal() {
    let board = Board::from_vec(&vec![(3, 3, '♔', true)]);
    test_move(&board, 3, 3, 2, 2, true);
    test_move(&board, 3, 3, 2, 4, true);
    test_move(&board, 3, 3, 4, 2, true);
    test_move(&board, 3, 3, 4, 4, true);
}

#[test]
fn invalid() {
    let board = Board::from_vec(&vec![(3, 3, '♔', true)]);
    test_move(&board, 3, 3, 3, 5, false);
    test_move(&board, 3, 3, 5, 3, false);
    test_move(&board, 3, 3, 5, 5, false);
}

#[test]
fn capture() {
    let board = Board::from_vec(&vec![(3, 3, '♔', true), (4, 3, '♙', false)]);
    test_move(&board, 3, 3, 4, 3, true);
}

#[test]
fn blocked() {
    let board = Board::from_vec(&vec![
        (3, 3, '♔', true),
        (3, 4, '♙', true),
        (4, 4, '♙', true),
    ]);
    test_move(&board, 3, 3, 3, 4, false);
    test_move(&board, 3, 3, 4, 4, false);
}

#[test]
fn in_check() {
    let board = Board::from_vec(&vec![(3, 3, '♔', true), (4, 4, '♕', false)]);
    assert!(MoveChecker::in_check(&board, true));
}

#[test]
fn castle() {
    let board = Board::from_vec(&vec![
        (4, 0, '♔', true),
        (0, 0, '♖', true),
        (7, 0, '♖', true),
    ]);
    let king = board.grid[0][4].as_ref().unwrap();
    let rook_k = board.grid[0][7].as_ref().unwrap();
    let rook_q = board.grid[0][0].as_ref().unwrap();
    assert!(MoveChecker::can_castle(&board, &king, rook_k, false, true));
    assert!(MoveChecker::can_castle(&board, &king, rook_q, true, true));

    let mut king = king.clone();
    king.moves += 1;
    assert!(!MoveChecker::can_castle(&board, &king, rook_k, true, true),);
    assert!(!MoveChecker::can_castle(&board, &king, rook_q, false, true),);
}

#[test]
fn castle_in_check() {
    let board = Board::from_vec(&vec![
        (4, 0, '♔', true),
        (0, 0, '♖', true),
        (3, 7, '♖', false),
    ]);
    let king = board.grid[0][4].as_ref().unwrap();
    let rook = board.grid[0][0].as_ref().unwrap();
    assert!(!MoveChecker::can_castle(&board, &king, rook, false, true));
}

#[test]
fn castle_while_blocked() {
    let board = Board::from_vec(&vec![
        (4, 0, '♔', true),
        (0, 0, '♖', true),
        (7, 0, '♖', true),
        (3, 0, '♕', true),
        (6, 0, '♖', false),
    ]);
    let king = board.grid[0][4].as_ref().unwrap();
    let rook_k = board.grid[0][7].as_ref().unwrap();
    let rook_q = board.grid[0][0].as_ref().unwrap();
    assert!(!MoveChecker::can_castle(&board, &king, rook_k, false, true));
    assert!(!MoveChecker::can_castle(&board, &king, rook_q, true, true));
}

#[test]
fn castle_960() {
    let board = Board::from_vec(&vec![
        (1, 0, '♔', true),
        (0, 0, '♖', true),
        (4, 0, '♖', true),
    ]);
    let king = board.grid[0][1].as_ref().unwrap();
    let rook_k = board.grid[0][4].as_ref().unwrap();
    let rook_q = board.grid[0][0].as_ref().unwrap();
    assert!(MoveChecker::can_castle(&board, &king, rook_k, true, true));
    assert!(MoveChecker::can_castle(&board, &king, rook_q, false, true));
}
