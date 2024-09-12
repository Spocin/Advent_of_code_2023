use crate::pipe::{Pipe, PipeCoordinates, PipeType};

pub struct Maze {
    map: Vec<Vec<Pipe>>,
    starting_pipe_coordinates: PipeCoordinates,
}

impl Maze {
    pub fn new(input: &String) -> Maze {
        let mut map: Vec<Vec<Pipe>> = Vec::with_capacity(input.lines().count());
        let mut start_coordinates_opt: Option<PipeCoordinates> = None;

        //Go over the lines
        for (y, line) in input.lines().enumerate() {
            println!("{}", line);
            let mut line_pipes: Vec<Pipe> = Vec::with_capacity(line.len());

            //Go over the chars
            for (x, char) in line.chars().enumerate() {
                let coordinates = PipeCoordinates::new(x,y);

                //Make char into Pipe
                let pipe = match Pipe::new(coordinates, char) {
                    Ok(pipe) => pipe,
                    Err(msg) => panic!("Error while building maze: {} at {:?}", msg, coordinates)
                };

                //Surface the starting pipe
                if start_coordinates_opt.is_none() && pipe.pipe_type() == PipeType::START {
                    start_coordinates_opt = Some(coordinates);
                }

                line_pipes.push(pipe);
            }

            map.push(line_pipes);
        }

        let starting_pipe_coordinates = match start_coordinates_opt {
            None => panic!("No starting pipe found"),
            Some(val) => val
        };

        Maze {
            map,
            starting_pipe_coordinates,
        }
    }
}