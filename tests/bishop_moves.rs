use chess::board::Board;
use chess::pieces::Moves;
use chess::Coordinate;

/// sets up board used for all tests
fn test_board() -> Board {
    let mut board = Board::empty();
    board.place_piece(3, 3, "♗", true);
    board.place_piece(6, 6, "♗", true);
    board.place_piece(0, 0, "♗", false);
    return board;
}

/// Tests if the piece at (x, y) can move to the position
fn test_move(x: usize, y: usize, position: Coordinate, expected: bool) {
    let board = test_board();
    let moves = Moves::Bishop;
    match &board.grid[y][x] {
        Some(piece) => assert_eq!(moves.can_move(&piece, position, &board), expected),
        None => assert!(false),
    }
}

#[test]
fn northeast_diagonal() {
    test_move(3, 3, Coordinate { x: 1, y: 5 }, true);
}

#[test]
fn northwest_diagonal() {
    test_move(3, 3, Coordinate { x: 5, y: 5 }, true);
}

#[test]
fn southeast_diagonal() {
    test_move(3, 3, Coordinate { x: 1, y: 1 }, true);
}

#[test]
fn southwest_diagonal() {
    test_move(3, 3, Coordinate { x: 5, y: 1 }, true);
}

#[test]
fn out_of_bounds() {
    test_move(3, 3, Coordinate { x: 8, y: 8 }, false);
}

#[test]
fn vertical() {
    test_move(3, 3, Coordinate { x: 3, y: 5 }, false);
}

#[test]
fn horizontal() {
    test_move(3, 3, Coordinate { x: 1, y: 3 }, false);
}

#[test]
fn capture() {
    test_move(3, 3, Coordinate { x: 0, y: 0 }, true);
}

#[test]
fn occupied() {
    test_move(3, 3, Coordinate { x: 6, y: 6 }, false);
}

#[test]
fn blocked() {
    test_move(3, 3, Coordinate { x: 7, y: 7 }, false);
}
