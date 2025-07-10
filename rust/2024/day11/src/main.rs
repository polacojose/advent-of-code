use std::{collections::HashMap, fs};

use day11::blink::{Stone, blink_n};

fn main() {
    println!("Part1 {}", num_stones(25));
    println!("Part2 {}", num_stones(75));
}

fn num_stones(i: u32) -> usize {
    let stones = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| Stone(s.trim().parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    let mut stone_map = HashMap::new();
    let total_stones = stones
        .into_iter()
        .map(|s| blink_n(i, &s, &mut stone_map))
        .sum::<usize>();
    total_stones
}
