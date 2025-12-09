mod orientation;

use clap::{Parser, arg};
use itertools::Itertools;
use std::collections::HashSet;
use rayon::prelude::*;
use std::fs;
use std::string::String;
use std::time::Instant;
use crate::orientation::Orientation;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool,
}

fn cube_area(a: &(u64, u64), b: &(u64, u64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

fn part1(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .combinations(2)
        .map(|combinations| {
            let p1 = combinations[0];
            let p2 = combinations[1];
            cube_area(p1, p2)
        }).max().unwrap()
}

fn point_within(rectangle: ((u64, u64), (u64, u64)), point: (u64, u64)) -> bool {
    let left_x = rectangle.0.0.min(rectangle.1.0);
    let right_x = rectangle.0.0.max(rectangle.1.0);
    let top_y = rectangle.0.1.max(rectangle.1.1);
    let bottom_y = rectangle.0.1.min(rectangle.1.1);
    point.0 > left_x
        && point.0 < right_x
        && point.1 > bottom_y
        && point.1 < top_y
}

fn part2(input: &mut [(u64, u64)]) -> u64 {
    let mut connections: Vec<((u64, u64), (u64, u64))> = Vec::new();
    // get all in
    input.windows(2).for_each(|window| {
        connections.push((window[0], window[1]));
    });
    // add last connection to finish area
    connections.push((*input.first().unwrap(), *input.last().unwrap()));

    let mut tiles: HashSet<(u64, u64)> = HashSet::new();
    connections.iter().for_each(|(a, b)| {
       let alignment = Orientation::from_line((*a, *b));
        match alignment {
            Orientation::Horizontal => {
                let min_x = a.0.min(b.0) as usize;
                let max_x = a.0.max(b.0) as usize;
                (min_x..=max_x).for_each(|x| {
                    tiles.insert((x as u64, a.1));
                })
            }
            Orientation::Vertical => {
                let min_y = a.1.min(b.1) as usize;
                let max_y = a.1.max(b.1) as usize;
                (min_y..=max_y).for_each(|y| {
                    tiles.insert((a.0, y as u64));
                })
            }
        }
    });

    input
        .iter()
        .combinations(2)
        .par_bridge()
        .filter(|combinations| {
            let p1 = combinations[0];
            let p2 = combinations[1];
            !tiles.iter().any(|point| {
                point_within((*p1, *p2), *point)
            })
        }).map(|combinations| {
        let p1 = combinations[0];
        let p2 = combinations[1];
        cube_area(p1, p2)
    }).max().unwrap()
}

fn main() {
    let total_time = Instant::now();

    let args = Args::parse();
    let parse_time = Instant::now();
    let read = fs::read_to_string(args.path).unwrap();

    let mut coords = read
        .lines()
        .map(|line| {
            let mut coords = line.split(",");
            let x = coords.next().unwrap().parse::<u64>().unwrap();
            let y = coords.next().unwrap().parse::<u64>().unwrap();
            (x, y)
        })
        .collect::<Vec<(u64, u64)>>();

    let parse_duration = parse_time.elapsed();
    let calc_time = Instant::now();
    let part1_time = Instant::now();

    let part1 = part1(&coords);
    let par1_duration = part1_time.elapsed();

    let part2_time = Instant::now();
    let part2 = part2(&mut coords);

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
