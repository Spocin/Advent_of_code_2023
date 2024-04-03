use std::path::Path;

use day_6_wait_for_it::find_combinations_to_win_race;

const PATH_TO_INPUT: &str = "../day_6_wait_for_it/resources/input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let wining_combinations_count = find_combinations_to_win_race(path);

    println!("Results multiplied: {}", wining_combinations_count);
}