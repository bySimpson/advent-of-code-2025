mod instruction;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RemAssign;
use clap::{arg, Parser};
use anyhow::{Result};
use std::string::String;
use crate::instruction::Instruction;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}


fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        instructions.push(Instruction::new(&line));
    }

    let mut part1 = 0;
    instructions.iter().fold(50, |acc, x| {
        let ret = (acc + x.get_rotation_number()).rem_euclid(100);
        if ret == 0 {
            part1 += 1;
        }
        ret
    });

    let mut part2 = 0;
    instructions.iter().fold(50, |acc, x| {
        let new_position = acc + x.get_rotation_number();
        let dial_pos = new_position.rem_euclid(100);
        if x.get_rotation_number() > 0 {
            part2 += new_position / 100;
        } else {
            let reversed = (100 - acc) % 100;
            part2 += (reversed - x.get_rotation_number()) / 100;
        }
        dial_pos
    });

    println!("Part 1: {}", part1);

    println!("Part 2: {}", part2); // 6810 - too high, 6750 - too high, 2680 - too low
    Ok(())
}