use std::fmt::{Display, Formatter};

pub struct MapCoordinates {
    pub name: String,

    pub l_name: String,
    pub r_name: String,
}

impl MapCoordinates {
    pub fn new(line: &str) -> MapCoordinates {
        //Find brackets
        let start_idx: usize;
        match line.chars().position(|el| el == '(') {
            None => { panic!("Can't create MapCoordinates. No '(' found in line:\n{}", line) }
            Some(val) => { start_idx = val }
        }

        let end_idx: usize;
        match line.chars().position(|el| el == ')') {
            None => { panic!("Can't create MapCoordinates. No ')' found in line:\n{}", line) }
            Some(val) => { end_idx = val }
        }

        //Extrapolate info from withing brackets
        let brackets_content = &line[start_idx + 1 ..= end_idx - 1];
        let names = brackets_content.split(',')
            .map(|el| el.trim())
            .collect::<Vec<&str>>();

        return MapCoordinates {
            name: line[..=2].to_string(),
            l_name: names[0].to_string(),
            r_name: names[1].to_string(),
        }
    }
}

impl Display for MapCoordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "MapCoordinates [ name: {}, l_name: {}, r_name: {} ]", self.name, self.l_name, self.r_name);
    }
}