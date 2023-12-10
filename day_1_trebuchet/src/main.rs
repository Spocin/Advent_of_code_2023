use std::fs;
use std::path::Path;

const PATH_TO_INPUT: &str = "./day_1_trebuchet/resources/input_part_two.txt";

const DIGITS_SPELLED: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn main() {
    println!("Loading file from {}", PATH_TO_INPUT);

    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    let sum: u32 = input.lines()
        .map(|line| {
            let chars = line.chars();

            let mut a: char = '0';
            'outer: for (idx, char) in chars.clone().enumerate() {
                if char.is_digit(10) {
                    a = char;
                    break 'outer;
                }

                let slice = &line[idx..];
                for (spelled, letter) in DIGITS_SPELLED {
                    if spelled.len() > slice.len() { break; }

                    let slice_exact = &slice[..spelled.len()];
                    if spelled.eq(slice_exact) {
                        a = letter;
                        break 'outer;
                    }
                }
            }

            let mut b: char = '0';
            'outer: for (idx, char) in chars.rev().enumerate() {
                if char.is_digit(10) {
                    b = char;
                    break 'outer;
                }

                let slice: &str = &line.chars().rev().collect::<String>()[idx..];
                for (spelled, letter) in DIGITS_SPELLED {
                    let spelled_reversed: String = spelled.chars().rev().collect();
                    if spelled_reversed.len() > slice.len() { break; }

                    let slice_exact = &slice[..spelled_reversed.len()];

                    if spelled_reversed.eq(slice_exact) {
                        b = letter;
                        break 'outer;
                    }
                }
            }

            return format!("{}{}", a, b).parse::<u32>().unwrap();
        })
        .sum();

    println!("Sum of line: {}", sum);
}
