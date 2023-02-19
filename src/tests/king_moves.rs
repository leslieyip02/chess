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
    let mut board = Board::from_vec(&vec![
        (4, 0, '♔', true),
        (0, 0, '♖', true),
        (7, 0, '♖', true),
    ]);
    match MoveChecker::castle(&board, false, true) {
        Some((king_x, rook_x)) => {
            assert_eq!(king_x, 4);
            assert_eq!(rook_x, 0)
        }
        None => assert!(false),
    };
    match MoveChecker::castle(&board, true, true) {
        Some((king_x, rook_x)) => {
            assert_eq!(king_x, 4);
            assert_eq!(rook_x, 7)
        }
        None => assert!(false),
    };

    board.grid[0][4].as_mut().unwrap().moves += 1;
    match MoveChecker::castle(&board, false, true) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
    match MoveChecker::castle(&board, true, true) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
}

#[test]
fn castle_in_check() {
    let board = Board::from_vec(&vec![
        (4, 0, '♔', true),
        (0, 0, '♖', true),
        (3, 7, '♖', false),
    ]);
    match MoveChecker::castle(&board, false, true) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
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
    match MoveChecker::castle(&board, false, true) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
    match MoveChecker::castle(&board, true, true) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
}

#[test]
fn castle_960() {
    let board = Board::from_vec(&vec![
        (1, 0, '♔', true),
        (0, 0, '♖', true),
        (4, 0, '♖', true),
    ]);
    match MoveChecker::castle(&board, false, true) {
        Some((king_x, rook_x)) => {
            assert_eq!(king_x, 1);
            assert_eq!(rook_x, 0)
        }
        None => assert!(false),
    };
    match MoveChecker::castle(&board, true, true) {
        Some((king_x, rook_x)) => {
            assert_eq!(king_x, 1);
            assert_eq!(rook_x, 4)
        }
        None => assert!(false),
    };
}

#[test]
fn checkmate() {
    let mut board = Board::new();
    board.make_move("f3", true);
    board.make_move("e6", false);
    board.make_move("g4", true);
    board.make_move("Qh4", false);
    assert!(MoveChecker::checkmate(&board, true));
}
