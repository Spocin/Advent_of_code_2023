use crate::computed_maze::SearchDirection::{EAST, NORTH, SOUTH, WEST};
use crate::maze::Maze;
use crate::pipe::PipeType::*;
use crate::pipe::{Pipe, PipeCoordinates, PipeType};

pub struct ComputedMaze<'a> {
    one_way: Vec<&'a Pipe>,
    other_way: Vec<&'a Pipe>,
    furthest_point_distance: u64,
}

pub enum SearchDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl SearchDirection {
    fn move_matrix_by(&self) -> (i32, i32) {
        match *self {
            NORTH => (0, -1),
            EAST => (1, 0),
            SOUTH => (0, 1),
            WEST => (-1, 0),
        }
    }

    fn allowed_pipes(&self) -> [PipeType; 3] {
        match *self {
            NORTH => [NS, SW, SE],
            EAST => [EW, NW, SW],
            SOUTH => [NS, NE, NW],
            WEST => [EW, NE, SE],
        }
    }
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
        maze: &'a Maze,
        selected_pipe: &Pipe,
        coordinates_to_ignore_opt: Option<&PipeCoordinates>,
    ) -> &'a Pipe {
        let curr_coords = selected_pipe.coordinates();

        //Check North
        if curr_coords.y() != 0 {
            if let Some(north_pipe) = Self::check_neighbour(NORTH, maze, selected_pipe, coordinates_to_ignore_opt) {
                return north_pipe;
            }
        }

        //Check East
        if curr_coords.x() != maze.width() {
            println!("Curr: {:?}", curr_coords);
            if let Some(east_pipe) = Self::check_neighbour(EAST, maze, selected_pipe, coordinates_to_ignore_opt) {
                println!("Returned East: {:?}", east_pipe);
                return east_pipe;
            }
        }

        //Check South
        if curr_coords.y() != maze.height() {
            if let Some(south_pipe) = Self::check_neighbour(SOUTH, maze, selected_pipe, coordinates_to_ignore_opt) {
                println!("Returned South: {:?}", south_pipe);
                return south_pipe;
            }
        }

        //Check West
        if curr_coords.x() != 0 {
            if let Some(west_pipe) = Self::check_neighbour(WEST, maze, selected_pipe, coordinates_to_ignore_opt) {
                return west_pipe;
            }
        }

        panic!("Did not find any matching pipe around: {:?}", selected_pipe);
    }

    fn check_neighbour<'a>(
        direction: SearchDirection,
        maze: &'a Maze,
        selected_pipe: &Pipe,
        coordinates_to_ignore_opt: Option<&PipeCoordinates>,
    ) -> Option<&'a Pipe> {
        let (x, y) = direction.move_matrix_by();
        let curr_cords = selected_pipe.coordinates();

        //Computed moved indexes
        let moved_x: usize = usize::try_from(i32::try_from(curr_cords.x()).unwrap() + x).unwrap();
        let moved_y: usize = usize::try_from(i32::try_from(curr_cords.y()).unwrap() + y).unwrap();

        //Get that pipe
        let targeted_pipe_opt = maze.map()[moved_y][moved_x].as_ref();

        //Short circuit when targeted coordinates are not a pipe
        if targeted_pipe_opt.is_none() {
            return None;
        }

        //Check if found pipe should be ignored
        let targeted_pipe = targeted_pipe_opt?;
        if let Some(coordinates_to_ignore) = coordinates_to_ignore_opt {
            if coordinates_to_ignore == targeted_pipe.coordinates() {
                return None;
            }
        }

        //Check whether targeted pipe can be connected
        if !direction.allowed_pipes().contains(targeted_pipe.pipe_type()) {
            return None;
        }

        Some(targeted_pipe)
    }
}