use crate::maze::Maze;
use crate::pipe::Pipe;

pub struct ComputedMaze<'a> {
    one_way: Vec<&'a Pipe>,
    other_way: Vec<&'a Pipe>,
    furthest_point_distance: u64,
}

impl ComputedMaze<'_> {
    pub fn new(maze: &Maze) -> ComputedMaze {
        let (
            one_way,
            other_way,
            furthest_point_distance
        ) = Self::build_pipes_chain(maze);
        //Perhaps this can be moved to separate function to postpone computation?

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

    pub fn furthest_point_distance(&self) -> u64 {
        self.furthest_point_distance
    }
}