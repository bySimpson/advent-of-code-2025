use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use clap::{arg, Parser};
use anyhow::{Result};
use std::string::String;
use itertools::FoldWhile::{Continue, Done};
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

    let mut input = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        input.push(line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>());
    }

    let part1 = input.iter().enumerate().map(|(idx, line)| {
        let mut outvec: Vec<u64> = Vec::new();
        let mut last_index: isize = -1;
        for current_digit in (0..=1).rev() {
            outvec.push(line.iter().enumerate().fold_while(0u64, |acc, (idx, nmbr)| {
                if idx < line.len() - current_digit {
                    if *nmbr > acc && last_index < idx as isize {
                        last_index = idx as isize;
                        if *nmbr == 9 {
                            return  Done(*nmbr);
                        }
                        return Continue(*nmbr)
                    }
                    return Continue(acc);
                }
                return Done(acc);
            }).into_inner());
        }
        outvec
    }).map(|number| {
        number.iter().rev().enumerate().fold(0, |acc, (idx, value)| {
            acc + *value * 10u64.pow(idx as u32)
        })
    }).sum::<u64>();

    println!("Part 1: {}", part1);


    let part2 = input.iter().enumerate().map(|(idx, line)| {
        let mut outvec: Vec<u64> = Vec::new();
        let mut last_index: isize = -1;
        for current_digit in (0..=11).rev() {
            outvec.push(line.iter().enumerate().fold_while(0u64, |acc, (idx, nmbr)| {
                if idx < line.len() - current_digit {
                    if *nmbr > acc && last_index < idx as isize {
                        last_index = idx as isize;
                        if *nmbr == 9 {
                            return  Done(*nmbr);
                        }
                        return Continue(*nmbr)
                    }
                    return Continue(acc);
                }
                return Done(acc);
            }).into_inner());
        }
        outvec
    }).map(|number| {
        number.iter().rev().enumerate().fold(0, |acc, (idx, value)| {
            acc + *value * 10u64.pow(idx as u32)
        })
    }).sum::<u64>();

    println!("Part 2: {}", part2);
    Ok(())
}