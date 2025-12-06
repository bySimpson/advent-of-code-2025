mod operation;

use clap::{arg, Parser};
use itertools::Itertools;
use std::fs;
use std::ops::ControlFlow;
use std::ops::ControlFlow::Continue;
use std::string::String;
use std::time::Instant;
use crate::operation::Operation;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}
fn main() {
    let total_time = Instant::now();

    let args = Args::parse();
    let parse_time = Instant::now();
    let read = fs::read_to_string(args.path).unwrap();

    let mut operations = Vec::new();
    let operations_len = read.lines().next().unwrap().split_whitespace().count();
    let mut numbers: Vec<Vec<u64>>  = vec![Vec::new(); operations_len];
    read.lines().rev().enumerate().for_each(|(iter, line)| {
        let pieces = line.split_whitespace();
        if iter == 0 {
            for piece in pieces {
                operations.push(Operation::new(piece));
            }
        } else {
            for (i, piece) in pieces.enumerate() {
                numbers[i].push(piece.parse::<u64>().unwrap());
            }
        }
    });


    let parse_duration = parse_time.elapsed();
    let calc_time = Instant::now();
    let part1_time = Instant::now();

    let part1 = numbers.iter().enumerate().map(|(iter, numbers)| {
        let operation = operations.get(iter).unwrap();
        operation.calculate_part_01(numbers)
    }).sum::<u64>();

    let par1_duration = part1_time.elapsed();

    let part2_time = Instant::now();

    let part2 = 0;

    let part2_duration = part2_time.elapsed();
    let calc_duration = calc_time.elapsed();
    let total_duration = total_time.elapsed();

    println!("Part 1 ({:?}): {}", par1_duration, part1);
    println!("Part 2 ({:?}): {}", part2_duration, part2);

    println!("Perf - Total: {:?}, Parsing: {:?}, Calculation total: {:?}", total_duration, parse_duration, calc_duration);
}