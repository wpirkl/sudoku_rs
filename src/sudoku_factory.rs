use crate::sudoku::Sudoku;
use crate::sudoku_iterator::SudokuIteratorMode;
use crate::sudoku_pencil_notes::{HiddenSingleIterator, PencilNotes};
use rand::{Rng, RngCore};

use crate::sudoku_fmt::*;
use crate::sudoku_pencil_notes_fmt::*;

use std::thread;
use std::time::Duration;

// const SLEEP_DURATION_SECS: Duration = Duration::from_millis(0);
const SLEEP_DURATION_SECS: Duration = Duration::from_secs(1);

pub struct SudokuFactory<const N_ROWS: usize, const N_COLS: usize>
{
    pub rng: Box<dyn RngCore>
}

impl<const N_ROWS: usize, const N_COLS: usize> SudokuFactory<N_ROWS, N_COLS> {

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

    pub fn new(rng: Box<dyn RngCore>) -> Self
    {
        let _ = Self::CHECK_CONSTRAINTS;

        SudokuFactory { rng: rng }
    }

    pub fn generate(&mut self) -> Sudoku<N_ROWS, N_COLS>
    {
        // Placeholder implementation
        let mut sudoku = Sudoku { board: [[0; N_COLS]; N_ROWS] };
        let mut pencil_notes = PencilNotes::<N_ROWS, N_COLS>::new();

        for iterations in 0..(N_ROWS * N_COLS) {

            for row in 0..N_ROWS {
                for col in 0..N_COLS {
                    if sudoku.board[row][col] != 0 {
                        continue;
                    }

                    if let Some(number) = pencil_notes.get_possibility(row, col) {

                        println!("{}: Naked single found at ({}, {}) with possibility {}", iterations+1, row, col, number);

                        sudoku.board[row][col] = number;
                        pencil_notes.set_possibility(row, col, number);
                        pencil_notes.eliminate_possibility(row, col, number);

                        // thread::sleep(SLEEP_DURATION_SECS);
                    }
                }
            }

            // handle naked and hidden pairs
            for cnt in 0..N_ROWS {
                // pencil_notes.handle_naked_pairs(cnt, 0, SudokuIteratorMode::Row);
                // pencil_notes.handle_naked_pairs(0, cnt, SudokuIteratorMode::Column);
                // pencil_notes.handle_hidden_pairs(cnt, 0, SudokuIteratorMode::Row);
                // pencil_notes.handle_hidden_pairs(0, cnt, SudokuIteratorMode::Column);
            }

            for row in 0..N_ROWS/3 {
                for col in 0..N_COLS/3 {
                    // pencil_notes.handle_naked_pairs(row*3, col*3, SudokuIteratorMode::Square);
                    // pencil_notes.handle_hidden_pairs(row*3, col*3, SudokuIteratorMode::Square);
                }
            }
            let mut found_hidden_singles = false;

            for row in 0..N_ROWS/3 {
                for col in 0..N_COLS/3 {
                    for (r,c, possibility) in HiddenSingleIterator::<N_ROWS, N_COLS>::new(&pencil_notes, row*3, col*3, SudokuIteratorMode::Square) {
                        
                        if sudoku.board[r][c] == 0 {
                            println!("{}: Hidden single square found at ({}, {}) with possibility {}", iterations+1, r, c, possibility);
                            sudoku.board[r][c] = possibility;
                            pencil_notes.set_possibility(r, c, possibility);
                            pencil_notes.eliminate_possibility(r, c, possibility);

                            // thread::sleep(SLEEP_DURATION_SECS);
                            found_hidden_singles = true;
                            break;
                        }
                    }
                }
            }

            for row in 0..N_ROWS {
                for (r,c, possibility) in HiddenSingleIterator::<N_ROWS, N_COLS>::new(&pencil_notes, row, 0, SudokuIteratorMode::Column) {

                    if sudoku.board[r][c] == 0 {
                        println!("{}: Hidden single column found at ({}, {}) with possibility {}", iterations+1, r, c, possibility);
                        sudoku.board[r][c] = possibility;
                        pencil_notes.set_possibility(r, c, possibility);
                        pencil_notes.eliminate_possibility(r, c, possibility);

                        // thread::sleep(SLEEP_DURATION_SECS);
                        found_hidden_singles = true;

                        break;
                    }
                }
            }

            for col in 0..N_COLS {
                for(r,c, possibility) in HiddenSingleIterator::<N_ROWS, N_COLS>::new(&pencil_notes, 0, col, SudokuIteratorMode::Row) {

                    if sudoku.board[r][c] == 0 {
                        println!("{}: Hidden single row found at ({}, {}) with possibility {}", iterations+1, r, c, possibility);
                        sudoku.board[r][c] = possibility;
                        pencil_notes.set_possibility(r, c, possibility);
                        pencil_notes.eliminate_possibility(r, c, possibility);

                        // thread::sleep(SLEEP_DURATION_SECS);
                        found_hidden_singles = true;

                        break;
                    }
                }
            }

            if !found_hidden_singles {

                if let Some((row, col)) = pencil_notes.find_highest_entropy_cell() {

                    if sudoku.board[row][col] == 0 {

                        let mask = pencil_notes.get_possibilities(row, col);
                        if let Some(selected_bit) = self.select_random_bit(mask)
                        {
                            let number = selected_bit + 1;
                            
                            if sudoku.board[row][col] == 0 {
                                
                                println!("{}: Filling cell ({}, {}) with random number {}", iterations+1, row, col, number);
                                sudoku.board[row][col] = number;
                                pencil_notes.set_possibility(row, col, number);
                                pencil_notes.eliminate_possibility(row, col, number);
                                // break;
                            }
                            // thread::sleep(SLEEP_DURATION_SECS);
                        }
                        else
                        {
                            // assert!(false, "No valid options left for cell ({}, {})", row, col);
                        }
                    }
                }
            }

            println!("\nCurrent Sudoku State:\n{}", sudoku);
            println!("Current Pencil Notes State:\n{}", pencil_notes);
            thread::sleep(SLEEP_DURATION_SECS);

            let valid = sudoku.is_valid();
            if !valid {
                println!("Sudoku state is invalid!\n{}", sudoku);
                // assert!(false, "Sudoku state is invalid!");
                break;
            }

            if sudoku.is_complete() {
                // println!("Sudoku generation complete after {} iterations!", iterations + 1);
                break;
            }

        }

        sudoku
    }

    pub fn select_random_bit(&mut self,bitfield: u32) -> Option<u32> {
    
        let number_of_ones = bitfield.count_ones();
        let mut mask = bitfield; 
    
        match number_of_ones {
            0 => None,
            1 => Some(mask.trailing_zeros()),
            _ => {
    
                let target_index = self.rng.random_range(0..number_of_ones);
    
                for _ in 0..target_index {
                    mask &= mask - 1;
                }
    
                Some(mask.trailing_zeros())
            }
        }
    }

}


