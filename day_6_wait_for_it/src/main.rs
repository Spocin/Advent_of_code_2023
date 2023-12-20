use std::fs;
use std::path::Path;

const PATH_TO_INPUT: &str = "./day_6_wait_for_it/resources/test_input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");


}
