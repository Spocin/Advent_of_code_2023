use crate::pipe::Pipe;

pub struct Maze {
    map: Vec<Vec<Pipe>>, //Map of points
    furthest_pipe: Option<Pipe>,
}

impl Maze {
    pub fn new(input: &String) -> Maze {
        todo!()
    }

    pub(crate) fn compute_distances(&self) {
        todo!()
    }

    pub(crate) fn find_and_save_furthest_pipe(&self) {
        todo!()
    }

    pub fn get_furthest_pipe(&self) -> Option<Pipe> {
        todo!()
    }
}