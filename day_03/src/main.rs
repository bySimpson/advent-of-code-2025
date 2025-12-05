use clap::{arg, Parser};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;
use std::string::String;
use std::time::Instant;

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

    let input = read.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let parse_duration = parse_time.elapsed();

    let calc_time = Instant::now();
    let (part1, part2) = input.par_iter().map(|line| {
        let p1 = calculate_joltage(line, 2);
        let p2 = calculate_joltage(line, 12);
        (p1, p2)
    }).reduce(|| (0, 0), |acc, item| {
        (acc.0 + item.0, acc.1 + item.1)
    });
    
    let calc_duration = calc_time.elapsed();
    let total_duration = total_time.elapsed();

    println!("Part 1 {}", part1);
    println!("Part 2 {}", part2);

    println!("Perf - Total: {:?}, Parsing: {:?}, Calculation total: {:?}", total_duration, parse_duration, calc_duration);
}

fn calculate_joltage(line: &[u32], battery_amount: usize) -> u32 {
    let mut outnumber = 0u32;
    let mut last_index: i32 = -1;
    for (iter, current_digit) in (0..=battery_amount -1usize).rev().enumerate() {
        let c_match = line.iter().enumerate().fold_while(0u32, |acc, (idx, number)| {
            // Skip last x positions since number needs space for x places afterwards. Return early
            if idx < line.len() - current_digit {
                // Check if higher value is reached
                if *number > acc && last_index < idx as i32 {
                    // Save index for next calculation
                    last_index = idx as i32;
                    // leftmost 9 is best value. Early break
                    if *number == 9 {
                        return Done(*number);
                    }
                    return Continue(*number)
                }
                return Continue(acc);
            }
            Done(acc)
        }).into_inner();
        outnumber += c_match * 10u32.pow((battery_amount - iter - 1) as u32);
    }
    outnumber
}