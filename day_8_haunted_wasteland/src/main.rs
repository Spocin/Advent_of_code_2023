use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::map_coordinates::MapCoordinates;

mod map_coordinates;

const PATH_TO_INPUT: &str = "day_8_haunted_wasteland/resources/test_input.txt";

fn main() {
    let path = Path::new(PATH_TO_INPUT);
    let parsed_data = parse_input(path);

    println!("Commands: [{}]", parsed_data.0.len());
    println!("{:?}", parsed_data.0);
    println!();
    println!("Parsed coordinates: [{}]", parsed_data.1.len());
    for coordinate in &parsed_data.1 {
        println!("{}", coordinate.1);
    }

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

pub fn count_steps_though_coordinates(commands: Vec<char>, coordinates: HashMap<String, MapCoordinates>) -> u128 {
    let mut count: u128 = 0;

    let starting_nodes = coordinates
        .iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(_,el)| el)
        .collect::<Vec<_>>();

    /*let mut tmp_coordinate  = "AAA";
    let mut tmp_idx = 0;
    while tmp_coordinate != "ZZZ" {
        let curr_coords = coordinates.get(tmp_coordinate);

        if curr_coords.is_none() { panic!("Can't find such coordinate name: {}", tmp_coordinate) }

        match commands[tmp_idx] {
            'L' => { tmp_coordinate = &curr_coords.unwrap().l_name },
            'R' => {tmp_coordinate = &curr_coords.unwrap().r_name }
            _ => panic!("Unknown command!"), /*TODO Parse commands into enum*/
        }
        
        match tmp_idx {
            x if x < commands.len() - 1 => { tmp_idx += 1; }
            x if x == commands.len() - 1 => { tmp_idx = 0; }
            _ => panic!("Unknown value of tmp_idx")
        }
        
        count += 1;
    }*/

    return count;
}