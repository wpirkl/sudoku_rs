mod sudoku;
mod sudoku_iterator;
mod sudoku_fmt;
mod sudoku_pencil_notes;

use sudoku::Sudoku;
use sudoku_pencil_notes::PencilNotes;


fn main() {

    println!("Sudoku Iterator");

    let board = 
        [[5,3,0,0,7,0,0,0,0]
        ,[6,0,0,1,9,5,0,0,0]
        ,[0,9,8,0,0,0,0,6,0]
        ,[8,0,0,0,6,0,0,0,3]
        ,[4,0,0,8,0,3,0,0,1]
        ,[7,0,0,0,2,0,0,0,6]
        ,[0,6,0,0,0,0,2,8,0]
        ,[0,0,0,4,1,9,0,0,5]
        ,[0,0,0,0,8,0,0,7,9]];

    // let board = [[0; 12]; 12];

    let board = Sudoku::<9,9>::new(board);

    let is_valid = board.is_valid();

    println!("The board is: {}", is_valid);
    println!("{}", board);

    let mut pencil_notes = PencilNotes::<9,9>::new();

    println!("Pencil Notes: {:?}", pencil_notes);

}
