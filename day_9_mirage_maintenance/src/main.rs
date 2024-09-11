#![feature(test)]
extern crate test;

mod main_spec;

use std::fs;
use std::path::Path;

const PATH_TO_TEST_INPUT: &str = "day_9_mirage_maintenance/resources/input.txt";

pub fn main() {
    let path = Path::new(PATH_TO_TEST_INPUT);
    let data = parse_input(path);

    let sum: i64 = data
        .iter()
        .map(|line| compute_next_num(line.clone()))
        .sum();

    println!("Computed sum: {:?}", sum);
}

//Reads input from given file and parses its content
fn parse_input(path_to_input: &Path) -> Vec<Vec<i64>> {
    let input = match fs::read_to_string(path_to_input) {
        Err(err) => panic!("Something wrong with the path: {:?} {:?}", path_to_input ,err),
        Ok(input) => input
    };

    input
        .lines()
        .map(|line|
            line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        ).collect()
}

//Computes next number of the given input
pub fn compute_next_num(x: Vec<i64>) -> i64 {
    let mut generations = vec![x.clone(), create_new_gen(&x)];

    let mut curr_idx = generations.len() - 1;
    while !are_all_el_zero(&generations[curr_idx]) {
        generations.push(create_new_gen(&generations[curr_idx]));
        curr_idx += 1;
    }

    println!("Input {:?} creates: {} generations", x, curr_idx + 1);

    walk_back_tree(&generations)
}

//Based on given value creates new generation of values
fn create_new_gen(x: &[i64]) -> Vec<i64> {
    x.windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

//Checks if all elements are zero
fn are_all_el_zero(x: &[i64]) -> bool {
    x.iter().all(|el| *el == 0)
}

//Iterates tree back, predicting the next value until first generation
fn walk_back_tree(x: &[Vec<i64>]) -> i64 {
    x.iter()
        .rev()
        .skip(1)
        .fold(0, |mut acc, gen| {
            acc = gen.first().unwrap() - acc;

            acc
        })
}