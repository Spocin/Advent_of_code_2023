use std::collections::HashMap;
use std::fs;
use std::path::Path;

const PATH_TO_INPUT: &str = "./day_4_scratchcards/resources/input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    let lines: Vec<&str> = input.lines().collect();
    let mut cards_results: HashMap<usize, u8> = HashMap::new();
    let sum: u32 = lines.iter().enumerate().map(|(line_idx, line)| process_card(line, line_idx, &lines, &mut cards_results)).sum();

    println!("Sum of cards: {}", sum as usize + lines.len());
}

fn process_card(line: &&str, line_idx: usize, lines: &Vec<&str>, card_results: &mut HashMap<usize, u8>) -> u32 {
    if !card_results.contains_key(&line_idx) {
        let content_start_idx = &line.find(':').unwrap();

        let content = &line[content_start_idx+1..];

        let sections: [&str; 2] = content
            .split('|')
            .map(|section| section.trim())
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();

        let wining_numbers= parse_section_to_numbers(sections[0]);
        let given_numbers= parse_section_to_numbers(sections[1]);

        let matched_numbers: u8 = wining_numbers
            .iter()
            .map(|wining_number| match given_numbers.contains(&wining_number) {
                true => { 1 }
                false => { 0 }
            })
            .sum();

        card_results.insert(line_idx, matched_numbers);
    }

    let result = card_results.get(&line_idx).unwrap();

    if *result == 0 { return 0 }

    let copies = &lines[line_idx+1..=line_idx+*result as usize];

    let mut sum: u32 = *result as u32;
    for (copy_idx, copy) in copies.iter().enumerate() {
        sum += process_card(copy, line_idx+copy_idx+1, &lines, card_results);
    }

    return sum;
}

fn parse_section_to_numbers(section: &str) -> Vec<u8> {
    return section
        .split(' ')
        .filter(|&number| !number.is_empty())
        .map(|number| number.trim().parse::<u8>().unwrap())
        .collect();
}