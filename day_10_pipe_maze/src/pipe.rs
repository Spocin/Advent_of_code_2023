#[derive(PartialEq)]
pub enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    EMPTY,
    START,
}

impl PipeType {
    pub fn from(c: char) -> Result<PipeType, String> {
        match c {
            '|' => Ok(PipeType::NS),
            '-' => Ok(PipeType::EW),
            'L' => Ok(PipeType::NE),
            'J' => Ok(PipeType::NW),
            '7' => Ok(PipeType::SW),
            'F' => Ok(PipeType::SE),
            '.' => Ok(PipeType::EMPTY),
            'S' => Ok(PipeType::START),
            _ => Err(format!("Unknown PipeType: {}. Can't convert", c)),
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
    pub fn new(coordinates: PipeCoordinates, char: char) -> Result<Pipe, String> {
        let pipe_type = match PipeType::from(char) {
            Ok(val) => val,
            Err(msg) => return Err(msg),
        };

        Ok(Pipe {
            coordinates,
            pipe_type,
        })
    }

    pub fn pipe_type(&self) -> &PipeType {
        &self.pipe_type
    }
}