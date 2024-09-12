mod maze;
mod pipe;

use crate::maze::Maze;
use crate::pipe::Pipe;
use std::fs;
use std::path::Path;

const PATH_TO_TEST_INPUT: &str = "day_10_pipe_maze/resources/input.txt";

fn main() {
    let path = Path::new(PATH_TO_TEST_INPUT);
    let data = parse_input(path);

    let furthest_pipe = find_furthest_pipe(&data);

    println!("Furthest point is: ({}) steps away from start", furthest_pipe.get_steps());
}

//Reads input from given file
fn parse_input(path_to_input: &Path) -> String {
    match fs::read_to_string(path_to_input) {
        Err(err) => panic!("Something wrong with the path: {:?} {:?}", path_to_input, err),
        Ok(input) => input
    }
}

fn find_furthest_pipe(input: &String) -> Pipe {
    let maze = Maze::new(input);
    maze.compute_distances();
    maze.find_and_save_furthest_pipe();

    match maze.get_furthest_pipe() {
        None => panic!("No furthest pipe found!"),
        Some(point) => point
    }
}

#[cfg(test)]
mod tests {
    use crate::find_furthest_pipe;

    #[test]
    fn test_1() {
        let input: String = r###"
            .....
            .012.
            .1.3.
            .234.
            .....
        "###.to_string();

        let res_pipe = find_furthest_pipe(&input);

        assert_eq!(res_pipe.get_steps(), 4);
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

        let res_pipe = find_furthest_pipe(&input);

        assert_eq!(res_pipe.get_steps(), 8);
    }
}