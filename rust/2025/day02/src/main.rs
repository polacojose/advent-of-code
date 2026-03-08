pub mod detector;
pub mod digit;
pub mod models;
pub mod scanner;

use std::fs;

use crate::{models::IDRange, scanner::invalid_ids_at_in_id_range};

fn main() {
    let ranges: Vec<IDRange> = fs::read_to_string("input")
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let sum = ranges
        .into_iter()
        .map(|r| invalid_ids_at_in_id_range(&r).into_iter().sum::<u64>())
        .sum::<u64>();

    println!("Part 2: {sum}");
}
