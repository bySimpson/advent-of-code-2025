use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use clap::{Parser, arg};
use std::fs;
use std::iter::Map;
use std::string::String;
use std::time::Instant;
use ordered_float::OrderedFloat;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool,
}

fn euclidian_distance(a: &(u64, u64, u64), b: &(u64, u64, u64)) -> f64 {
    let dx = a.0 as f64 - b.0 as f64;
    let dy = a.1 as f64 - b.1 as f64;
    let dz = a.2 as f64 - b.2 as f64;

    (dx*dx + dy*dy + dz*dz).sqrt()
}

fn part1(input: &mut [(u64, u64, u64)], connections: usize) -> u64 {
    let mut out = 0;
    let mut cluster_amounts: Vec<u64> = Vec::new();
    // coords - cluster number
    let mut connected_boxes: HashMap<(u64, u64, u64), u64> = HashMap::new();

    input.iter().combinations(2).sorted_by_key(|(combinations)| {
        let p1 = combinations[0];
        let p2 = combinations[1];
        OrderedFloat(euclidian_distance(p1, p2))
    }).get(0..connections)
        .for_each(|combinations| {
            let p1 = combinations[0];
            let p2 = combinations[1];
            if !connected_boxes.contains_key(p1) && !connected_boxes.contains_key(p2) {
                // both not found, new cluster group
                cluster_amounts.push(2);
                let index = cluster_amounts.len() as u64 - 1;
                connected_boxes.insert(*p1, index);
                connected_boxes.insert(*p2, index);
            } else if connected_boxes.contains_key(p1) && connected_boxes.contains_key(p2) {
                // both exist, connect clusters
                let p1_index = connected_boxes[p1];
                let p2_index = connected_boxes[p2];
                if p1_index != p2_index {
                    connected_boxes.iter_mut().filter(|(_, group)| **group == p2_index).for_each(|(_, group)| {
                        *group = p1_index;
                    });
                    cluster_amounts[p1_index as usize] += cluster_amounts[p2_index as usize];
                    cluster_amounts[p2_index as usize] = 1;
                }
            } else {
                // one already has cluster group, connect to it
                let index = if connected_boxes.contains_key(p1) {
                    connected_boxes[p1]
                } else {
                    connected_boxes[p2]
                };
                connected_boxes.insert(*p1, index);
                connected_boxes.insert(*p2, index);
                cluster_amounts[index as usize] += 1;
            }
        });
    cluster_amounts.iter().sorted().rev().get(0..3).fold(1, |acc, x| acc * x)
}

fn part2(input: &mut [(u64, u64, u64)]) -> u64 {
    let mut out = 0;
    out
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
            let z = coords.next().unwrap().parse::<u64>().unwrap();
            (x, y, z)
        })
        .collect::<Vec<(u64, u64, u64)>>();

    let parse_duration = parse_time.elapsed();
    let calc_time = Instant::now();
    let part1_time = Instant::now();

    let part1 = part1(&mut coords, 1000);
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
