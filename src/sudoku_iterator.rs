
#[derive(Debug)]
pub enum SudokuIteratorMode
{
    Row,
    Column,
    Square,
    Affected,
    All
}


#[derive(Debug)]
pub struct SudokuIterator<const N_ROWS: usize, const N_COLS: usize>
{
    row: usize,
    col: usize,

    c_r: usize,
    c_c: usize,

    sq_r: usize,
    sq_c: usize,

    mode: SudokuIteratorMode,
}


impl<const N_ROWS: usize, const N_COLS: usize> SudokuIterator<N_ROWS, N_COLS> {

    pub fn new(row: usize, col: usize, mode: SudokuIteratorMode) -> Self {

        SudokuIterator { row: row, col: col, c_r: 0, c_c: 0, sq_r: (row/3)*3, sq_c: (col/3)*3, mode: mode }
    }

    pub fn next_row(&mut self) -> Option<(usize, usize)>
    {
        if self.c_r >= N_ROWS
        {
            return None;
        }

        let result = (self.c_r, self.col);

        self.c_r += 1;

        Some(result)
    }

    pub fn next_column(&mut self) -> Option<(usize, usize)>
    {
        if self.c_c >= N_COLS
        {
            return None;
        }

        let result = (self.row, self.c_c);

        self.c_c += 1;

        Some(result)
    }

    pub fn next_square(&mut self) -> Option<(usize, usize)>
    {
        if self.c_r >= 3
        {
            return None;
        }

        let result = (self.sq_r + self.c_r, self.sq_c + self.c_c);

        self.c_c += 1;
        if self.c_c >= 3
        {
            self.c_c = 0;
            self.c_r += 1;
        }

        Some(result)
    }

    pub fn next_affected(&mut self) -> Option<(usize, usize)>
    {
        if self.c_r >= N_ROWS
        {
            // we're out of range
            return None;
        }

        let mut n_r = self.c_r;
        let mut n_c = self.c_c;
        let mut search = true;
        
        while N_ROWS > self.c_r && search
        {
            while N_COLS > self.c_c && search
            {
                // println!("c_r: {:?}, c_c: {:?}", self.c_r, self.c_c);
                if self.sq_r <= self.c_r && self.sq_r + 3 > self.c_r &&
                   self.sq_c <= self.c_c && self.sq_c + 3 > self.c_c
                {
                    n_r = self.c_r;
                    n_c = self.c_c;

                    search = false;
                }
                // check if we are in the current col or current row
                else if self.c_r == self.row || self.c_c == self.col
                {
                    n_r = self.c_r;
                    n_c = self.c_c;

                    search = false;
                }
                // skip this cell
                else
                {
                    search = true;
                }

                // increase column
                self.c_c = self.c_c + 1;

            }

            if N_COLS <= self.c_c
            {
                self.c_c = 0;
                self.c_r = self.c_r + 1;
            }
        }

        if search
        {
            // we didn't find anything
            return None;
        }

        // found the cell we need to return
        Some((n_r, n_c))

    }

    pub fn next_all(&mut self) -> Option<(usize, usize)>
    {
        if self.c_r >= N_ROWS
        {
            return None;
        }

        let result = (self.c_r, self.c_c);

        self.c_c += 1;
        if self.c_c >= N_COLS
        {
            self.c_r += 1;
            self.c_c = 0;
        }

        Some(result)
    }

}


impl<const N_ROWS: usize, const N_COLS: usize> Iterator for SudokuIterator<N_ROWS, N_COLS>
{
    type Item = (usize, usize);

    // for row 0, col 0:
    // x o o o o o o o o  | . . . . o . . . .
    // o o o . . . . . .  | . . . . o . . . .
    // o o o . . . . . .  | . . . . o . . . .
    // o . . . . . . . .  | . . . o o o . . .
    // o . . . . . . . .  | o o o o x o o o o
    // o . . . . . . . .  | . . . o o o . . .
    // o . . . . . . . .  | . . . . o . . . .
    // o . . . . . . . .  | . . . . o . . . .
    // o . . . . . . . .  | . . . . o . . . .

    // this means, we'll descend row by row
    // if it's the row of the constructor, return all the indices
    // if it's the column of the constructor, return the column indice
    // if the row and col are inside of the 3x3 square, return row & col

    fn next(&mut self) -> Option<Self::Item> {

        match self.mode {
            SudokuIteratorMode::Row => self.next_row(),
            SudokuIteratorMode::Column => self.next_column(),
            SudokuIteratorMode::Square => self.next_square(),
            SudokuIteratorMode::Affected => self.next_affected(),
            SudokuIteratorMode::All => self.next_all(),
        }
    }
}
