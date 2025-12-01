pub enum Instruction {
    Left(i32),
    Right(i32)
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        match line.starts_with("R") {
            true => {
                Instruction::Right(line.replace("R", "").parse::<i32>().unwrap())
            }
            false => {
                Instruction::Left(line.replace("L", "").parse::<i32>().unwrap())
            }
        }
    }

    pub fn get_rotation_number(&self) -> i32 {
        match self { 
            Instruction::Left(number) => number * -1,
            Instruction::Right(number) => *number,
        }
    }
}