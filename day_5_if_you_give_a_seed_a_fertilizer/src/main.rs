use std::fs;
use std::path::Path;
use iterchunks::IterArrayChunks;

const INPUT: &str = "./day_5_if_you_give_a_seed_a_fertilizer/resources/test_input.txt";

fn main() {
    println!("Loading file from {}", INPUT);

    let path = Path::new(INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    let lines: Vec<&str> = input.lines().collect();

    let seeds: Vec<u32> = lines[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .map(|seed_str| seed_str.parse::<u32>().unwrap())
        .collect();

    let steps_maps: [Vec<u32>; 7] = lines[2..]
        .split(|&line| line.eq(""))
        .map(|step| {
            let mut step_params: Vec<u32> = vec![];

            for line in step.iter().skip(1) {
                line.trim()
                    .split(" ")
                    .map(|line_params| line_params.parse::<u32>().unwrap())
                    .for_each(|param| step_params.push(param));
            }

            return step_params;
        })
        .collect::<Vec<Vec<u32>>>()
        .try_into()
        .unwrap();

    let min_location: u32 = seeds
        .iter()
        .map(|&seed| {
            let mut input = seed;

            println!("Initial input: {}", input);

            for steps_map in &steps_maps {
                for [dest_start, source_start , length] in steps_map.iter().array_chunks() {
                    //When input is not in range go to the next line
                    if input <= *source_start || input >= *source_start + *length {
                        println!("\tContinue without change: {}", input);
                        continue;
                    }

                    input = *dest_start + (input - *source_start);
                    println!("\tReplaced input to: {}", input);
                }
                println!();
            }

            return input;
        })
        .min()
        .unwrap();

    println!("Smalled location nr: {}", min_location)
}