use std::fs;
use std::path::Path;
use std::str::from_utf8;

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
            if chars[curr_idx as usize] != '*' {
                curr_idx += 1;
                continue;
            }

            sum += walk_around_gear(
                &lines,
                line_idx,
                curr_idx as usize,
            );

            //+2 can be added here as we already checked it while walking around gear
            curr_idx += 2;
        }
    }

    println!("Sum of parts {}", sum);
}

fn walk_around_gear(
    lines: &Vec<&str>,
    line_idx: usize,
    gear_idx: usize,
) -> u64 {
    let num_reaches_start_of_line = gear_idx == 0;
    let num_reaches_end_of_line = gear_idx == lines[line_idx].len() - 1;

    let check_start_idx = if !num_reaches_start_of_line { gear_idx - 1 } else { gear_idx };
    let check_end_idx = if !num_reaches_end_of_line { gear_idx + 1 } else { gear_idx };

    let mut numbers: Vec<&[u8]> = vec![];

    //Above
    if line_idx != 0 {
        let line = lines[line_idx - 1].as_bytes();

        println!("Above");
        numbers.append(&mut find_numbers(line, check_start_idx, check_end_idx));

        if numbers.len() > 2 { return 0; }
    }

    //Right
    if !num_reaches_end_of_line {
        let line = &lines[line_idx].as_bytes()[gear_idx + 1..];

        println!("Right");
        numbers.append(&mut find_numbers(line, gear_idx + 1, line.len() - 1));

        if numbers.len() > 2 { return 0; }
    }

    //Below
    if line_idx != lines.len() - 1 {
        let line = lines[line_idx + 1].as_bytes();

        println!("Below");
        numbers.append(&mut find_numbers(line, check_start_idx, check_end_idx));

        if numbers.len() > 2 { return 0; }
    }

    //Left
    if !num_reaches_start_of_line {
        let line = &lines[line_idx].as_bytes()[..= gear_idx - 1];

        println!("Left");
        numbers.append(&mut find_numbers(line, 0, gear_idx - 1));
    }

    if numbers.len() != 2 { return 0; }

    //Gear has 2 numbers around it. Multiply them.
    return numbers
        .iter()
        .map(|bytes| from_utf8(bytes).unwrap())
        .map(|string| string.parse::<u64>().unwrap())
        .fold(0, |acc, num| acc * num);
}

fn find_numbers(line: &[u8], start_idx: usize, end_idx: usize) -> Vec<&[u8]> {
    println!("Line: {:?}", from_utf8(line).unwrap());
    println!("StartIdx: {}", start_idx);
    println!("EndIdx: {}", end_idx);
    println!();

    /* TODO
    Implement method that will search for digit between start_idx and end_idx.
    When encountered search for digits both ways in the given line.
    */

    return vec![];
}