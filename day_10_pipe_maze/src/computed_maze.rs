use crate::maze::Maze;
use crate::pipe::Pipe;

pub struct ComputedMaze<'a> {
    one_way: Vec<&'a Pipe>,
    other_way: Vec<&'a Pipe>,
    furthest_point_distance: u64,
}

impl ComputedMaze<'_> {
    /// Walks pipes starting from S in both directions.
    /// Stops when both iterators are on the same pipe.
    pub fn new(maze: &Maze) -> ComputedMaze {
        let mut one_way = Vec::new();
        let mut other_way = Vec::new();

        return todo!();

         ComputedMaze {
            one_way,
            other_way,
            furthest_point_distance: one_way.len() as u64,
        }
    }

    pub fn furthest_point_distance(&self) -> u64 {
        self.furthest_point_distance
    }
}