use crate::sudoku_iterator::{SudokuIterator, SudokuIteratorMode};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PencilNotes<const N_ROWS: usize, const N_COLS: usize>
{
    pub possibilities: [[u32; N_COLS]; N_ROWS],
}


impl<const N_ROWS: usize, const N_COLS: usize> PencilNotes<N_ROWS, N_COLS> {

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

        let number_of_symbols_mask = ((1 as u32) << ((N_ROWS / 3) * (N_COLS / 3))) - 1;

        PencilNotes { possibilities: [[number_of_symbols_mask; N_COLS]; N_ROWS] }
    }

    pub fn add_possibility(&mut self, row: usize, col: usize, number: u32) {

        let mask = 1 << (number - 1);
        self.possibilities[row][col] |= mask;
    }

    pub fn remove_possibility(&mut self, row: usize, col: usize, number: u32) {

        let mask = 1 << (number - 1);
        self.possibilities[row][col] &= !mask;
    }

    pub fn set_possibility(&mut self, row: usize, col: usize, number: u32) {

        let mask = 1 << (number - 1);
        self.possibilities[row][col] = mask;
    }

    pub fn clear_possibilities(&mut self, row: usize, col: usize) {

        self.possibilities[row][col] = 0;
    }

    pub fn eliminate_possibility(&mut self, row: usize, col: usize, number: u32) {

        for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, SudokuIteratorMode::Affected) {

            if r == row && c == col {
                continue;
            }

            self.remove_possibility(r, c, number);
        }
    }

    pub fn get_possibility(&self, row: usize, col: usize) -> u32 {

        self.possibilities[row][col]
    }

    pub fn has_possibility(&self, row: usize, col: usize, number: u32) -> bool {
        
        let mask = 1 << (number - 1);
        (self.possibilities[row][col] & mask) != 0
    }

    pub fn count_possibilities(&self, row: usize, col: usize) -> u32 {

        self.possibilities[row][col].count_ones()
    }

    pub fn find_lowest_entropy_cell(&self) -> Option<(usize, usize)> {

        let mut min_count = u32::MAX;
        let mut best_pos = None;

        for r in 0..N_ROWS {
            for c in 0..N_COLS {

                // We need to cast Store to u32 to use count_ones logic generically
                // Assuming you added a helper or Into<u32> to your trait

                let count = self.count_possibilities(r,c);

                // 0 means contradiction (impossible), 1 means solved.
                // We only care about cells with > 1 possibilities.
                if count > 1 {
                    if count < min_count {
                        min_count = count;
                        best_pos = Some((r, c));
                        
                        // Optimization: If we find a cell with 2 possibilities,
                        // it's impossible to beat (since 1 is solved). Return immediately.
                        if min_count == 2 {
                            return best_pos;
                        }
                    }
                }
                else
                {
                    // assert!(count != 0, "Contradiction found at cell ({}, {})", r, c);
                }
            }
        }

        best_pos
    }

}


pub struct PossibilityIterator {
    mask: u32
}

impl PossibilityIterator {
    pub fn new(mask: u32) -> Self {
        PossibilityIterator { mask }
    }
}

impl Iterator for PossibilityIterator {
    type Item = u32;

    // This iterator yields the indices (1-based) of set bits in the mask.

    fn next(&mut self) -> Option<Self::Item> {

        while self.mask > 0 {
            let trailing = self.mask.trailing_zeros();
            self.mask &= self.mask - 1;
            return Some(trailing + 1);
        }
        None
    }
}


pub struct HiddenSingleIterator<const N_ROWS: usize, const N_COLS: usize> {
    counts: [u32; 9],
    positions: [(usize, usize); 9],
    current: usize
}


impl<const N_ROWS: usize, const N_COLS: usize> HiddenSingleIterator<N_ROWS, N_COLS> {

    pub fn new(pencil_notes: &PencilNotes<N_ROWS, N_COLS>, row: usize, col: usize, mode: SudokuIteratorMode) -> Self {

        let mut iterator = HiddenSingleIterator {
            counts: [0; 9],
            positions: [(N_ROWS, N_COLS); 9],
            current: 0 };

        for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, mode) {
         
            let cell_possibility = pencil_notes.get_possibility(r, c);
            for possibility in PossibilityIterator::new(cell_possibility) {
        
                iterator.counts[possibility as usize - 1] += 1;
                iterator.positions[possibility as usize - 1] = (r, c);
            }
        }

        iterator
    }
}


impl<const N_ROWS: usize, const N_COLS: usize> Iterator for HiddenSingleIterator<N_ROWS, N_COLS> {

    type Item = (usize, usize, u32);

    fn next(&mut self) -> Option<Self::Item> {

        for number in self.current..9 {
            if self.counts[number] == 1 {

                let (r, c) = self.positions[number];
                self.current = number + 1;
                return Some((r, c, number as u32 + 1));
            }
        }

        None
    }
}
