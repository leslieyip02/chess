use super::test_move;
use crate::board::Board;

#[test]
fn single_forward() {
    let board = Board::from_vec(&vec![(0, 1, "♙", true), (0, 6, "♙", false)]);
    test_move(&board, 0, 1, 0, 2, true);
    test_move(&board, 0, 6, 0, 5, true);
}

#[test]
fn double_forward() {
    let board = Board::from_vec(&vec![
        (0, 1, "♙", true),
        (0, 6, "♙", false),
        (1, 2, "♙", true),
        (1, 5, "♙", false),
    ]);
    test_move(&board, 0, 1, 0, 3, true);
    test_move(&board, 0, 6, 0, 4, true);
    test_move(&board, 1, 2, 1, 4, false);
    test_move(&board, 1, 5, 1, 3, false);
}

#[test]
fn invalid() {
    let board = Board::from_vec(&vec![(0, 1, "♙", true), (0, 6, "♙", false)]);
    test_move(&board, 0, 1, 0, 0, false);
    test_move(&board, 0, 1, 1, 1, false);
    test_move(&board, 0, 1, 1, 2, false);
    test_move(&board, 0, 1, 0, 4, false);
}

#[test]
fn capture() {
    let board = Board::from_vec(&vec![
        (1, 1, "♙", true),
        (0, 2, "♙", false),
        (2, 2, "♙", false),
        (3, 3, "♙", false),
    ]);
    test_move(&board, 1, 1, 0, 2, true);
    test_move(&board, 1, 1, 2, 2, true);
    test_move(&board, 3, 3, 2, 2, false);
}

#[test]
fn blocked() {
    let board = Board::from_vec(&vec![
        (0, 1, "♙", true),
        (0, 2, "♙", false),
        (1, 1, "♙", true),
        (1, 3, "♙", true),
    ]);
    test_move(&board, 0, 1, 0, 2, false);
    test_move(&board, 0, 1, 0, 3, false);
    test_move(&board, 1, 1, 1, 3, false);
}
