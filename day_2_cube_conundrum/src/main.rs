use std::fs;
use std::path::Path;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

const PATH_TO_INPUT: &str = "C:\\Users\\Spocin\\IdeaProjects\\Advent_of_code_2023\\day_2_cube_conundrum\\resources\\input.txt";

fn main() {
    println!("Loading file from {}", PATH_TO_INPUT);

    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    let sum: u32 = input.lines()
        .map(|line| {
            let sections: Vec<&str> = line.split(":").collect();
            let draws: Vec<&str> = sections[1].split(";").map(|draw| draw.trim()).collect();

            for draw in draws {
                let results: Vec<&str> = draw.split(",").map(|result| result.trim()).collect();

                for result in results {
                    let amount = result.split(" ").collect::<Vec<&str>>()[0];
                    let color = result.split(" ").collect::<Vec<&str>>()[1];

                    match color {
                        "red" => if amount.parse::<u32>().unwrap() > MAX_RED { return 0; },
                        "green" => if amount.parse::<u32>().unwrap() > MAX_GREEN { return 0; },
                        "blue" => if amount.parse::<u32>().unwrap() > MAX_BLUE { return 0; },
                        _ => panic!("Unknown color!")
                    }
                }
            }

            let game_num = sections[0].strip_prefix("Game ").unwrap();
            return game_num.parse::<u32>().unwrap();
        })
        .sum();

    println!("Sum of games: {}", sum);
}
