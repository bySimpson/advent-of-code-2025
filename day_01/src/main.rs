mod instruction;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RemAssign;
use clap::{arg, Parser};
use anyhow::{Result};
use std::string::String;
use std::time::Instant;
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
    let total_time = Instant::now();
    
    
    let args = Args::parse();
    let parse_time = Instant::now();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        instructions.push(Instruction::new(&line));
    }
    let parse_duration = parse_time.elapsed();

    let calc_time = Instant::now();
    let part1_time = Instant::now();
    let mut part1 = 0;
    instructions.iter().fold(50, |acc, x| {
        let ret = (acc + x.get_rotation_number()).rem_euclid(100);
        if ret == 0 {
            part1 += 1;
        }
        ret
    });
    let par1_duration = part1_time.elapsed();

    let part2_time = Instant::now();
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

    let part2_duration = part2_time.elapsed();
    let calc_duration = calc_time.elapsed();
    let total_duration = total_time.elapsed();

    println!("Part 1 ({:?}): {}", par1_duration, part1);
    println!("Part 2 ({:?}): {}", part2_duration, part2);

    println!("Perf - Total: {:?}, Parsing: {:?}, Calculation total: {:?}", total_duration, parse_duration, calc_duration);

    Ok(())
}