use clap::{Parser, arg};
use itertools::Itertools;
use std::collections::HashSet;
use rayon::prelude::*;
use std::fs;
use std::ops::BitXor;
use std::string::String;
use std::time::Instant;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool,
}

fn part1(button_presses: &[u64], solution: u64) -> u64 {
    let mut c_operation: HashSet<u64> = HashSet::new();
    c_operation.insert(0); // start with a blank field
    let mut iteration = 0;
    'outer: loop {
        iteration += 1;
        let mut next: HashSet<u64> = HashSet::new();

        for operation in c_operation {
            for c_button_press in button_presses.iter() {
                let current = operation.bitxor(c_button_press);
                if current == solution {
                    break 'outer;
                } else {
                    next.insert(current);
                }
            }
        }

        c_operation = next;
    }
    iteration
}

fn part2(button_presses: &[u64], required_joltages: &[u64]) -> u64 {
    let mut c_operation: HashSet<Vec<u64>> = HashSet::new();
    c_operation.insert(vec![0; required_joltages.len()]); // start with a blank field
    let mut iteration = 0;
    'outer: loop {
        iteration += 1;
        let mut next: HashSet<Vec<u64>> = HashSet::new();

        for operation in c_operation {
            for c_button_press in button_presses.iter() {
                let mut current= Vec::new();
                for (i, c_joltage_level) in operation.iter().enumerate() {
                    let change = if c_button_press & (1 << i) > 0 {1} else {0};
                    let new = c_joltage_level + change;
                    if new > required_joltages[i] {
                        // skip, not viable
                        break;
                    }
                    current.push(new);
                }
                if current == required_joltages {
                    break 'outer;
                } else {
                    next.insert(current);
                }
            }
        }

        c_operation = next;
    }
    iteration
}

fn main() {
    let total_time = Instant::now();

    let args = Args::parse();
    let parse_time = Instant::now();
    let read = fs::read_to_string(args.path).unwrap();
    let mut button_presses: Vec<Vec<u64>> = Vec::new();
    let mut solutions: Vec<u64> = Vec::new();
    let mut required_joltage: Vec<Vec<u64>> = Vec::new();
    read.lines()
        .for_each(|line| {
            let mut c_presses: Vec<u64> = Vec::new();
            line.split_whitespace().enumerate().for_each(|(i, s)| {
                if i == 0 {
                    let mut c_solution: u64 = 0;
                    s.replace("[", "").replace("]", "").chars().enumerate().for_each(|(c_i, char)| {
                        let is_set: u64 = if char == '#' {1} else {0};
                        c_solution += is_set << c_i;
                    });
                    solutions.push(c_solution);
                } else {
                    let mut c_press: u64 = 0;
                    if !s.contains("{") {
                        s.replace("(", "").replace(")", "").split(",").map(|c| c.parse::<u64>().unwrap()).for_each(|n| {
                            c_press += 1 << n;
                        });
                        c_presses.push(c_press);
                    } else {
                        required_joltage.push(s.replace("{", "").replace("}", "").split(",").map(|c| c.parse::<u64>().unwrap()).collect::<Vec<u64>>())
                    }
                }
            });
            button_presses.push(c_presses);
        });

    let parse_duration = parse_time.elapsed();
    let calc_time = Instant::now();
    let part1_time = Instant::now();

    let part1 = button_presses.iter().enumerate().map(|(i, button_presses)| {
        part1(button_presses, solutions[i])
    }).sum::<u64>();
    let par1_duration = part1_time.elapsed();

    let part2_time = Instant::now();
    let part2 = required_joltage.iter().enumerate().map(|(i, required_joltage)| {
        part2(&*button_presses[i], required_joltage)
    }).inspect(|c| println!("{c}")).sum::<u64>();

    let part2_duration = part2_time.elapsed();
    let calc_duration = calc_time.elapsed();
    let total_duration = total_time.elapsed();

    println!("Part 1 ({:?}): {}", par1_duration, part1);
    println!("Part 2 ({:?}): {}", part2_duration, part2);

    println!(
        "Perf - Total: {:?}, Parsing: {:?}, Calculation total: {:?}",
        total_duration, parse_duration, calc_duration
    );
}
