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

pub struct Pipe {
    coordinates: (usize, usize),
    pipe_type: PipeType,
    steps: Option<u64>,
    was_visited: bool
}

impl Pipe {
    pub fn new(coordinates: (usize, usize), char: char) -> Pipe {
        todo!()
    }

    pub(crate) fn get_steps() -> u64 {
        todo!()
    }
}