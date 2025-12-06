pub enum Operation {
    Plus,
    Multiply
}

impl Operation {
    pub fn new(line: &str) -> Self {
        match line {
            "+" => Operation::Plus,
            "*" => Operation::Multiply,
            _ => panic!("Invalid operation: {}", line)
        }
    }

    pub fn calculate_part_01(&self, line_number: &[u64]) -> u64 {
        match self {
            Operation::Plus => line_number.iter().sum(),
            Operation::Multiply => line_number.iter().product(),
        }
    }
}