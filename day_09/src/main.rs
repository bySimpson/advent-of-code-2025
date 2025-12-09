mod orientation;

use clap::{Parser, arg};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::fs;
use std::ops::ControlFlow;
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

fn cross(a: (u64, u64), b: (u64, u64)) -> u64 {
    //v.x * w.y - v.y * w.x
    a.0 * b.1 - a.1 * b.0
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

fn lines_intersect(line_a: ((u64, u64), (u64, u64)), line_b: ((u64, u64), (u64, u64))) -> bool {
    let orientation_a = Orientation::from_line(line_a);
    let orientation_b = Orientation::from_line(line_b);
    if orientation_a == orientation_b {
        return false;
    }

    match orientation_a {

        Orientation::Horizontal => {
            let line_b_min_y = line_b.0.1.min(line_b.1.1);
            let line_b_max_y = line_b.0.1.max(line_b.1.1);

            let line_a_min_x = line_a.0.0.min(line_a.1.0);
            let line_a_max_x = line_a.0.0.max(line_a.1.0);

            line_a.0.1 > line_b_min_y && line_a.0.1 < line_b_max_y && line_b.0.0 > line_a_min_x && line_b.0.0 < line_a_max_x
        }
        Orientation::Vertical => {
            let line_b_min_x = line_b.0.0.min(line_b.1.0);
            let line_b_max_x = line_b.0.0.max(line_b.1.0);

            let line_a_min_y = line_b.0.1.min(line_b.1.1);
            let line_a_max_y = line_b.0.1.max(line_b.1.1);
            line_a.0.0 > line_b_min_x && line_a.0.0 < line_b_max_x && line_b.0.1 > line_a_min_y && line_b.0.1 < line_a_max_y
        }
    }
}

fn part2(input: &mut [(u64, u64)]) -> u64 {
    let mut connections: Vec<((u64, u64), (u64, u64))> = Vec::new();
    // get all in
    input.windows(2).for_each(|window| {
        connections.push((window[0], window[1]));
    });
    // add last connection to finish area
    connections.push((*input.first().unwrap(), *input.last().unwrap()));

    input
        .iter()
        .combinations(2)
        .filter(|combinations| {
            let p1 = combinations[0];
            let p2 = combinations[1];

            //borders do not matter, make rectangle smaller by 1
            let bottom_left = (p1.0.min(p2.0) + 1, p1.1.min(p2.1) + 1);
            let top_left = (p1.0.min(p2.0) + 1, p1.1.max(p2.1) - 1);
            let bottom_right = (p1.0.max(p2.0) - 1, p1.1.min(p2.1) + 1);
            let top_right = (p1.0.max(p2.0) - 1, p1.1.max(p2.1) - 1);

            !connections.iter().any(|(first, second)| {
                lines_intersect((bottom_left, bottom_right), (*first, *second)) ||
                lines_intersect((top_left, top_right), (*first, *second)) ||
                lines_intersect((bottom_left, top_left), (*first, *second)) ||
                lines_intersect((bottom_right, top_right), (*first, *second))
            })
        }).inspect(|x| println!("{:?}", x)).map(|combinations| {
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
    println!("Part 2 ({:?}): {}", part2_duration, part2); // 3664486347 - too low

    println!(
        "Perf - Total: {:?}, Parsing: {:?}, Calculation total: {:?}",
        total_duration, parse_duration, calc_duration
    );
}
