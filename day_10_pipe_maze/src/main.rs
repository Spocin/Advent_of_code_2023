mod maze;
mod pipe;
mod computed_maze;

use crate::computed_maze::ComputedMaze;
use crate::maze::Maze;
use std::fs;
use std::path::Path;

const PATH_TO_TEST_INPUT: &str = "day_10_pipe_maze/resources/input.txt";

fn main() {
    let path = Path::new(PATH_TO_TEST_INPUT);
    let data = parse_input(path);

    let furthest_distance = find_furthest_pipe_distance(&data);

    println!("Furthest point is: ({}) steps away from start", furthest_distance);
}

//Reads input from given file
fn parse_input(path_to_input: &Path) -> String {
    match fs::read_to_string(path_to_input) {
        Err(err) => panic!("Something wrong with the path: {:?} {:?}", path_to_input, err),
        Ok(input) => input
    }
}

fn find_furthest_pipe_distance(input: &String) -> u64 {
    let maze = Maze::new(input);
    let computed_maze = ComputedMaze::new(&maze);

    computed_maze.furthest_point_distance
}

#[cfg(test)]
mod tests {
    use crate::find_furthest_pipe_distance;

    #[test]
    fn test_1() {
        let input: String = r###"
            .....
            .012.
            .1.3.
            .234.
            .....
        "###.to_string();

        let res_pipe = find_furthest_pipe_distance(&input);

        assert_eq!(res_pipe, 4);
    }

    #[test]
    fn test_2() {
        let input = r###"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        ""###.to_string();

        let res_pipe = find_furthest_pipe_distance(&input);

        assert_eq!(res_pipe, 8);
    }
}