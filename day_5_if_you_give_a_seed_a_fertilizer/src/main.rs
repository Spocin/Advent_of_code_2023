use std::collections::HashMap;
use std::fs;
use std::path::Path;

const INPUT: &str = "./day_5_if_you_give_a_seed_a_fertilizer/resources/input.txt";

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

    let steps_maps: [HashMap<u32, u32>; 7] = lines[2..]
        .split(|&line| line.eq(""))
        .map(|step| str_to_step_map(&step))
        .collect::<Vec<HashMap<u32,u32>>>()
        .try_into()
        .unwrap();

    let min_location: u32 = seeds
        .iter()
        .map(|&seed| {
            println!("Search for: {}", seed);
            let mut seed_tmp = seed;
            for steps_map in &steps_maps {
                match steps_map.get(&seed_tmp) {
                    None => {
                        continue;
                    }
                    Some(val) => {
                        seed_tmp = *val;
                    }
                }
            }

            return seed_tmp;
        })
        .min()
        .unwrap();

    println!("Smalled location nr: {}", min_location)
}

fn str_to_step_map(step_str: &[&str]) -> HashMap<u32, u32> {
    println!("Building map for: {}", step_str[1]);
    let step_maps: Vec<[u32; 3]> = step_str.iter()
        .skip(1)
        .map(|line|
            line
            .trim()
            .split(" ")
            .map(|num_as_str| num_as_str.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .try_into().unwrap()
        )
        .collect::<Vec<[u32; 3]>>()
        .try_into()
        .unwrap();

    //Flatten maps
    let folded_map: HashMap<u32, u32> = step_maps
        .iter()
        .fold(HashMap::new(), |mut acc, map| {
            println!("Map: {} {} {}", map[0], map[1], map[2]);
            for idx in 0..map[2] {
                acc.insert(map[1]+idx, map[0]+idx);
            }

            return acc;
        });

    return folded_map;
}