use sudoku::sudoku_pencil_notes::PencilNotes;

#[test]
fn test_naked_pair_removes_candidates() {

    let mut pencil_notes = PencilNotes::<9, 9>::new();

    pencil_notes.clear();

    // Set up two cells with identical candidate pairs
    pencil_notes.set_possibility(0, 0, 0b000000011); // Candidates 1 and 2
    pencil_notes.set_possibility(0, 1, 0b000000011); // Candidates 1 and 2 (this is the naked pair)

    // Set up a third cell in the same unit with additional candidates
    pencil_notes.set_possibility(0, 2, 0b000011100); // Candidates    3, 4, 5
    pencil_notes.set_possibility(0, 5, 0b000101100); // Candidates    3, 4,  , 6 (this is a hidden pair)

    pencil_notes.set_possibility(0, 3, 0b111000001);

    print!("Before: ");
    for i in 0..9 {
        print!("{:09b}, ", pencil_notes.possibilities[0][i]);
    }
    print!("\n");

    // Apply naked pair logic
    pencil_notes.handle_naked_pairs(0, 0, sudoku::sudoku_iterator::SudokuIteratorMode::Column);

    print!("After: ");
    for i in 0..9 {
        print!("{:09b}, ", pencil_notes.possibilities[0][i]);
    }
    print!("\n");

    assert!(false, "Not implemented yet");
}
