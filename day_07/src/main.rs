mod field;

use crate::field::Field;
use clap::{Parser, arg};
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
    debug: bool,
}

fn get_field_at_pos(grid: &Vec<Vec<Field>>, x: usize, y: usize) -> Field {
    grid[y][x]
}

fn set_field_at_pos(grid: &mut Vec<Vec<Field>>, x: usize, y: usize, field: Field) {
    grid[y][x] = field;
}

fn enumerate_above(grid: &Vec<Vec<Field>>, x: usize, y: usize) -> Field {
    // row above
    if y != 0 {
        return get_field_at_pos(grid, x, y - 1);
    }
    Field::Space
}

fn part1(grid: &Vec<Vec<Field>>) -> usize {
    let mut splits = 0;
    let mut current = grid.clone();
    // first row can be skipped since there is nothing to simulate
    for (y, line) in grid.iter().skip(1).enumerate() {
        for (x, field) in line.iter().enumerate() {
            let above_field = enumerate_above(&current, x, y);
            if above_field == Field::Beam {
                let current_field = get_field_at_pos(&current, x, y);
                match current_field {
                    Field::Beam => (),
                    Field::Splitter => {
                        if x != 0 {
                            set_field_at_pos(&mut current, x - 1, y, Field::Beam);
                        }
                        if x < grid[0].len() {
                            set_field_at_pos(&mut current, x + 1, y, Field::Beam);
                        }
                        splits += 1;
                    }
                    Field::Space => {
                        set_field_at_pos(&mut current, x, y, Field::Beam);
                    }
                }
            }
        }
    }

    for (y, line) in grid.iter().skip(1).enumerate() {
        for (x, field) in line.iter().enumerate() {
            print!("{}", get_field_at_pos(&current, x, y));
        }
        print!("\n");
    }
    splits
}

fn part2(grid: &mut Vec<Vec<Field>>) -> usize {
    let mut total = 0;
    total
}

fn main() {
    let total_time = Instant::now();

    let args = Args::parse();
    let parse_time = Instant::now();
    let read = fs::read_to_string(args.path).unwrap();

    let mut grid = read
        .lines()
        .map(|line| line.chars().map(|c| Field::new(c)).collect::<Vec<Field>>())
        .collect::<Vec<Vec<Field>>>();

    let parse_duration = parse_time.elapsed();
    let calc_time = Instant::now();
    let part1_time = Instant::now();

    let part1 = part1(&mut grid.clone());
    let par1_duration = part1_time.elapsed();

    let part2_time = Instant::now();
    let part2 = part2(&mut grid);

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
