# chess ♙

![demo](./demo/fools_mate.gif)

A command line recreation of chess™ using Rust.

### How to Play
1. Get a second player.
2. Take turns by typing moves in [algebraic notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)).
3. Fight over the keyboard.
4. Type `quit` when you want to exit.

### Randomisation
[*Chess960*](https://en.wikipedia.org/wiki/Fischer_random_chess) or *Fischer Random Chess* is a variation of chess with a randomised starting position. It follows 2 rules:

    1. Bishops must be on opposite coloured squares
    2. The king must be between the 2 rooks.

Pawns will be stay in their normal starting positions.

### Customisation
- Colours can be customised in `board.rs` by modifying the ANSI escape codes.
- The board size can be changed in `board.rs` also, but the input parser won't be happy.