use std::{fs, thread};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;
use std::time::Duration;

use crate::map_coordinates::MapCoordinates;

mod map_coordinates;

const PATH_TO_INPUT: &str = "day_8_haunted_wasteland/resources/input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let parsed_data = parse_input(path);

    println!("Commands: [{}]", parsed_data.0.len());
    /*println!("{:?}", parsed_data.0);
    println!();
    println!("Parsed coordinates: [{}]", parsed_data.1.len());
    for coordinate in &parsed_data.1 {
        println!("{}", coordinate.1);
    }*/

    let steps = count_steps_though_coordinates(parsed_data.0, parsed_data.1);
    println!("Steps required to go through map: {}", steps);
}

pub fn parse_input(path_to_input: &Path) -> (Vec<char>, HashMap<String, MapCoordinates>) {
    let input = fs::read_to_string(path_to_input);

    match input {
        Err(err) => panic!("Something wrong with the path! {:?}", err),
        Ok(input) => {
            let lines = input.lines().collect::<Vec<&str>>();

            if let Some(inv_chars) = check_commands_syntax(&lines[0]) {
                panic!("Command contains invalid characters: {:?}", inv_chars);
            }

            let commands = lines[0].chars().collect::<Vec<char>>();

            let coordinates_map = lines[2..].iter()
                .fold(HashMap::new(), |mut acc, &el| {
                    let map_coordinates = MapCoordinates::new(el);

                    acc.insert(map_coordinates.name.clone(), map_coordinates);

                    return acc;
                });

            return (commands, coordinates_map);
        }
    }

    fn check_commands_syntax(commands: &str) -> Option<Vec<char>> {
        const ALLOWED_CHARS: [char; 2] = [
            'R',
            'L'
        ];

        let invalid_chars = commands
            .chars()
            .fold(vec![], |mut acc, el| {
                if ALLOWED_CHARS.iter().any(|invalid_el| &el == invalid_el) {
                    return acc;
                }

                acc.push(el);

                return acc;
            });

        if invalid_chars.is_empty() {
            return None;
        }

        return Some(invalid_chars);
    }
}

pub fn count_steps_though_coordinates(commands: Vec<char>, coordinates: HashMap<String, MapCoordinates>) -> u64 {
    let count = Arc::new(AtomicU64::new(0));
    let finished = Arc::new(AtomicBool::new(false));

    let count_read = count.clone();
    let finished_read = finished.clone();
    let logger_thread = thread::spawn(move || {
        while !finished_read.load(Relaxed) {
            println!("Current count: {}", count_read.load(Relaxed));

            sleep(Duration::from_secs(1));
        }
    });

    let mut starting_nodes = coordinates
        .iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(_,el)| el)
        .collect::<Vec<_>>();

    let mut tmp_idx = 0;

    while !are_all_nodes_end_with(&starting_nodes, 'Z') {
        for node in starting_nodes.iter_mut() {
            match commands[tmp_idx] {
                'L' => { *node = coordinates.get(&node.l_name).unwrap() },
                'R' => { *node = coordinates.get(&node.r_name).unwrap() }
                _ => panic!("Unknown command!"), /*TODO Parse commands into enum*/
            }
        }

        match tmp_idx {
            x if x < commands.len() - 1 => { tmp_idx += 1; }
            x if x == commands.len() - 1 => { tmp_idx = 0; }
            _ => panic!("Unknown value of tmp_idx")
        }

        count.fetch_add(1, Relaxed);
    }

    finished.fetch_and(true, Relaxed);

    logger_thread.join().unwrap();

    return count.load(Relaxed);
}

fn are_all_nodes_end_with(nodes: &Vec<&MapCoordinates>, end_char: char) -> bool {
    return nodes.iter().all(|el| el.name.ends_with(end_char));
}