use super::test_move;
use crate::board::Board;

#[test]
fn straight() {
    let board = Board::from_vec(&vec![(3, 3, '♕', true)]);
    test_move(&board, 3, 3, 0, 3, true);
    test_move(&board, 3, 3, 7, 3, true);
    test_move(&board, 3, 3, 3, 0, true);
    test_move(&board, 3, 3, 3, 7, true);
}

#[test]
fn diagonal() {
    let board = Board::from_vec(&vec![(3, 3, '♕', true)]);
    test_move(&board, 3, 3, 1, 5, true);
    test_move(&board, 3, 3, 5, 5, true);
    test_move(&board, 3, 3, 1, 1, true);
    test_move(&board, 3, 3, 5, 1, true);
}

#[test]
fn invalid() {
    let board = Board::from_vec(&vec![(3, 3, '♕', true)]);
    test_move(&board, 3, 3, 6, 7, false);
    test_move(&board, 3, 3, 2, 5, false);
}

#[test]
fn capture() {
    let board = Board::from_vec(&vec![
        (3, 3, '♕', true),
        (0, 3, '♕', false),
        (0, 0, '♕', false),
    ]);
    test_move(&board, 3, 3, 0, 3, true);
    test_move(&board, 3, 3, 0, 0, true);
}

#[test]
fn blocked() {
    let board = Board::from_vec(&vec![
        (3, 3, '♕', true),
        (5, 3, '♕', true),
        (6, 6, '♕', true),
    ]);
    test_move(&board, 3, 3, 5, 3, false);
    test_move(&board, 3, 3, 7, 3, false);
    test_move(&board, 3, 3, 6, 6, false);
    test_move(&board, 3, 3, 7, 7, false);
}
