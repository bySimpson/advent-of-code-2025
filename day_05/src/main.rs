
use clap::{arg, Parser};
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
    let part1_time = Instant::now();

    let part1 = food.iter().filter(|c_food| {
        for c_range in &good_food_ranges {
            if c_range.contains(c_food) {
                return true;
            }
        }
        false
    }).count();

    let par1_duration = part1_time.elapsed();

    let part2_time = Instant::now();
    let mut ranges_sorted= good_food_ranges;
    ranges_sorted.sort_by_key(|i| *i.start());

    let mut merged = vec![];

    let mut current = ranges_sorted[0].clone();

    // Go through sorted ist and always compare start next start positions with the next ending.
    // Ranges overlapping multiple times will be caught in the next iteration.
    for next in ranges_sorted.into_iter().skip(1) {
        // +1 since inclusive
        if *next.start() <= *current.end() + 1 {
            // Overlapping - merge them
            current = *current.start()..=(*current.end().max(next.end()));
        } else {
            // No overlap - push current and start new range
            merged.push(current);
            current = next;
        }
    }
    merged.push(current);

    let part2 = merged.iter().map(|c_iter| {
        c_iter.end() - c_iter.start() + 1
    }).sum::<u64>();

    let part2_duration = part2_time.elapsed();
    let calc_duration = calc_time.elapsed();
    let total_duration = total_time.elapsed();

    println!("Part 1 ({:?}): {}", par1_duration, part1);
    println!("Part 2 ({:?}): {}", part2_duration, part2);

    println!("Perf - Total: {:?}, Parsing: {:?}, Calculation total: {:?}", total_duration, parse_duration, calc_duration);
}