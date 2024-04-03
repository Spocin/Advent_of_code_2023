use std::path::Path;

use day_7_camel_cards::camel_cards::calculate_wining_cards;

const PATH_TO_INPUT: &str = "/resources/input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let total_winnings = calculate_wining_cards(path);

    println!("Total winnings: {}", total_winnings);
}
