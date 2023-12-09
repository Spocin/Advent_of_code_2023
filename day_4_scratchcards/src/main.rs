mod day_4;

fn main() {
    let input = day_4::TEST_INPUT;

    let lines: Vec<&str> = input.lines().collect();
    let sum: u32 = lines.iter().enumerate().map(|(line_idx, line)| process_card(line, line_idx, &lines)).sum();

    println!("Sum of cards: {}", sum as usize + lines.len());
}

fn process_card(line: &&str, line_idx: usize, lines: &Vec<&str>) -> u32 {
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

    let mut sum: u32 = 0;
    for wining_number in wining_numbers {
        if !given_numbers.contains(&wining_number) { continue; }
        sum += 1;
    }

    if sum == 0 { return 0 };

    let copies = &lines[line_idx+1..=line_idx+sum as usize];

    println!("Copies for {}", line);
    copies.iter().for_each(|copy| println!("\t{}", copy));

    for (copy_idx, copy) in copies.iter().enumerate() {
        sum += process_card(copy, line_idx+copy_idx+1, &lines);
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