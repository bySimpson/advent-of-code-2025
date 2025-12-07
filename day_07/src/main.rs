mod field;

use crate::field::Field;
use clap::{Parser, arg};
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

fn get_field_at_pos(grid: &[Vec<Field>], x: usize, y: usize) -> Field {
    grid[y][x]
}

fn set_field_at_pos(grid: &mut [Vec<Field>], x: usize, y: usize, field: Field) {
    grid[y][x] = field;
}

fn enumerate_above(grid: &[Vec<Field>], x: usize, y: usize) -> Field {
    // row above
    if y != 0 {
        return get_field_at_pos(grid, x, y - 1);
    }
    Field::Space
}

fn part1(grid: &mut [Vec<Field>]) -> usize {
    let mut splits = 0;
    let len_x = grid[0].len();
    let len_y = grid.len();
    for y in 0..len_y {
        for x in 0..len_x {
            let above_field = enumerate_above(grid, x, y);
            if let Field::Beam(prev_paths) = above_field {
                let current_field = get_field_at_pos(grid, x, y);
                match current_field {
                    Field::Splitter => {
                        if x != 0 {
                            set_field_at_pos(grid, x - 1, y, Field::Beam(prev_paths));
                        }
                        if x < grid[0].len() {
                            set_field_at_pos(grid, x + 1, y, Field::Beam(prev_paths));
                        }
                        splits += 1;
                    }
                    Field::Space => {
                        set_field_at_pos(grid, x, y, Field::Beam(1));
                    },
                    _ => ()
                }
            }
        }
    }
    splits
}

fn part2(grid: &mut [Vec<Field>]) -> u64 {
    let len_x = grid[0].len();
    let len_y = grid.len();
    for y in 0..len_y {
        for x in 0..len_x {
            let above_field = enumerate_above(grid, x, y);
            if let Field::Beam(prev_paths) = above_field {
                let current_field = get_field_at_pos(grid, x, y);
                match current_field {
                    Field::Splitter => {
                        if x != 0 {
                            if let Field::Beam(merger) = get_field_at_pos(grid, x - 1, y) {
                                set_field_at_pos(grid, x - 1, y, Field::Beam(prev_paths + merger));
                            } else {
                                set_field_at_pos(grid, x - 1, y, Field::Beam(prev_paths));
                            }
                        }
                        if x < grid[0].len() {
                            if let Field::Beam(merger) = get_field_at_pos(grid, x + 1, y) {
                                set_field_at_pos(grid, x + 1, y, Field::Beam(prev_paths + merger));
                            } else {
                                set_field_at_pos(grid, x + 1, y, Field::Beam(prev_paths));
                            }
                        }
                    }
                    Field::Space => {
                        set_field_at_pos(grid, x, y, Field::Beam(prev_paths));
                    },
                    Field::Beam(merger) => {
                        set_field_at_pos(grid, x, y, Field::Beam(prev_paths + merger));
                    }
                }
            }
        }
    }
    let mut out = 0;
    for i in 0..grid[0].len() {
        if let Field::Beam(add) = grid.last().unwrap()[i] {
            out += add;
        }
    }
    out
}

fn main() {
    let total_time = Instant::now();

    let args = Args::parse();
    let parse_time = Instant::now();
    let read = fs::read_to_string(args.path).unwrap();

    let mut grid_part1 = read
        .lines()
        .map(|line| line.chars().map(Field::new).collect::<Vec<Field>>())
        .collect::<Vec<Vec<Field>>>();

    let mut grid_part2 = grid_part1.clone();

    let parse_duration = parse_time.elapsed();
    let calc_time = Instant::now();
    let part1_time = Instant::now();

    let part1 = part1(&mut grid_part1);
    let par1_duration = part1_time.elapsed();

    let part2_time = Instant::now();
    let part2 = part2(&mut grid_part2);

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
