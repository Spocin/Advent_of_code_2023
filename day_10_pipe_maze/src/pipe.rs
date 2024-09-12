#[derive(PartialEq)]
pub enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    START,
}

impl PipeType {
    pub fn from(c: char) -> Option<PipeType> {
        match c {
            '|' => Some(PipeType::NS),
            '-' => Some(PipeType::EW),
            'L' => Some(PipeType::NE),
            'J' => Some(PipeType::NW),
            '7' => Some(PipeType::SW),
            'F' => Some(PipeType::SE),
            'S' => Some(PipeType::START),
            '.' => None,
            _ => panic!("Unknown PipeType: {}. Can't convert", c),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PipeCoordinates {
    x: usize,
    y: usize,
}

impl PipeCoordinates {
    pub fn new(x: usize, y: usize) -> PipeCoordinates {
        PipeCoordinates { x, y }
    }
}

pub struct Pipe {
    coordinates: PipeCoordinates,
    pipe_type: PipeType,
}

impl Pipe {
    pub fn new(coordinates: PipeCoordinates, char: char) -> Option<Pipe> {
        let pipe_type = match PipeType::from(char) {
            Some(pipe_type) => pipe_type,
            None => return None
        };

        Some(Pipe {
            coordinates,
            pipe_type,
        })
    }

    pub fn pipe_type(&self) -> &PipeType {
        &self.pipe_type
    }
}