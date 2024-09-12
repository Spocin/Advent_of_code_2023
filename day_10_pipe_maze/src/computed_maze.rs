use crate::maze::Maze;
use crate::pipe::Pipe;

pub struct ComputedMaze<'of_struct> {
    one_way: Vec<&'of_struct Pipe>,
    other_way: Vec<&'of_struct Pipe>,
    pub furthest_point_distance: u64,
}

impl ComputedMaze {
    pub fn new(maze: &Maze) -> ComputedMaze {
        let (
            one_way,
            other_way,
            furthest_point_distance
        ) = Self::build_pipes_chain(maze);

         ComputedMaze {
            one_way,
            other_way,
            furthest_point_distance
        }
    }

    /// Walks pipes starting from S in both directions.
    /// Stops when both iterators are on the same pipe.
    fn build_pipes_chain<'of_struct>(maze: &Maze) -> (
        Vec<&'of_struct Pipe>,
        Vec<&'of_struct Pipe>,
        u64
    ) {
        todo!()
    }
}