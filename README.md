# chess ♙

![demo](./demo/fools_mate.gif)

A command line recreation of chess™ using rust.

### How to Play
1. Get a second player.
2. Take turns by typing moves in [algebraic notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)).
3. Fight over the keyboard.
4. Type `quit` when you want to exit.

### Customisation
- Colours can be customised in `board.rs` by modifying the ANSI escape codes.
- The board size can be changed in `board.rs` also, but the input parser won't be happy.