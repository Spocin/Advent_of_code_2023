use std::path::Path;

use day_7_camel_cards::camel_cards::calculate_wining_cards;

const PATH_TO_TEST_INPUT: &str = "/resources/test_input.txt";

#[test]
fn test_calculate_winnings() {
    let path = Path::new(PATH_TO_TEST_INPUT);
    let total_winnings = calculate_wining_cards(path);

    assert_eq!(total_winnings, 6440);
}