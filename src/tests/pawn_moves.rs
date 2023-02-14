use crate::board::Board;
use crate::pieces::MoveChecker;
use crate::Coordinate;

/// sets up board used for all tests
fn test_board() -> Board {
    let mut board = Board::empty();
    board.place_piece(0, 1, "♙", true);
    board.place_piece(1, 2, "♙", true);
    board.place_piece(0, 6, "♙", false);
    board.place_piece(1, 5, "♙", false);
    board.place_piece(3, 1, "♙", true);
    board.place_piece(2, 2, "♙", false);
    board.place_piece(4, 2, "♙", false);
    board.place_piece(5, 1, "♙", false);
    board.place_piece(5, 2, "♙", false);
    board.place_piece(6, 1, "♙", false);
    board.place_piece(6, 3, "♙", false);
    return board;
}

/// Tests if the piece at (x, y) can move to the position
fn test_move(x: usize, y: usize, position: Coordinate, expected: bool) {
    let board = test_board();
    let moves = MoveChecker::Pawn;
    match &board.grid[y][x] {
        Some(piece) => assert_eq!(moves.can_move(&piece, position, &board), expected),
        None => assert!(false),
    }
}

#[test]
fn single_forward_white() {
    test_move(0, 1, Coordinate { x: 0, y: 2 }, true);
    test_move(1, 2, Coordinate { x: 1, y: 3 }, true);
}

#[test]
fn double_forward_white() {
    // only pawns that haven't moved at all can move 2 squares
    test_move(0, 1, Coordinate { x: 0, y: 3 }, true);
    test_move(1, 2, Coordinate { x: 1, y: 4 }, false);
}

#[test]
fn single_forward_black() {
    test_move(0, 6, Coordinate { x: 0, y: 5 }, true);
    test_move(1, 5, Coordinate { x: 1, y: 4 }, true);
}

#[test]
fn double_forward_black() {
    test_move(0, 6, Coordinate { x: 0, y: 4 }, true);
    test_move(1, 5, Coordinate { x: 1, y: 3 }, false);
}

#[test]
fn horizontal() {
    test_move(0, 1, Coordinate { x: 1, y: 1 }, false);
}

#[test]
fn diagonal() {
    test_move(1, 2, Coordinate { x: 0, y: 3 }, false);
}

#[test]
fn capture() {
    test_move(3, 1, Coordinate { x: 2, y: 2 }, true);
    test_move(3, 1, Coordinate { x: 4, y: 2 }, true);
}

#[test]
fn capture_friendly() {
    test_move(0, 1, Coordinate { x: 1, y: 2 }, false);
}

#[test]
fn occupied() {
    test_move(5, 1, Coordinate { x: 5, y: 2 }, false);
    test_move(6, 1, Coordinate { x: 6, y: 3 }, false);
}

#[test]
fn blocked() {
    test_move(5, 1, Coordinate { x: 5, y: 3 }, false);
}
