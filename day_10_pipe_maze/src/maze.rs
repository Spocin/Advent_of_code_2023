use crate::pipe::{Pipe, PipeCoordinates, PipeType};

pub struct Maze {
    map: Vec<Vec<Option<Pipe>>>,
    starting_pipe_coordinates: PipeCoordinates,
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(input: &String) -> Maze {
        let mut map: Vec<Vec<Option<Pipe>>> = Vec::with_capacity(input.lines().count());
        let mut start_coordinates_opt: Option<PipeCoordinates> = None;

        //Go over the lines
        for (y, line) in input.lines().enumerate() {
            let mut line_pipes: Vec<Option<Pipe>> = Vec::with_capacity(line.len());

            //Go over the chars
            for (x, char) in line.chars().enumerate() {
                let coordinates = PipeCoordinates::new(x,y);

                //Make char into Pipe
                let pipe = Pipe::new(coordinates, char);

                if let Some(ref pipe) = pipe {
                    //Surface the starting pipe
                    if start_coordinates_opt.is_none() && *pipe.pipe_type() == PipeType::START {
                        start_coordinates_opt = Some(coordinates);
                    }
                }

                line_pipes.push(pipe);
            }

            map.push(line_pipes);
        }

        let starting_pipe_coordinates = match start_coordinates_opt {
            None => panic!("No starting pipe found"),
            Some(val) => val
        };

        let width = map[0].len();
        let height = map.len();

        Maze {
            map,
            starting_pipe_coordinates,
            width,
            height,
        }
    }

    pub fn map(&self) -> &Vec<Vec<Option<Pipe>>> {
        &self.map
    }

    pub fn starting_pipe_coordinates(&self) -> &PipeCoordinates {
        &self.starting_pipe_coordinates
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}