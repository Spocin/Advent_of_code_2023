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
            if !chars[curr_idx as usize].is_numeric() {
                curr_idx += 1;
                continue;
            }

            //Walk forward to check where num ends.
            let found_num_as_string = build_number(&chars[curr_idx as usize..chars.len()]);
            println!("Found num: {}", found_num_as_string);

            // 1. Search for symbol around num
                // 1.1. Parse string as u32
                // 1.2. Add to sum

            //+1 can be added here as we already checked it while building string
            curr_idx += found_num_as_string.len() as u32 + 1;
        }
    }

    println!("Sum of parts {}", sum);
}

fn build_number(line: &[char]) -> String {
    let mut num = String::from(line[0]);

    for char in &line[1..line.len()] {
        if char.is_numeric() {
            num.push(*char);
        } else {
            break;
        }
    }

    return num;
}
