use sudoku::sudoku_factory::{self, SudokuFactory}; // Import the function from the parent module
use sudoku::sudoku_fmt::*;

use rand::SeedableRng;
use rand::rngs::StdRng;


#[test]
fn test_sudoku_generation() {

    const N_ROWS: usize = 9;
    const N_COLS: usize = 9;

    let rng = Box::new(StdRng::seed_from_u64(42));

    let mut factory = SudokuFactory::<N_ROWS, N_COLS>::new(rng);
    for n in 0..100 {
        let sudoku = factory.generate();

        println!("Generated Sudoku: {} \n{}", n, sudoku);

        // Check that all cells are filled (non-zero)
        for r in 0..N_ROWS {
            for c in 0..N_COLS {
                assert!(sudoku.board[r][c] != 0, "Cell ({}, {}) is empty!", r, c);
            }
        }

        // Check that the Sudoku is valid
        assert!(sudoku.is_valid(), "Generated Sudoku is not valid!");
    }
}
