use std::fs;
use std::path::Path;

const PATH_TO_INPUT: &str = "./day_6_wait_for_it/resources/input.txt";

#[derive(Debug)]
struct RaceDescription {
    race_length: u64,
    distance_to_beat: u64,
}

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    /* Split by whitespace. Parse to u32 */
    let races_parsed: [u64; 2] = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();

    let race_descriptions = RaceDescription {
        race_length: races_parsed[0],
        distance_to_beat: races_parsed[1],
    };

    let mut wining_combinations_count = 0;
    for button_hold_time in 0..race_descriptions.race_length {
        if (button_hold_time * (race_descriptions.race_length - button_hold_time)) > race_descriptions.distance_to_beat {
            wining_combinations_count += 1;
        }
    }

    println!("Results multiplied: {}", wining_combinations_count);
}
