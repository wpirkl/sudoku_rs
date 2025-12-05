use sudoku::sudoku_factory::{select_random_bit, SudokuFactory}; // Import the function from the parent module
use sudoku::sudoku_fmt::*;

#[test]
fn test_returns_none_for_empty_mask() {
    let mask = 0;
    assert_eq!(select_random_bit(mask), None);
}

#[test]
fn test_returns_exact_index_for_single_bit(){
    // Mask: 000...01000 (only index 3 is set)
    
    // Run it multiple times to ensure it's not accidentally working
    for i in 0..10 as u32 {
        let mask = 1 << i;
        assert_eq!(select_random_bit(mask), Some(i));
    }
}

#[test]
fn test_picks_valid_options_only() {
    // Mask: 101 (Indices 0 and 2 are set. Index 1 is NOT set)

    for _ in 0..50 {
        let mask = rand::random::<u32>();
        let result = select_random_bit(mask).unwrap();

        assert!(result < 32, "Function returned out-of-bounds index: {}", result);

        let result = (1 as u32) << result; // Convert to 0-based index

        let valid = mask & result == result;
        assert!(valid, "Function selected an invalid bit: {:08x} != {:08x}", result, mask);
    }
}

#[test]
fn test_eventually_picks_all_options() {
    // Mask: 11 (Indices 0 and 1 are set)
    let mask = (1 << 0) | (1 << 1);
    
    let mut picked_zero = false;
    let mut picked_one = false;

    // Run enough times to statistically guarantee both are picked
    // Probability of failing 100 times is 1 / 2^100 (basically impossible)
    for _ in 0..100 {
        match select_random_bit(mask) {
            Some(0) => picked_zero = true,
            Some(1) => picked_one = true,
            _ => panic!("Picked invalid bit"),
        }
    }

    assert!(picked_zero, "Failed to pick bit 0 after 100 tries (bad RNG?)");
    assert!(picked_one, "Failed to pick bit 1 after 100 tries (bad RNG?)");
}


#[test]
fn test_sudoku_generation() {

    const N_ROWS: usize = 9;
    const N_COLS: usize = 9;

    let factory = SudokuFactory::<N_ROWS, N_COLS>::new();
    let sudoku = factory.generate();

    println!("Generated Sudoku:\n{}", sudoku);

    // Check that all cells are filled (non-zero)
    for r in 0..N_ROWS {
        for c in 0..N_COLS {
            assert!(sudoku.board[r][c] != 0, "Cell ({}, {}) is empty!", r, c);
        }
    }

    // Check that the Sudoku is valid
    assert!(sudoku.is_valid(), "Generated Sudoku is not valid!");
}
