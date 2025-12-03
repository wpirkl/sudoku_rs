use std::cmp;
use std::fmt;

use crate::sudoku::Sudoku;

impl<const N_ROWS: usize, const N_COLS: usize> fmt::Display for Sudoku<N_ROWS, N_COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        // 1. APPLY YOUR LOGIC
        // We calculate block size by strictly dividing by 3.
        // We use .max(1) to prevent "divide by zero" errors if rows/cols < 3.
        let block_h = (N_ROWS / 3).max(1);
        let block_w = (N_COLS / 3).max(1);

        // 2. Dynamic Padding
        let max_dim = cmp::max(N_ROWS, N_COLS);
        let max_val_width = max_dim.to_string().len();

        // Helper to draw horizontal separators
        let draw_separator = |f: &mut fmt::Formatter| -> fmt::Result {
            write!(f, "+")?;
            for i in 0..N_COLS {
                // Draw dashes for content + 2 spaces padding
                for _ in 0..(max_val_width + 2) { 
                    write!(f, "-")?; 
                }
                
                // Draw a "+" at block intersections based on block_w
                if (i + 1) % block_w == 0 {
                    write!(f, "+")?;
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)
        };

        // --- PRINTING ---

        draw_separator(f)?;

        for (row_idx, row) in self.board.iter().enumerate() {
            write!(f, "|")?;

            for (col_idx, cell) in row.iter().enumerate() {
                let val = if *cell <= 0 { ".".to_string() } else { cell.to_string() };

                write!(f, " {:>width$} ", val, width = max_val_width)?;

                // Vertical separator based on block_w
                if (col_idx + 1) % block_w == 0 {
                    write!(f, "|")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;

            // Horizontal separator based on block_h
            if (row_idx + 1) % block_h == 0 {
                draw_separator(f)?;
            }
        }

        Ok(())
    }
}

