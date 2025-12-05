use sudoku::sudoku_iterator::*;


#[test]
fn test_sudoku_iterator_square_mode() {
    let mut iterator = SudokuIterator::<9, 9>::new(4, 4, SudokuIteratorMode::Square);

    let mut results = Vec::new();
    while let Some((r, c)) = iterator.next_square() {
        results.push((r, c));
    }

    let expected = vec![
        (3, 3), (3, 4), (3, 5),
        (4, 3), (4, 4), (4, 5),
        (5, 3), (5, 4), (5, 5),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_sudoku_iterator_row_mode() {
    let mut iterator = SudokuIterator::<9, 9>::new(2, 0, SudokuIteratorMode::Row);

    let mut results = Vec::new();
    while let Some((r, c)) = iterator.next_row() {
        results.push((r, c));
    }

    let expected = vec![
        (0, 0), (1, 0), (2, 0),
        (3, 0), (4, 0), (5, 0),
        (6, 0), (7, 0), (8, 0),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_sudoku_iterator_column_mode() { 
    let mut iterator = SudokuIterator::<9, 9>::new(0, 5, SudokuIteratorMode::Column);

    let mut results = Vec::new();
    while let Some((r, c)) = iterator.next_column() {
        results.push((r, c));
    }

    let expected = vec![
        (0, 0), (0, 1), (0, 2),
        (0, 3), (0, 4), (0, 5),
        (0, 6), (0, 7), (0, 8),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_sudoku_iterator_affected_mode() {

    let mut iterator = SudokuIterator::<9, 9>::new(0, 8, SudokuIteratorMode::Affected);

    let mut results = Vec::new();
    while let Some((r, c)) = iterator.next_affected() {
        results.push((r, c));
    }

    let expected = vec![
        // Row 0
        (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8),
        (1, 6), (1, 7), (1, 8),
        (2, 6), (2, 7), (2, 8),
        (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), (8, 8),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_sudoku_iterator_all_mode() {
    let mut iterator = SudokuIterator::<9, 9>::new(0, 0, SudokuIteratorMode::All);

    let mut results = Vec::new();
    while let Some((r, c)) = iterator.next_all() {
        results.push((r, c));
    }

    let mut expected = Vec::new();
    for r in 0..9 {
        for c in 0..9 {
            expected.push((r, c));
        }
    }

    assert_eq!(results, expected);
}