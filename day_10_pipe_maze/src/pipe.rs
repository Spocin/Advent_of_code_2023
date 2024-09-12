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

impl PartialEq<PipeType> for &PipeType {
    fn eq(&self, other: &PipeType) -> bool {
        self == other
    }
}

impl PipeType {
    pub fn from<'a>(c: char) -> Result<PipeType, &'a str> {
        match c {
            '|' => Ok(PipeType::NS),
            '-' => Ok(PipeType::EW),
            'L' => Ok(PipeType::NE),
            'J' => Ok(PipeType::NW),
            '7' => Ok(PipeType::SW),
            'F' => Ok(PipeType::SE),
            '.' => Ok(PipeType::EMPTY),
            'S' => Ok(PipeType::START),
            _ => Err("Unknown PipeType. Can't convert"),
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
    pub fn new<'a>(coordinates: PipeCoordinates, char: char) -> Result<Pipe, &'a str> {
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