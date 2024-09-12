use crate::pipe::Pipe;

pub struct Maze<'of_struct> {
    map: Vec<Vec<&'of_struct Pipe>>,
    starting_pipe: &'of_struct Pipe,
}

impl Maze {
    pub fn new(input: &String) -> Maze {
        todo!()
        //Parse input to map of Pipes
    }
}