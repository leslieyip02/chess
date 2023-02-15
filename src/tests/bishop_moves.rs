use super::test_move;
use crate::board::Board;

#[test]
fn diagonal() {
    let board = Board::from_vec(&vec![(3, 3, "♗", true)]);
    test_move(&board, 3, 3, 1, 5, true);
    test_move(&board, 3, 3, 5, 5, true);
    test_move(&board, 3, 3, 1, 1, true);
    test_move(&board, 3, 3, 5, 1, true);
}

#[test]
fn invalid() {
    let board = Board::from_vec(&vec![(3, 3, "♗", true)]);
    test_move(&board, 3, 3, 8, 8, false);
    test_move(&board, 3, 3, 3, 5, false);
    test_move(&board, 3, 3, 5, 3, false);
}

#[test]
fn capture() {
    let board = Board::from_vec(&vec![(3, 3, "♗", true), (0, 0, "♗", false)]);
    test_move(&board, 3, 3, 0, 0, true);
}

#[test]
fn blocked() {
    let board = Board::from_vec(&vec![(3, 3, "♗", true), (6, 6, "♗", true)]);
    test_move(&board, 3, 3, 6, 6, false);
    test_move(&board, 3, 3, 7, 7, false);
}
