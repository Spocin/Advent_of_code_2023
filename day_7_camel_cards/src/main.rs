use std::path::Path;

use day_7_camel_cards::camel_cards::calculate_total_winnings;

const PATH_TO_INPUT: &str = "/resources/input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let total_winnings = calculate_total_winnings(path);

    println!("Total winnings: {}", total_winnings);
}
