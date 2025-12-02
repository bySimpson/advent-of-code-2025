use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{arg, Parser};
use anyhow::{Result};
use std::string::String;
use rayon::prelude::*;

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

    let mut iters = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        line.split(',').for_each(|range| {
            let mut from_to = range.split('-');
            iters.push(from_to.next().unwrap().parse::<u64>().unwrap()..=from_to.next().unwrap().parse::<u64>().unwrap());
        })
    }

    let part1 = iters.into_iter().flatten().filter(|number| {
        // get number of digits
        let digits = number.checked_ilog10().unwrap_or(0) + 1;
        // Only account for symmetric numbers
        if digits % 2 == 0 {
            // Get first half of number
            let first_half = number / 10u64.pow(digits / 2);
            // Get second half of number
            let second_half = number - first_half * 10u64.pow(digits / 2);
            if first_half == second_half {
                return true;
            }
            return false;
        }
        false
    }).sum::<u64>();


    println!("Part 1: {}", part1);

    println!("Part 2: {}", 0);
    Ok(())
}