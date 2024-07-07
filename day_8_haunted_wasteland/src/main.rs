use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::map_coordinates::MapCoordinates;

mod map_coordinates;

const PATH_TO_INPUT: &str = "day_8_haunted_wasteland/resources/test_input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let parsed_data = parse_input(path);

    println!("Commands: [{}]\n{}", parsed_data.0.len(), parsed_data.0);
    println!();
    println!("Parsed coordinates: [{}]", parsed_data.1.len());
    for coordinate in parsed_data.1 {
        println!("{}", coordinate.1);
    }
}

fn parse_input(path_to_input: &Path) -> (String, HashMap<String, MapCoordinates>) {
    let input = fs::read_to_string(path_to_input);

    match input {
        Err(err) => panic!("Something wrong with the path! {:?}", err),
        Ok(input) => {
            let lines = input.lines().collect::<Vec<&str>>();

            let commands: String = lines[0].into();

            if let Some(inv_chars) = check_commands_syntax(&commands) {
                panic!("Command contains invalid characters: {:?}", inv_chars);
            }

            let coordinates_map = lines[2..].iter()
                .fold(HashMap::new(), |mut acc, &el| {
                    let map_coordinates = MapCoordinates::new(el);

                    acc.insert(map_coordinates.name.clone(), map_coordinates);

                    return acc;
                });

            return (commands, coordinates_map);
        }
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