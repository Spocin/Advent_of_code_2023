mod day_4;

fn main() {
    let input = day_4::INPUT;

    let sum: u32 = input.lines().map(|line| {
        let content_start_idx = &line
            .find(':')
            .unwrap();

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

            if sum > 1 {
                sum *= 2;
            } else {
                sum += 1;
            }
        }

        return sum;
    }).sum();

    println!("Sum of cards: {}", sum);
}

fn parse_section_to_numbers(section: &str) -> Vec<u8> {
    return section
        .split(' ')
        .filter(|&number| !number.is_empty())
        .map(|number| number.trim().parse::<u8>().unwrap())
        .collect();
}