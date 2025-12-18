use rand::rngs::StdRng;
use rand::SeedableRng;

use sudoku::sudoku_pencil_notes::{PencilNotes, RandomBit};

#[test]
fn test_naked_pair_removes_candidates() {

    let mut pencil_notes = PencilNotes::<9, 9>::new();

    pencil_notes.clear();

    // Set up two cells with identical candidate pairs
    pencil_notes.set_possibility(0, 0, 0b000000011); // Candidates 1 and 2
    pencil_notes.set_possibility(0, 1, 0b000000011); // Candidates 1 and 2 (this is the naked pair)

    // Set up a third cell in the same unit with additional candidates
    pencil_notes.set_possibility(0, 2, 0b000011100); // Candidates    3, 4, 5
    pencil_notes.set_possibility(0, 3, 0b111000001); // Naked pair removal would eliminate 1 here
    pencil_notes.set_possibility(0, 5, 0b000101100); // Candidates    3, 4,  , 6 (this is a hidden pair)


/* 
    print!("Before: ");
    for i in 0..9 {
        print!("{:09b}, ", pencil_notes.possibilities[0][i]);
    }
    print!("\n");
 */
    // Apply naked pair logic
    pencil_notes.handle_naked_pairs(0, 0, sudoku::sudoku_iterator::SudokuIteratorMode::Column);

/*
    print!("After: ");
    for i in 0..9 {
        print!("{:09b}, ", pencil_notes.possibilities[0][i]);
    }
    print!("\n");
*/

    // Verify that candidates 1 and 2 have been removed from other cells in the unit
    assert_eq!(pencil_notes.get_possibilities(0, 0), 0b000000011);
    assert_eq!(pencil_notes.get_possibilities(0, 1), 0b000000011);
    assert_eq!(pencil_notes.get_possibilities(0, 2), 0b000011100);
    assert_eq!(pencil_notes.get_possibilities(0, 3), 0b111000000); // 1 and 2 removed
    assert_eq!(pencil_notes.get_possibilities(0, 5), 0b000101100);

}



#[test]
fn test_hidden_pair_removes_candidates() {

    let mut pencil_notes = PencilNotes::<9, 9>::new();

    pencil_notes.clear();

    // Set up two cells with identical candidate pairs
    pencil_notes.set_possibility(0, 0, 0b000000011); // Candidates 1 and 2
    pencil_notes.set_possibility(0, 1, 0b000000011); // Candidates 1 and 2 (this is the naked pair)

    // Set up a third cell in the same unit with additional candidates
    pencil_notes.set_possibility(0, 2, 0b000011100); // Candidates    3, 4, 5    (3, 4 is hidden pair)
    pencil_notes.set_possibility(0, 3, 0b111000001); // naked pair removal would eliminate 1 here
    pencil_notes.set_possibility(0, 5, 0b000101100); // Candidates    3, 4,  , 6 (3, 4 is a hidden pair)



    print!("Before: ");
    for i in 0..9 {
        print!("{:09b}, ", pencil_notes.possibilities[0][i]);
    }
    print!("\n");

    // Apply naked pair logic
    pencil_notes.handle_hidden_pairs(0, 0, sudoku::sudoku_iterator::SudokuIteratorMode::Column);


    print!("After: ");
    for i in 0..9 {
        print!("{:09b}, ", pencil_notes.possibilities[0][i]);
    }
    print!("\n");


    // Verify that candidates 1 and 2 have been removed from other cells in the unit
    assert_eq!(pencil_notes.get_possibilities(0, 0), 0b000000011);
    assert_eq!(pencil_notes.get_possibilities(0, 1), 0b000000011);
    assert_eq!(pencil_notes.get_possibilities(0, 2), 0b000001100);
    assert_eq!(pencil_notes.get_possibilities(0, 3), 0b111000001); // 1 and 2 removed
    assert_eq!(pencil_notes.get_possibilities(0, 5), 0b000001100);

}






#[test]
fn test_returns_none_for_empty_mask() {

    let mut random_bit = RandomBit::new(Box::new(StdRng::seed_from_u64(42)));
    let mask = 0;
    assert_eq!(random_bit.select_random_bit(mask), None);
}

#[test]
fn test_returns_exact_index_for_single_bit(){
    // Mask: 000...01000 (only index 3 is set)
    let mut random_bit = RandomBit::new(Box::new(StdRng::seed_from_u64(42)));
    
    // Run it multiple times to ensure it's not accidentally working
    for i in 0..10 as u32 {
        let mask = 1 << i;
        assert_eq!(random_bit.select_random_bit(mask), Some(i));
    }
}

#[test]
fn test_picks_valid_options_only() {
    // Mask: 101 (Indices 0 and 2 are set. Index 1 is NOT set)
    let mut random_bit = RandomBit::new(Box::new(StdRng::seed_from_u64(42)));
    
    for _ in 0..50 {
        let mask = rand::random::<u32>();
        let result = random_bit.select_random_bit(mask).unwrap();

        assert!(result < 32, "Function returned out-of-bounds index: {}", result);

        let result = (1 as u32) << result; // Convert to 0-based index

        let valid = mask & result == result;
        assert!(valid, "Function selected an invalid bit: {:08x} != {:08x}", result, mask);
    }
}

#[test]
fn test_eventually_picks_all_options() {

    let mut random_bit = RandomBit::new(Box::new(StdRng::seed_from_u64(42)));
    
    // Mask: 11 (Indices 0 and 1 are set)
    let mask = (1 << 0) | (1 << 1);
    
    let mut picked_zero = false;
    let mut picked_one = false;

    // Run enough times to statistically guarantee both are picked
    // Probability of failing 100 times is 1 / 2^100 (basically impossible)
    for _ in 0..100 {
        match random_bit.select_random_bit(mask) {
            Some(0) => picked_zero = true,
            Some(1) => picked_one = true,
            _ => panic!("Picked invalid bit"),
        }
        if picked_one && picked_zero {
            break;
        }
    }

    assert!(picked_zero, "Failed to pick bit 0 after 100 tries (bad RNG?)");
    assert!(picked_one, "Failed to pick bit 1 after 100 tries (bad RNG?)");
}



#[test]
fn test_select_random()
{
    let mut random_bit= RandomBit::new(Box::new(StdRng::seed_from_u64(42)));

    let mut mask:u32 = (1u32 << 9) - 1;

    for _ in 0..9
    {
        if let Some(number) = random_bit.select_random_bit(mask) {

            let old_mask = mask;
            let selected = 1u32 << number;
            mask = mask & !selected;
            println!("selected from mask: 0b{:09b} & !0b{:09b} ({}) -> 0b{:09b}", old_mask, selected, number, mask);

        } else {
            println!("Could not select from mask 0b{:09b}", mask);
        }
    }
}