use std::fs;

use inflator::Inflator;

mod inflator;

fn main() {
    let chars = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .chars()
        .collect::<Vec<_>>();
    part1(chars.as_slice());
    part2(chars.as_slice());
}

fn part1(input: &[char]) {
    let mut inflator = Inflator::new(input, false);
    println!("Part1: {}", inflator.count_inflated());
}

fn part2(input: &[char]) {
    let mut inflator = Inflator::new(input, true);
    println!("Part2: {}", inflator.count_inflated());
}
