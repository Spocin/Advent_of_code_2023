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

        resolve_search(&mut numbers, line, check_start_idx, check_end_idx);

        if numbers.len() > 2 { return 0; }
    }

    //Right
    if !num_reaches_end_of_line {
        let line = &lines[line_idx].as_bytes()[gear_idx + 1..];

        resolve_search(&mut numbers, line, 0, 0);

        if numbers.len() > 2 { return 0; }
    }

    //Below
    if line_idx != lines.len() - 1 {
        let line = lines[line_idx + 1].as_bytes();

        resolve_search(&mut numbers, line, check_start_idx, check_end_idx);

        if numbers.len() > 2 { return 0; }
    }

    //Left
    if !num_reaches_start_of_line {
        let line = &lines[line_idx].as_bytes()[..= gear_idx - 1];

        resolve_search(&mut numbers, line, gear_idx - 1, gear_idx - 1);
    }

    if numbers.len() != 2 { return 0; }

    //Gear has 2 numbers around it. Multiply them.
    return numbers
        .iter()
        .map(|bytes| from_utf8(bytes).unwrap())
        .map(|string| string.parse::<u64>().unwrap())
        .fold(1, |acc, num| acc * num);
}

fn resolve_search<'a>(numbers: &mut Vec<&'a [u8]>, line: &'a [u8], start_idx: usize, end_idx: usize) {
    //Search only right from start_idx
    //Ex. 123....
    //    *......
    if start_idx == 0 {
        match (line[start_idx].is_ascii_digit(), search_for_digit(&line[1..])) {
            (true, Some(v)) => numbers.push(&line[start_idx..=end_idx + v]),
            (false, Some(v)) => numbers.push(&line[1..=end_idx + v]),
            (true, None) => numbers.push(&line[start_idx..=start_idx]),
            (_, _) => {}
        }
        return;
    }

    //Search only left from end_idx
    //Ex. ....123
    //    ......*
    if end_idx == line.len() - 1 {
        match (line[end_idx].is_ascii_digit(), search_for_digit_reversed(&line[..end_idx - 1])) {
            (true, Some(v)) => numbers.push(&line[start_idx - v..]),
            (false, Some(v)) => numbers.push(&line[start_idx - v..end_idx - 1]),
            (true, None) => numbers.push(&line[end_idx..=end_idx]),
            (_,_) => {}
        }
        return;
    }

    //When char at the same idx as gear is not a digit
    //1. number can start at the left diagonal and go left
    //2. number can start at the right diagonal and go right
    //Ex. .123.123.
    //    ....*....
    if !line[start_idx + 1].is_ascii_digit() {
        match search_for_digit(&line[end_idx..]) {
            Some(v) => numbers.push(&line[end_idx..=end_idx + v]),
            _ => {},
        }

        match search_for_digit_reversed(&line[..=start_idx]) {
            Some(v) => numbers.push(&line[v..=start_idx]),
            _ => {},
        }
        return;
    }

    //When char at the same idx as gear is a digit we need to search from idx of a gear both ways
    //Ex. ...123...
    //    ....*....
    if line[start_idx + 1].is_ascii_digit() {
        let mut l_ptr = start_idx;
        let mut r_ptr = end_idx;

        while line[l_ptr].is_ascii_digit() {
            l_ptr -= 1;
        }

        while line[r_ptr].is_ascii_digit() {
            r_ptr += 1;
        }

        match (l_ptr == start_idx, r_ptr == end_idx) {
            (true, true) => numbers.push(&line[start_idx + 1..=end_idx - 1]),
            (true, false) => numbers.push(&line[start_idx + 1..r_ptr]),
            (false, true) => numbers.push(&line[l_ptr + 1..=end_idx - 1]),
            (false, false) => numbers.push(&line[l_ptr + 1..=r_ptr])
        }
    }
}

fn search_for_digit(line: &[u8]) -> Option<usize> {
    if !line[0].is_ascii_digit() { return None; }

    for (idx, char) in line[1..].iter().enumerate() {
        if !char.is_ascii_digit() { return Some(idx); }
    }

    return Some(line.len() - 1);
}

fn search_for_digit_reversed(line: &[u8]) -> Option<usize> {
    if !line[line.len() - 1].is_ascii_digit() { return None; }

    for (idx, char) in line[..=line.len() - 1].iter().enumerate().rev() {
        if !char.is_ascii_digit() { return Some(idx + 1)}
    }

    return Some(0);
}