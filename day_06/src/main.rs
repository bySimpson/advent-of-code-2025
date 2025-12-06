mod operation;

use clap::{arg, Parser};
use std::fs;
use std::ops::ControlFlow;
use std::ops::ControlFlow::Continue;
use std::string::String;
use std::time::Instant;
use crate::operation::{Alignment, Operation};

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

    let mut operations = Vec::new();
    let operations_len = read.lines().next().unwrap().split_whitespace().count();
    let len_y = read.lines().count();
    let mut numbers: Vec<Vec<u64>>  = vec![Vec::new(); operations_len];
    read.lines().enumerate().for_each(|(iter, line)| {
        let pieces = line.split_whitespace();
        if iter == len_y - 1 {
            for piece in pieces {
                operations.push(Operation::new(piece));
            }
        } else {
            for (i, piece) in pieces.enumerate() {
                numbers[i].push(piece.parse::<u64>().unwrap());
            }
        }
    });

    //preparations for part 2
    let mut operation_alignment = Vec::new();
    let _: ControlFlow<()> = read.lines().last().unwrap().chars().enumerate().try_for_each(|(iter, char)| {
        if char.is_whitespace() {
            return Continue(());
        }
        for (line_nmbr, c_line) in read.lines().enumerate() {
            if line_nmbr == len_y - 1 {
                // Last line, left alignment
                operation_alignment.push(Alignment::Left);
                break;
            }
            if c_line.as_bytes()[iter] == " ".as_bytes()[0] {
                operation_alignment.push(Alignment::Right);
                break;
            }
        }
        Continue(())
    });


    let parse_duration = parse_time.elapsed();
    let calc_time = Instant::now();
    let part1_time = Instant::now();

    let part1 = numbers.iter().enumerate().map(|(iter, numbers)| {
        let operation = operations.get(iter).unwrap();
        operation.calculate_part_01(numbers)
    }).sum::<u64>();

    let par1_duration = part1_time.elapsed();

    let part2_time = Instant::now();

    let part2 = numbers.iter().enumerate().map(|(iter, numbers)| {
        let operation = operations.get(iter).unwrap();
        operation.calculate_part_02(numbers, operation_alignment[iter])
    }).sum::<u64>();

    let part2_duration = part2_time.elapsed();
    let calc_duration = calc_time.elapsed();
    let total_duration = total_time.elapsed();

    println!("Part 1 ({:?}): {}", par1_duration, part1);
    println!("Part 2 ({:?}): {}", part2_duration, part2);

    println!("Perf - Total: {:?}, Parsing: {:?}, Calculation total: {:?}", total_duration, parse_duration, calc_duration);
}