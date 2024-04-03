use std::fs;
use std::path::Path;

#[derive(Debug)]
struct RaceDescription {
    race_length: u64,
    distance_to_beat: u64,
}

pub fn find_combinations_to_win_race(path_to_input: &Path) -> i32 {
    let input = fs::read_to_string(path_to_input).expect("Something wrong with path!");

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

    return wining_combinations_count;
}