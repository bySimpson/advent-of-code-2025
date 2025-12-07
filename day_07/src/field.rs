use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Field {
    Beam,
    Splitter,
    Space
}

impl Field {
    pub fn new(character: char) -> Field {
        match character {
            'S' => Field::Beam,
            '.' => Field::Space,
            '^' => Field::Splitter,
            _ => panic!("Unknown field: {}", character)
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            Field::Splitter => write!(f, "^"),
            Field::Space => write!(f, "."),
            Field::Beam => write!(f, "|"),
        }
    }
}