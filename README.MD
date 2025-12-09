# Sudoku

A Rust implementation of a Sudoku puzzle solver and generator.

## Example

Actually this is a bad example as the generation returns invalid cells (6, 6) and (8, 5).

```
+-----------+-----------+-----------+
| 2   6   3 | 7   9   5 | 8   1   4 |
| 9   1   7 | 4   3   8 | 5   2   6 |
| 5   4   8 | 2   1   6 | 9   7   3 |
+-----------+-----------+-----------+
| 6   2   4 | 9   8   3 | 1   5   7 |
| 1   3   9 | 5   4   7 | 6   8   2 |
| 8   7   5 | 1   6   2 | 4   3   9 |
+-----------+-----------+-----------+
| 3   5   2 | 8   7   4 | .   6   1 |
| 7   9   6 | 3   5   1 | 2   4   8 |
| 4   8   1 | 6   2   . | 7   9   5 |
+-----------+-----------+-----------+
```

## Features

- Input validation
- Sudoku iterator
- Pencil notes stored in bitmask

## TODO

- Backpropagate changes on pencil notes
- Sudoku generation
- Unit testing
- Sudoku solving

## Installation

Clone the repository and build with Cargo:

```bash
git clone <repository-url>
cd sudoku
cargo build --release
```

## Usage

Run the program:

```bash
cargo run --release
```

## Project Structure

- `src/main.rs` - Entry point
- 

## Requirements

- Rust 1.56+
- Cargo

## License

MIT

## Contributing

Pull requests are welcome. For major changes, please open an issue first.