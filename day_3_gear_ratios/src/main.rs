use std::fs;
use std::path::Path;

const PATH_TO_INPUT: &str = "./day_3_gear_ratios/resources/test_input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: u64 = 0;

    for (lineIdx, line) in lines.iter().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();

        let mut curr_idx: u32 = 0;
        while curr_idx < line.len() as u32 {
            print!("{}", chars[curr_idx as usize]);

            curr_idx += 1;
        }

        println!();
    }

    println!("Sum of parts {}", sum);
}
