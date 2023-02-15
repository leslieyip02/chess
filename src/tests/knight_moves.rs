use super::test_move;
use crate::board::Board;

#[test]
fn l_shape() {
    let board = Board::from_vec(&vec![(3, 3, "♘", true)]);
    test_move(&board, 3, 3, 4, 5, true);
    test_move(&board, 3, 3, 5, 4, true);
    test_move(&board, 3, 3, 5, 2, true);
    test_move(&board, 3, 3, 4, 1, true);
    test_move(&board, 3, 3, 2, 1, true);
    test_move(&board, 3, 3, 1, 2, true);
    test_move(&board, 3, 3, 1, 4, true);
    test_move(&board, 3, 3, 2, 5, true);
}

#[test]
fn invalid() {
    let board = Board::from_vec(&vec![(7, 7, "♘", true)]);
    test_move(&board, 7, 7, 8, 9, false);
    test_move(&board, 7, 7, 7, 5, false);
    test_move(&board, 7, 7, 5, 5, false);
}

#[test]
fn capture() {
    let board = Board::from_vec(&vec![(3, 3, "♘", true), (4, 5, "♘", false)]);
    test_move(&board, 3, 3, 4, 5, true);
}

#[test]
fn blocked() {
    let board = Board::from_vec(&vec![(3, 3, "♘", true), (4, 5, "♘", true)]);
    test_move(&board, 3, 3, 4, 5, false);
}

#[test]
fn skip_over() {
    let board = Board::from_vec(&vec![
        (3, 3, "♘", true),
        (2, 4, "♘", true),
        (3, 4, "♘", true),
        (4, 4, "♘", true),
        (2, 2, "♘", false),
        (3, 2, "♘", false),
        (4, 2, "♘", false),
    ]);
    test_move(&board, 3, 3, 2, 5, true);
    test_move(&board, 3, 3, 4, 1, true);
}
