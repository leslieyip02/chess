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
    assert_eq!(MoveChecker::in_check(&board, true), true);
}
