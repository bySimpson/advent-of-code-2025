mod field;

use clap::{arg, Parser};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;
use std::string::String;
use std::time::Instant;
use crate::field::Field;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

fn get_field_at_pos(grid: &Vec<Vec<Field>>, x: usize, y: usize) -> Field {
    grid[y][x]
}

fn set_field_at_pos(grid: &mut Vec<Vec<Field>>, x: usize, y: usize, field: Field) {
    grid[y][x] = field;
}

fn enumerate_surroundings(grid: &Vec<Vec<Field>>, x: isize, y: isize) -> u32 {
    let mut paper_rolls = 0u32;
    let max_y = grid.len() as isize - 1;
    let max_x = grid[0].len() as isize - 1;
    // row above
    if y != 0 {
        let c_min_x = if x - 1 >= 0 {x - 1} else {x};
        let c_max_x = if x + 1 <= max_x {x + 1} else {x};
        for c_x in c_min_x..=c_max_x {
            if get_field_at_pos(grid, c_x as usize, y as usize - 1) == Field::Roll {
                paper_rolls += 1;
            }
        }
    }
    // row below
    if y < max_y {
        let c_min_x = if x - 1 >= 0 {x - 1} else {x};
        let c_max_x = if x + 1 <= max_x {x + 1} else {x};
        for c_x in c_min_x..=c_max_x {
            if get_field_at_pos(grid, c_x as usize, y as usize + 1) == Field::Roll {
                paper_rolls += 1;
            }
        }
    }

    if x - 1 >= 0 {
        if get_field_at_pos(grid, x as usize - 1, y as usize) == Field::Roll {
            paper_rolls += 1;
        }
    }

    if x + 1 <= max_x {
        if get_field_at_pos(grid, x as usize + 1, y as usize) == Field::Roll {
            paper_rolls += 1;
        }
    }
    paper_rolls
}

fn part1(grid: &mut Vec<Vec<Field>>) -> usize {
    let mut out = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, field) in line.iter().enumerate() {
            if get_field_at_pos(grid, x, y) == Field::Roll {
                if enumerate_surroundings(grid, x as isize, y as isize) < 4 {
                    out += 1;
                }
            }
        }
    }
    out
}

fn part2(grid: &mut Vec<Vec<Field>>) -> usize {
    let mut total = 0;
    let mut current = grid.clone();
    'outer: loop {
        let mut next = current.clone();
        let mut out = 0;
        for (y, line) in grid.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if get_field_at_pos(&mut current, x, y) == Field::Roll {
                    if enumerate_surroundings(&mut current, x as isize, y as isize) < 4 {
                        out += 1;
                        set_field_at_pos(&mut next, x, y, Field::Air);
                    }
                }
            }
        }
        total += out;
        if out == 0 {
            break 'outer;
        }
        current = next;
    }
    total
}

fn main() {
    let total_time = Instant::now();

    let args = Args::parse();
    let parse_time = Instant::now();
    let read = fs::read_to_string(args.path).unwrap();

    let mut input = read.lines().map(|line| {
        line.chars().map(|c| Field::new(c)).collect::<Vec<Field>>()
    }).collect::<Vec<Vec<Field>>>();

    let parse_duration = parse_time.elapsed();

    let calc_time = Instant::now();

    println!("Part 1: {}", part1(&mut input.clone()));
    println!("Part 2: {}", part2(&mut input));

    let calc_duration = calc_time.elapsed();

    let total_duration = total_time.elapsed();
    println!("Perf - Total: {:?}, Parsing: {:?}, Calculation: {:?}", total_duration, parse_duration, calc_duration);
}