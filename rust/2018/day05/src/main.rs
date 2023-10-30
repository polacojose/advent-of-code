mod reaction;

use std::fs;

use crate::reaction::{solve_reaction_length, solve_shortest_length_with_removal};

fn main() {
    let input_polymer = fs::read_to_string("input.txt").unwrap();
    part1(&input_polymer);
    part2(&input_polymer);
}

fn part1(polymer: impl AsRef<str>) {
    let result = solve_reaction_length(polymer);
    println!("Part1: {}", result);
}

fn part2(polymer: impl AsRef<str>) {
    let result = solve_shortest_length_with_removal(polymer);
    println!("Part2: {}", result);
}
