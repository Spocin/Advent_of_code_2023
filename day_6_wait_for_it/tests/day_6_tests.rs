use std::path::Path;

use day_6_wait_for_it::find_combinations_to_win_race;

const PATH_TO_TEST_INPUT: &str = "../day_6_wait_for_it/resources/test_input.txt";

#[test]
pub fn run_against_test_input() {
    let path = Path::new(PATH_TO_TEST_INPUT);
    println!("{:?}", path);
    assert_eq!(find_combinations_to_win_race(path), 71503);
}