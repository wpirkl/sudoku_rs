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

    pub fn reset(&mut self) {

        let number_of_symbols_mask = ((1 as u32) << ((N_ROWS / 3) * (N_COLS / 3))) - 1;

        for r in 0..N_ROWS {
            for c in 0..N_COLS {
                self.possibilities[r][c] = number_of_symbols_mask;
            }
        }
    }

    pub fn clear(&mut self) {

        for r in 0..N_ROWS {
            for c in 0..N_COLS {
                self.possibilities[r][c] = 0;
            }
        }
    }

    pub fn add_possibility(&mut self, row: usize, col: usize, number: u32) {

        let mask = 1 << (number - 1);
        self.possibilities[row][col] |= mask;
    }

    pub fn remove_possibility(&mut self, row: usize, col: usize, number: u32) {

        let mask = 1 << (number - 1);
        self.possibilities[row][col] &= !mask;
    }

    pub fn set_possibility(&mut self, row: usize, col: usize, mask: u32) {

        self.possibilities[row][col] = mask;
    }

    pub fn clear_possibilities(&mut self, row: usize, col: usize) {

        self.possibilities[row][col] = 0;
    }

    pub fn eliminate_possibility_row(&mut self, row: usize, col: usize, number: u32) {

        for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, SudokuIteratorMode::Row) {

            if r == row && c == col {
                continue;
            }

            self.remove_possibility(r, c, number);
        }
    }

    pub fn eliminate_possibility_col(&mut self, row: usize, col: usize, number: u32) {

        for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, SudokuIteratorMode::Column) {

            if r == row && c == col {
                continue;
            }

            self.remove_possibility(r, c, number);
        }
    }

    pub fn eliminate_possibility_square(&mut self, row: usize, col: usize, number: u32) {

        for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, SudokuIteratorMode::Square) {

            if r == row && c == col {
                continue;
            }

            self.remove_possibility(r, c, number);
        }
    }

    pub fn eliminate_possibility_affected(&mut self, row: usize, col: usize, number: u32) {

        for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, SudokuIteratorMode::Square) {

            if r == row && c == col {
                continue;
            }

            self.remove_possibility(r, c, number);
        }
    }

    pub fn eliminate_possibility(&mut self, row: usize, col: usize, number: u32) {

        self.eliminate_possibility_row(row, col, number);
        self.eliminate_possibility_col(row, col, number);   
        self.eliminate_possibility_square(row, col, number);
    }

    pub fn get_possibilities(&self, row: usize, col: usize) -> u32 {

        self.possibilities[row][col]
    }

    pub fn get_possibility(&self, row: usize, col: usize) -> Option<u32> {

        if 1 == self.count_possibilities(row, col) {
            let trailing = self.possibilities[row][col].trailing_zeros();

            Some(trailing + 1)
        } else {
            None
        }
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
                else if count == 0 {
                    // Contradiction found
                    // return None;
                }
            }
        }

        best_pos
    }


    pub fn handle_naked_pairs(&mut self, row: usize, col: usize, mode: SudokuIteratorMode) {
        
        for (row_idx_a, col_idx_a) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, mode) {

            if self.count_possibilities(row_idx_a, col_idx_a) != 2 {
                continue;
            }
            
            for (row_idx_b, col_idx_b) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, mode) {

                if row_idx_a == row_idx_b && col_idx_a == col_idx_b {
                    continue;
                }

                if self.possibilities[row_idx_a][col_idx_a] == self.possibilities[row_idx_b][col_idx_b] {

                    // Found a naked pair
                    let pair_mask = self.possibilities[row_idx_a][col_idx_a];

                    println!("Found naked pair at ({}, {}) and ({}, {}) with mask {:09b}", row_idx_a, col_idx_a, row_idx_b, col_idx_b, pair_mask);

                    // eliminate these two possibilities from other affected cells in the unit
                    for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, SudokuIteratorMode::Affected) {

                        if (r == row_idx_a && c == col_idx_a) || (r == row_idx_b && c == col_idx_b) {
                            continue;
                        }

                        let before = self.possibilities[r][c];
                        let after = before & (!pair_mask);
                        println!("Eliminating naked pair possibilities at ({}, {}): {:09b} & {:09b} -> {:09b}", r, c, before, pair_mask, after);
                        self.possibilities[r][c] = after;
                    }
                }
            }
        }
    }

    pub fn handle_hidden_pairs(&mut self, row: usize, col: usize, mode: SudokuIteratorMode) {

        let max_number:usize = N_ROWS/3 * N_COLS/3;

        // the index for places is the number 1-9 minus 1
        // it stores a bitmask for each place (0-max_number) where that number is a possibility...
        let mut places: [u32; 32] = [0 as u32; 32];

        // this stores the coordinates of each index of the iterator we check.
        let mut coordinates: [(usize, usize); 32] = [(0, 0); 32];

        for (index, (r, c)) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, mode).enumerate() {

            coordinates[index] = (r, c);
            let cellmask = self.get_possibilities(r, c);

            for possibility in PossibilityIterator::new(cellmask) {

                // println!("Naked Pair Check - Cell ({}, {}) has possibility {}", r, c, possibility);

                places[possibility as usize - 1] |= 1 << index;
            }
        }

        println!("Naked Pair Check - Places: {:09b}, {:09b}", places[0], places[1]);
        println!("Naked Pair Check - Coordinates: {:?}", coordinates);

        for num_a in 0..max_number
        {
            // gets the mask of places for number a
            let places_a = places[num_a];
            if places_a.count_ones() != 2 { continue; }   // if it's not 2 places, skip

            for num_b in (num_a + 1)..max_number
            {
                // gets the mask of places for number b
                let places_b = places[num_b];
                if places_a.count_ones() != 2 { continue; }   // if it's not 2 places, skip

                // if common_places.count_ones() == 2 {
                    // let common_places = places_a & places_b;
                // }

                if places_a == places_b { // if both masks share the same places, we found a naked pair

                    // println!("Found naked pair: numbers {} and {} in positions {:09b}", num_a, num_b, places_a);

                    // get the place index from places_a, which is the same as places_b
                    let mut indices = [(0 as usize, 0 as usize); 2];

                    for (i, coord_index) in PossibilityIterator::new(places_a).enumerate() {
                        indices[i as usize] = coordinates[coord_index as usize - 1];
                    }

                    // println!("Naked pair positions: {:?}", indices);

                    // create a mask to keep only the two numbers in the pair
                    let keep_mask: u32 = (1 << (num_a)) | (1 << (num_b));

                    println!("Naked pair keep mask: {:09b}", keep_mask);

                    for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, SudokuIteratorMode::Affected) {

                        let mut handle = true;
                        for (index_r, index_c) in indices {
                            // println!("Skipping elimination for naked pair cell ({}, {})", index_r, index_c);
                            if r == index_r && c == index_c {
                                handle = false;
                                break;
                            }
                        }

                        if handle {
                            let before = self.possibilities[r][c];
                            let after = (before) & (!keep_mask);

                            println!("Eliminating possibilities ({}, {}) at ({}, {}): {:09b} & {:09b} -> {:09b}", num_a, num_b, r, c, before, keep_mask, after);
                            self.possibilities[r][c] = after;
                        }
                    }
                }
            }
        }
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
    counts: [u32; 32],
    positions: [(usize, usize); 32],
    current: usize
}


impl<const N_ROWS: usize, const N_COLS: usize> HiddenSingleIterator<N_ROWS, N_COLS> {

    pub fn new(pencil_notes: &PencilNotes<N_ROWS, N_COLS>, row: usize, col: usize, mode: SudokuIteratorMode) -> Self {

        let mut iterator = HiddenSingleIterator {
            counts: [0; 32],
            positions: [(N_ROWS, N_COLS); 32],
            current: 0 };

        for (r, c) in SudokuIterator::<N_ROWS, N_COLS>::new(row, col, mode) {
         
            let cell_possibility = pencil_notes.get_possibilities(r, c);
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

        let max_number:usize = N_ROWS/3 * N_COLS/3;

        for number in self.current..max_number {
            if self.counts[number] == 1 {

                let (r, c) = self.positions[number];
                self.current = number + 1;
                return Some((r, c, number as u32 + 1));
            }
        }

        None
    }
}
