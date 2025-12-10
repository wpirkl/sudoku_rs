use std::cmp;
use std::fmt;

use crate::sudoku_pencil_notes::PencilNotes;

impl<const N_ROWS: usize, const N_COLS: usize> fmt::Display for PencilNotes<N_ROWS, N_COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        // 1. DIMENSION CALCULATIONS
        // Same logic as your template: Blocks are derived by dividing by 3.
        let block_h = (N_ROWS / 3).max(1); // e.g., 3 for 9x9
        let block_w = (N_COLS / 3).max(1); // e.g., 3 for 9x9
        
        // Calculate max number width (e.g. "9" is 1 char, "16" is 2 chars)
        let max_dim = cmp::max(N_ROWS, N_COLS);
        let max_val_width = max_dim.to_string().len();

        // 2. CALCULATE CELL WIDTH
        // A single cell contains 'block_w' numbers horizontally.
        // Example 9x9: Cell looks like " 1 2 3 ".
        // Width = (block_w * number_width) + (block_w spaces) + 1 extra space
        let cell_text_width = (block_w * max_val_width) + block_w + 1;

        // 3. SEPARATOR HELPER
        // Draws: +-------+-------+
        let draw_separator = |f: &mut fmt::Formatter| -> fmt::Result {
            write!(f, "+")?;
            for i in 0..N_COLS {
                // Draw dashes matching the text width of the cell
                for _ in 0..cell_text_width { 
                    write!(f, "-")?; 
                }
                
                // Block Intersection
                if (i + 1) % block_w == 0 {
                    write!(f, "+")?;
                } else {
                    write!(f, "-")?; // Simple column divider
                }
            }
            writeln!(f)
        };

        // --- PRINTING LOOP ---

        draw_separator(f)?;

        for (row_idx, row_data) in self.possibilities.iter().enumerate() {
            
            // KEY CHANGE: The "Sub-Row" Loop
            // Since a single cell is a grid, we must iterate through the 
            // height of that mini-grid (e.g., 3 lines of text per sudoku row).
            for sub_r in 0..block_h {
                
                write!(f, "|")?; // Start of the text line

                for (col_idx, mask) in row_data.iter().enumerate() {
                    
                    // Start of the cell padding
                    write!(f, " ")?;

                    // Iterate through the width of the mini-grid
                    for sub_c in 0..block_w {
                        // Calculate which number belongs at this (x,y) in the mini-grid
                        // Math: (row * width) + col + 1
                        let number = (sub_r * block_w) + sub_c + 1;
                        
                        // Check if this number is available in the bitmask
                        let is_possible = (mask & (1 << (number - 1))) != 0;

                        if is_possible {
                            // Print number padded (e.g., " 5" or "12")
                            write!(f, "{:>width$}", number, width = max_val_width)?;
                        } else {
                            // Print dot placeholder
                            write!(f, "{:>width$}", ".", width = max_val_width)?;
                        }

                        // Space between numbers inside the mini-grid
                        if sub_c < block_w - 1 {
                            write!(f, " ")?;
                        }
                    }

                    // End of cell padding
                    write!(f, " ")?;

                    // Vertical Separator (End of Cell)
                    if (col_idx + 1) % block_w == 0 {
                        write!(f, "|")?; // Thick border
                    } else {
                        write!(f, "|")?; // Thin border
                    }
                }
                
                // End of this text line
                writeln!(f)?;
            }

            // Horizontal Separator (End of Sudoku Row)
            if (row_idx + 1) % block_h == 0 {
                draw_separator(f)?;
            } else {
                // If we are between rows but not at a block boundary, 
                // we print a thinner separator line for readability
                write!(f, "+")?;
                for i in 0..N_COLS {
                    for _ in 0..cell_text_width { write!(f, "-")?; }
                    if (i + 1) % block_w == 0 { write!(f, "+")?; } else { write!(f, "+")?; }
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
