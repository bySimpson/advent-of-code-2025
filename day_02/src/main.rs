use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{ControlFlow};
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

    let line = reader.lines().next().unwrap()?;
    let iters: Vec<_> = line.split(',').map(|range| {
        let mut from_to = range.split('-');
        from_to.next().unwrap().parse::<u64>().unwrap()..=from_to.next().unwrap().parse::<u64>().unwrap()
    }).collect();

    let part1 = iters.clone().into_par_iter().fold(|| 0u64, |acc, iter| {
        iter.filter(|number| {
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
        }).sum::<u64>() + acc
    }).sum::<u64>();

    let part2 = iters.into_par_iter().fold(|| 0u64, |acc, iter| {
        iter.filter(|number| {
            // get number of digits
            let digits = number.checked_ilog10().unwrap_or(0) as u64 + 1u64;
            // try all possible lengths of the repeating pattern. digits/2 is maximum
            let out = (1u64..=digits/2).try_for_each(|number_length| {
                // multiple of checked length. If modulo != 0 it can be skipped
                if digits % number_length == 0 {
                    let mut compare_to = 0;
                    // compute first part of the number
                    let first_part = number / 10u64.pow(digits as u32 - number_length as u32);
                    for i in 0..(digits/number_length) {
                        // place first part of number into number to compare
                        compare_to += first_part * (10u64.pow(number_length as u32 * i as u32));
                    }
                    // compare if newly constructed number matches
                    if compare_to == *number {
                        if args.debug {
                            println!("Found {} ({})", number, first_part);
                        }
                        // early break
                        return ControlFlow::Break(true);
                    }
                }
                // continue as normal
                ControlFlow::Continue(())
            });
            // filter for only early brakes
            out == ControlFlow::Break(true)
        }).sum::<u64>() + acc
    }).sum::<u64>();


    println!("Part 1: {}", part1);

    println!("Part 2: {}", part2);
    Ok(())
}