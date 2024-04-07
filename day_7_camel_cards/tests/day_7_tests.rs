use std::path::Path;

use day_7_camel_cards::camel_cards::calculate_total_winnings;

const PATH_TO_TEST_INPUT: &str = "../day_7_camel_cards/resources/test_input.txt";

#[test]
fn test_calculate_winnings() {
    let path = Path::new(PATH_TO_TEST_INPUT);
    let total_winnings = calculate_total_winnings(path);

    assert_eq!(total_winnings, 5905);
}