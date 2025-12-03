
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

    pub fn add_possibility(&mut self, row: usize, col: usize, number: u8) {

        let mask = 1 << (number - 1);
        self.possibilities[row][col] |= mask;
    }

    pub fn remove_possibility(&mut self, row: usize, col: usize, number: u8) {

        let mask = 1 << (number - 1);
        self.possibilities[row][col] &= !mask;
    }

    pub fn set_possibility(&mut self, row: usize, col: usize, number: u32) {

        let mask = 1 << (number - 1);
        self.possibilities[row][col] = mask;
    }

    pub fn get_possibility(&mut self, row: usize, col: usize) -> u32 {

        self.possibilities[row][col]
    }

    pub fn has_possibility(&self, row: usize, col: usize, number: u8) -> bool {
        
        let mask = 1 << (number - 1);
        (self.possibilities[row][col] & mask) != 0
    }

    pub fn count_possibilities(&self, row: usize, col: usize) -> u32 {

        self.possibilities[row][col].count_ones()
    }

}

