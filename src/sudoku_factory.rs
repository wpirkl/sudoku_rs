use crate::sudoku::Sudoku;
use crate::sudoku_iterator::{SudokuIterator, SudokuIteratorMode};
use crate::sudoku_pencil_notes::{HiddenSingleIterator, PencilNotes};
use crate::sudoku_fmt::*;

use std::thread;
use std::time::Duration;

const SLEEP_DURATION_SECS: Duration = Duration::from_millis(1);

pub struct SudokuFactory<const N_ROWS: usize, const N_COLS: usize>();

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

    pub fn new() -> Self
    {
        let _ = Self::CHECK_CONSTRAINTS;

        SudokuFactory {}
    }

    pub fn generate(&self) -> Sudoku<N_ROWS, N_COLS>
    {
        // Placeholder implementation
        let mut sudoku = Sudoku { board: [[0; N_COLS]; N_ROWS] };
        let mut pencil_notes = PencilNotes::<N_ROWS, N_COLS>::new();

        for iterations in 0..(N_ROWS * N_COLS) {

            let mut found_hidden_singles = false;

            for row in 0..N_ROWS/3 {
                for col in 0..N_COLS/3 {
                    if let Some((r,c, possibility)) = HiddenSingleIterator::<N_ROWS, N_COLS>::new(&pencil_notes, row*3, col*3, SudokuIteratorMode::Square).next() {

                        // println!("Hidden single square found at ({}, {}) with possibility {}", r, c, possibility);

                        pencil_notes.clear_possibilities(r, c);
                        sudoku.board[r][c] = possibility;
                        pencil_notes.eliminate_possibility(r, c, possibility);

                        thread::sleep(SLEEP_DURATION_SECS);
                        found_hidden_singles = true;
                    }
                }
            }

            for row in 0..N_ROWS {
                if let Some((r,c, possibility)) = HiddenSingleIterator::<N_ROWS, N_COLS>::new(&pencil_notes, row, 0, SudokuIteratorMode::Column).next() {

                    // println!("Hidden single column found at ({}, {}) with possibility {}", r, c, possibility);

                    pencil_notes.clear_possibilities(r, c);
                    sudoku.board[r][c] = possibility;
                    pencil_notes.eliminate_possibility(r, c, possibility);

                    thread::sleep(SLEEP_DURATION_SECS);
                    found_hidden_singles = true;
                }
            }

            for col in 0..N_COLS {
                if let Some((r,c, possibility)) = HiddenSingleIterator::<N_ROWS, N_COLS>::new(&pencil_notes, 0, col, SudokuIteratorMode::Row).next() {

                    // println!("Hidden single row found at ({}, {}) with possibility {}", r, c, possibility);

                    pencil_notes.clear_possibilities(r, c);
                    sudoku.board[r][c] = possibility;
                    pencil_notes.eliminate_possibility(r, c, possibility);

                    thread::sleep(SLEEP_DURATION_SECS);
                    found_hidden_singles = true;
                }
            }

            if !found_hidden_singles {

                if let Some((row, col)) = pencil_notes.find_lowest_entropy_cell() {

                    let mask = pencil_notes.get_possibility(row, col);
                    if let Some(selected_bit) = select_random_bit(mask)
                    {
                        let number = selected_bit + 1;

                        // println!("Filling cell ({}, {}) with number {}", row, col, number);

                        sudoku.board[row][col] = number;
                        pencil_notes.clear_possibilities(row, col);
                        pencil_notes.eliminate_possibility(row, col, number);

                        thread::sleep(SLEEP_DURATION_SECS);
                    }
                    else
                    {
                        // assert!(false, "No valid options left for cell ({}, {})", row, col);
                    }
                }
            }

            // println!("Current Sudoku State:\n{}", sudoku);

            assert!(sudoku.is_valid(), "Sudoku state is invalid!");

            if sudoku.is_complete() {
                println!("Sudoku generation complete after {} iterations!", iterations + 1);
                break;
            }

        }

        sudoku
    }

}


pub fn select_random_bit(bitfield: u32) -> Option<u32> {

    let number_of_ones = bitfield.count_ones();
    let mut mask = bitfield; 

    match number_of_ones {
        0 => None,
        1 => Some(mask.trailing_zeros()),
        _ => {

            let target_index = rand::random_range(0..number_of_ones);

            for _ in 0..target_index {
                mask &= mask - 1;
            }

            Some(mask.trailing_zeros())
        }
    }
}
