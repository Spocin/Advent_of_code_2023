use crate::maze::Maze;
use crate::pipe::{Pipe, PipeCoordinates};

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

        let map = maze.map();
        let starting_coordinates = maze.starting_pipe_coordinates();

        //Each way will have starting pipe at the beginning
        let starting_pipe = map[starting_coordinates.y()][starting_coordinates.x()].as_ref().unwrap();
        one_way.push(starting_pipe);
        other_way.push(starting_pipe);

        //Find pipes adjacent to staring pipe
        one_way.push(Self::find_next_pipe(maze, one_way[0], None));
        other_way.push(Self::find_next_pipe(maze, other_way[0], Some(one_way[1].coordinates())));

        println!("First way: {:?}", one_way);
        println!("Second way: {:?}", other_way);

        todo!();

        let mut first_ptr = 1;
        let mut second_ptr = 1;

        //Walk pipes until both pointers point to same coordinates
        while *one_way[first_ptr].coordinates() != *other_way[second_ptr].coordinates() {
            //Find first pipe adjacent to S


            //Find second pipe adjacent to S

            //Push first pipe to vec
            //Push second pipe to vec

            first_ptr += 1;
            second_ptr += 1;
        }

        todo!();

         ComputedMaze {
            one_way,
            other_way,
            furthest_point_distance: one_way.len() as u64,
        }
    }

    pub fn furthest_point_distance(&self) -> u64 {
        self.furthest_point_distance
    }

    /// Searched for pipe around given selected_pipe.
    /// Ignores already found pipe under coordinates_to_ignore.
    fn find_next_pipe<'a>(
        maze: &Maze,
        selected_pipe: &Pipe,
        coordinates_to_ignore_opt: Option<&PipeCoordinates>,
    ) -> &'a Pipe {
        let curr_coords = selected_pipe.coordinates();

        //Check North
        if curr_coords.y() != 0 {
            if let Some(north_pipe) = Self::check_north(maze, selected_pipe, coordinates_to_ignore_opt) {
                return north_pipe;
            }
        }

        //Check East
        if curr_coords.x() != maze.width() {
            if let Some(east_pipe) = Self::check_east(maze, selected_pipe, coordinates_to_ignore_opt) {
                return east_pipe;
            }
        }

        //Check South
        if curr_coords.y() != maze.height() {
            if let Some(south_pipe) = Self::check_south(maze, selected_pipe, coordinates_to_ignore_opt) {
                return south_pipe;
            }
        }

        //Check West
        if curr_coords.x() != 0 {
            if let Some(west_pipe) = Self::check_west(maze, selected_pipe, coordinates_to_ignore_opt) {
                return west_pipe;
            }
        }

        panic!("Did not find any matching pipe around: {:?}", selected_pipe);
    }

    fn check_north<'a>(
        maze: &Maze,
        selected_pipe: &Pipe,
        coordinates_to_ignore_opt: Option<&PipeCoordinates>,
    ) -> Option<&'a Pipe> {
        todo!()
    }

    fn check_east<'a>(
        maze: &Maze,
        selected_pipe: &Pipe,
        coordinates_to_ignore_opt: Option<&PipeCoordinates>,
    ) -> Option<&'a Pipe> {
        todo!()
    }

    fn check_south<'a>(
        maze: &Maze,
        selected_pipe: &Pipe,
        coordinates_to_ignore_opt: Option<&PipeCoordinates>,
    ) -> Option<&'a Pipe> {
        todo!()
    }

    fn check_west<'a>(
        maze: &Maze,
        selected_pipe: &Pipe,
        coordinates_to_ignore_opt: Option<&PipeCoordinates>,
    ) -> Option<&'a Pipe> {
        todo!()
    }
}