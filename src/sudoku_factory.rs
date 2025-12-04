use crate::sudoku::Sudoku;
use crate::sudoku_iterator::SudokuIterator;
use crate::sudoku_pencil_notes::PencilNotes;
use crate::sudoku_fmt::*;

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


        for r in 0..N_ROWS {
            for c in 0..N_COLS {
                // get the possibilities for this cell
                let possibilities = pencil_notes.get_possibility(r, c);

                // select one possibility at random
                if let Some(selected_bit) = select_random_bit(possibilities) {

                    let number = selected_bit;
                    sudoku.board[r][c] = number;
                    pencil_notes.set_possibility(r, c, number);

                    for (it_r, it_c) in SudokuIterator::<N_ROWS, N_COLS>::new(r, c) {

                        // println!("Removing possibility {:?} from cell ({}, {})", number, it_r, it_c);
                        pencil_notes.remove_possibility(it_r, it_c, number);
                    }

                    // println!("({}, {}) => {} -- Possibilities: {:?}", r, c, number, pencil_notes);
                    println!("{}", sudoku);
                }
                else
                {
                    assert!(false, "No possibilities left for cell ({}, {}, {:?})", r, c, pencil_notes);
                }
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
        1 => Some(mask.trailing_zeros()+1),
        _ => {

            let target_index = rand::random_range(0..number_of_ones);

            for _ in 0..target_index {
                mask &= mask - 1;
            }

            Some(mask.trailing_zeros()+1)
        }
    }
}
