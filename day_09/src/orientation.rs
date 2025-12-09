#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical
}

impl Orientation {
    pub fn from_line(line: ((u64, u64), (u64, u64))) -> Orientation {
        if line.0.0 == line.1.0 {
            // x matches, vertical
            Orientation::Vertical
        } else if line.0.1 == line.1.1 {
            // y matches, horizontal
            Orientation::Horizontal
        } else {
            panic!("Invalid input!")
        }
    }
}