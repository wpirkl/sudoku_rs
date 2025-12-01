mod sudoku;
use sudoku::Sudoku;

mod sudoku_iterator;
use sudoku_iterator::SudokuIterator;






fn main() {

    println!("Sudoku Iterator");

    let board = 
        [["5","3",".",".","7",".",".",".","."]
        ,["6",".",".","1","9","5",".",".","."]
        ,[".","9","8",".",".",".",".","6","."]
        ,["8",".",".",".","6",".",".",".","3"]
        ,["4",".",".","8",".","3",".",".","1"]
        ,["7",".",".",".","2",".",".",".","6"]
        ,[".","6",".",".",".",".","2","8","."]
        ,[".",".",".","4","1","9",".",".","5"]
        ,[".",".",".",".","8",".",".","7","9"]];

    // let board = [[""; 12]; 12];

    let board = Sudoku::new(board);

    let is_valid = board.is_valid();

    println!("The board is: {}", is_valid);
    println!("{}", board);


}
