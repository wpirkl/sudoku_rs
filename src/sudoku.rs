
use crate::sudoku_iterator::{SudokuIterator, SudokuIteratorMode};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sudoku<const N_ROWS: usize, const N_COLS: usize>
{
    pub board: [[u32; N_COLS]; N_ROWS],
}


impl<const N_ROWS: usize, const N_COLS: usize> Sudoku<N_ROWS, N_COLS> {

    const CHECK_CONSTRAINTS: () = {
        // Constraint 1: Minimum Size
        assert!(N_ROWS > 3, "N_ROWS must be greater than 3");
        assert!(N_COLS > 3, "N_COLS must be greater than 3");

        // Constraint 2: Integer Size
        let block_h = N_ROWS / 3;
        let block_w = N_COLS / 3;
        let symbols_needed = block_h * block_w;

        assert!(
            symbols_needed <= u32::BITS as usize, 
            "The chosen type u32 is too small for this grid size!"
        );
    };

    pub fn new(board: [[u32; N_COLS]; N_ROWS]) -> Self
    {
        let _ = Self::CHECK_CONSTRAINTS;

        Sudoku { board: board.clone() }
    }

    pub fn is_complete(&self) -> bool
    {
        for r in 0..N_ROWS {
            for c in 0..N_COLS {
                if self.board[r][c] == 0
                {
                    return false;
                }
            }
        }

        true
    }
    
    pub fn is_valid(&self) -> bool
    {
        for r in 0..N_ROWS {
            let row = self.board[r];

            for c in 0..N_COLS {
                let cell_value = row[c];
                if cell_value == 0
                {
                    continue;
                }

                for (c_r, c_c) in SudokuIterator::<9, 9>::new(r, c, SudokuIteratorMode::Affected)
                {
                    if c_r == r && c_c == c
                    {
                        continue;
                    }

                    let check_value = self.board[c_r][c_c];
                    if check_value == cell_value
                    {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn max_number(&self) -> u32 {

        (N_ROWS / 3 * N_COLS / 3) as u32
    }

}

