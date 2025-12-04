use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Field {
    Roll,
    Air
}

impl Field {
    pub fn new(character: char) -> Field {
        match character {
            '@' => Field::Roll,
            '.' => Field::Air,
            _ => panic!("Unknown field: {}", character)
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            Field::Roll => write!(f, "@"),
            Field::Air => write!(f, "."),
        }
    }
}