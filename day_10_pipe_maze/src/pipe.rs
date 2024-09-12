#[repr(u8)]
pub enum PipeType {
    NS = b'|',
    EW = b'-',
    NE = b'L',
    NW = b'J',
    SW = b'7',
    SE = b'F',
    EMPTY = b'.',
    START = b'S',
}

impl PartialEq<PipeType> for &PipeType {
    fn eq(&self, other: &PipeType) -> bool {
        self == other
    }
}

#[derive(Clone, Copy)]
pub struct PipeCoordinates {
    pub x: usize,
    pub y: usize,
}

pub struct Pipe {
    coordinates: PipeCoordinates,
    pipe_type: PipeType,
}

impl Pipe {
    pub fn new(coordinates: PipeCoordinates, char: char) -> Pipe {
        todo!()
    }

    pub fn pipe_type(&self) -> &PipeType {
        &self.pipe_type
    }
}