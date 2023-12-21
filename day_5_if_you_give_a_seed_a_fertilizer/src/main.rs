use std::fs;
use std::path::Path;
use iterchunks::IterArrayChunks;

const INPUT: &str = "./day_5_if_you_give_a_seed_a_fertilizer/resources/input.txt";

fn main() {
    let path = Path::new(INPUT);
    let input = fs::read_to_string(path).expect("Something wrong with path!");

    let lines: Vec<&str> = input.lines().collect();

    let seeds: Vec<u64> = lines[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .map(|seed_str| seed_str.parse::<u64>().unwrap())
        .collect();

    let steps_maps: [Vec<u64>; 7] = lines[2..]
        .split(|&line| line.eq(""))
        .map(|step| {
            let mut step_params: Vec<u64> = vec![];

            for line in step.iter().skip(1) {
                line.trim()
                    .split(" ")
                    .map(|line_params| line_params.parse::<u64>().unwrap())
                    .for_each(|param| step_params.push(param));
            }

            return step_params;
        })
        .collect::<Vec<Vec<u64>>>()
        .try_into()
        .unwrap();

    let min_location: u64 = seeds
        .iter()
        .map(|&seed| {
            let mut input = seed;

            for steps_map in &steps_maps {
                for [dest_start, source_start , length] in IterArrayChunks::array_chunks(steps_map.iter()) {
                    if input >= *source_start && input <= *source_start + *length {
                        input = *dest_start + (input - *source_start);

                        //Leave after we found match in step
                        break;
                    }
                }
            }

            return input;
        })
        .min()
        .unwrap();

    println!("Smallest location nr: {}", min_location)
}