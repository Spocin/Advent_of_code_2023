use std::fs;
use std::path::Path;

const PATH_TO_INPUT: &str = "./day_3_gear_ratios/resources/test_input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: u64 = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();

        let mut curr_idx: u32 = 0;
        while curr_idx < line.len() as u32 {
            if !chars[curr_idx as usize].is_numeric() {
                curr_idx += 1;
                continue;
            }

            //Walk forward to check where num ends.
            let found_num_as_string = build_number(&chars[curr_idx as usize..chars.len()]);

            if does_symbol_exists_around_num(
                &lines,
                line_idx,
                curr_idx as usize,
                (curr_idx + (found_num_as_string.len() as u32 - 1)) as usize,
            ) {
                sum += found_num_as_string.parse::<u32>().unwrap() as u64;
            }

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

fn does_symbol_exists_around_num(
    lines: &Vec<&str>,
    line_idx: usize,
    num_start_idx: usize,
    num_end_idx: usize,
) -> bool {
    let num_reaches_start_of_line = num_start_idx == 0;
    let num_reaches_end_of_line = num_end_idx == lines[line_idx].len() - 1;

    //Above
    if line_idx != 0 {
        //Top left
        if !num_reaches_start_of_line {
            if is_symbol(&lines[line_idx - 1].as_bytes()[num_start_idx - 1]) { return true; }
        }

        //Top
        for char in &lines[line_idx - 1].as_bytes()[num_start_idx..=num_end_idx] {
            if is_symbol(char) { return true; }
        }

        //Top right
        if !num_reaches_end_of_line {
            if is_symbol(&lines[line_idx - 1].as_bytes()[num_end_idx + 1]) { return true; }
        }
    }

    //Right
    if !num_reaches_end_of_line {
        if is_symbol(&lines[line_idx].as_bytes()[num_end_idx + 1]) { return true; }
    }


    //Below
    if line_idx != lines.len() {
        //Bottom right
        if !num_reaches_end_of_line {
            if is_symbol(&lines[line_idx + 1].as_bytes()[num_end_idx + 1]) { return true; }
        }

        //Bottom
        for char in &lines[line_idx + 1].as_bytes()[num_start_idx..=num_end_idx] {
            if is_symbol(char) { return true; }
        }

        //Bottom left
        if !num_reaches_start_of_line {
            if is_symbol(&lines[line_idx + 1].as_bytes()[num_start_idx - 1]) { return true; }
        }
    }

    //Left
    if !num_reaches_start_of_line {
        if is_symbol(&lines[line_idx].as_bytes()[num_start_idx - 1]) { return true; }
    }

    return false;
}

fn is_symbol(symbol: &u8) -> bool {
    return !symbol.is_ascii_digit() && *symbol != b'.'
}