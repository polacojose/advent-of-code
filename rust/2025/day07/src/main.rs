use std::fs;

use day07::{Equation, Operator, find_operators};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    part1();
    part2();
}

fn part1() {
    let p1_operators = [Operator::Add, Operator::Mul];
    let s: u64 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<Equation>().unwrap())
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|e| {
            if let Ok(operands) = find_operators(e.solution, &e.operands, &p1_operators) {
                if !operands.is_empty() {
                    Some(e.solution)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum();

    println!("Part1: {s}");
}

fn part2() {
    let p2_operators = [Operator::Add, Operator::Mul, Operator::Comp];
    let s: u64 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<Equation>().unwrap())
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|e| {
            if let Ok(operands) = find_operators(e.solution, &e.operands, &p2_operators) {
                if !operands.is_empty() {
                    Some(e.solution)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum();

    println!("Part2: {s}");
}
