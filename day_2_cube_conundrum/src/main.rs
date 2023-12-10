use std::fs;
use std::path::Path;

const PATH_TO_INPUT: &str = "./day_2_cube_conundrum/resources/input.txt";

fn main() {
    println!("Loading file from {}", PATH_TO_INPUT);

    let path = Path::new(PATH_TO_INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    let sum: u32 = input.lines()
        .map(|line| {
            let sections: Vec<&str> = line.split(":").collect();
            let draws: Vec<&str> = sections[1].split(";").map(|draw| draw.trim()).collect();

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for draw in draws {
                let results: Vec<&str> = draw.split(",").map(|result| result.trim()).collect();

                for result in results {
                    let amount = result.split(" ").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
                    let color = result.split(" ").collect::<Vec<&str>>()[1];

                    match color {
                        "red" => if amount > max_red { max_red = amount; },
                        "green" => if amount > max_green { max_green = amount; },
                        "blue" => if amount > max_blue { max_blue = amount; },
                        _ => panic!("Unknown color!")
                    }
                }
            }

            return max_red * max_green * max_blue;
        })
        .sum();

    println!("Sum of games: {}", sum);
}
