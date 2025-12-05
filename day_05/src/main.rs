
use clap::{arg, Parser};
use itertools::Itertools;
use std::fs;
use std::ops::ControlFlow;
use std::ops::ControlFlow::Continue;
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

    let mut good_food_ranges = Vec::new();
    let mut food = Vec::new();

    let mut range_mode = true;
    let _: ControlFlow<()> = read.lines().try_for_each(|line| {
        if line.is_empty() {
            range_mode = false;
            return Continue(());
        }
        if range_mode {
            let mut split = line.split("-");
            good_food_ranges.push(split.next().unwrap().parse::<u64>().unwrap()..=split.next().unwrap().parse::<u64>().unwrap());
        } else {
            food.push(line.parse::<u64>().unwrap());
        }
        return Continue(());
    });


    let parse_duration = parse_time.elapsed();
    let calc_time = Instant::now();

    let part1 = food.iter().filter(|c_food| {
        for c_range in &good_food_ranges {
            if c_range.contains(c_food) {
                return true;
            }
        }
        false
    }).count();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", 0);

    let calc_duration = calc_time.elapsed();

    let total_duration = total_time.elapsed();
    println!("Perf - Total: {:?}, Parsing: {:?}, Calculation: {:?}", total_duration, parse_duration, calc_duration);
}