use std::fs;
use std::path::Path;

const PATH_TO_INPUT: &str = "./day_6_wait_for_it/resources/input.txt";

#[derive(Debug)]
struct RaceDescription {
    race_length: u32,
    distance_to_beat: u32,
}

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    /* Split by whitespace. Parse to u32 */
    let races_parsed: [Vec<u32>; 2] = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
        .try_into()
        .unwrap();

    let mut race_descriptions: Vec<RaceDescription> = vec![];
    for el_idx in 0..races_parsed[0].len() {
        race_descriptions.push(RaceDescription {
            race_length: races_parsed[0][el_idx],
            distance_to_beat: races_parsed[1][el_idx],
        });
    }

    let wining_combinations_for_race = race_descriptions
        .iter()
        .map(|race| {
            let mut wining_combinations_count = 0;
            for button_hold_time in 0..race.race_length {
                if (button_hold_time * (race.race_length - button_hold_time)) > race.distance_to_beat {
                    wining_combinations_count += 1;
                }
            }
            return wining_combinations_count;
        })
        .collect::<Vec<u32>>();

    let results_multiplied = wining_combinations_for_race.iter().fold(1, |acc, el| acc * el);

    println!("Races results: {:?}", wining_combinations_for_race);
    println!("Results multiplied: {}", results_multiplied);
}
