use super::test_move;
use crate::board::Board;

#[test]
fn straight() {
    let board = Board::from_vec(&vec![(3, 3, '♖', true)]);
    test_move(&board, 3, 3, 0, 3, true);
    test_move(&board, 3, 3, 7, 3, true);
    test_move(&board, 3, 3, 3, 0, true);
    test_move(&board, 3, 3, 3, 7, true);
}

#[test]
fn invalid() {
    let board = Board::from_vec(&vec![(3, 3, '♖', true)]);
    test_move(&board, 3, 3, 4, 4, false);
    test_move(&board, 3, 3, 8, 8, false);
}

#[test]
fn capture() {
    let board = Board::from_vec(&vec![(3, 3, '♖', true), (0, 3, '♖', false)]);
    test_move(&board, 3, 3, 0, 3, true);
}

#[test]
fn blocked() {
    let board = Board::from_vec(&vec![(3, 3, '♖', true), (5, 3, '♖', true)]);
    test_move(&board, 3, 3, 5, 3, false);
    test_move(&board, 3, 3, 7, 3, false);
}
