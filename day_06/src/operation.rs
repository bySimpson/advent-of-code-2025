#[derive(Debug, Copy, Clone)]
pub enum Alignment {
    Left,
    Right
}

#[derive(Debug, Copy, Clone)]
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

    fn get_digit_at(input: u64, placement: u32) -> u64 {
        let len = input.checked_ilog10().unwrap_or(0) + 1;
        if placement >= len {
            return 0;
        }
        let output = input / 10u64.pow(len - placement - 1);
        let prefix = output / 10;

        output - prefix * 10
    }

    pub fn calculate_part_02(&self, numbers_input: &[u64], alignment: Alignment) -> u64 {
        let amount_max_digits = numbers_input.iter().map(|n| n.checked_ilog10().unwrap_or(0) + 1).max().unwrap_or(0);
        let mut calc_vec = vec![0; amount_max_digits as usize];
        for requested_digit in 0..amount_max_digits {
            for c_number in numbers_input.iter().rev() {
                let c_out: u64 = calc_vec[requested_digit as usize];
                let c_size =  c_out.checked_ilog10().unwrap_or(0) + 1;

                let digit = match alignment {
                    Alignment::Left => Self::get_digit_at(*c_number, requested_digit),
                    Alignment::Right => {
                        let c_number_size = c_number.checked_ilog10().unwrap_or(0) + 1;
                        let diff = amount_max_digits - c_number_size;
                        if requested_digit >= diff  {
                            Self::get_digit_at(*c_number, requested_digit - diff)
                        } else {
                            0
                        }
                    }
                };
                let to_pos = if c_out == 0 {0} else {c_size};
                calc_vec[requested_digit as usize] += digit * 10u64.pow(to_pos);
            }
        }

        let out = self.calculate_part_01(&calc_vec);
        out
    }
}