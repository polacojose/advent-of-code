mod checksum;

use std::fs;

use crate::checksum::{find_common_in_pair, gen_checksum};

fn main() {
    let ids = read_ids();
    part1(&ids);
    part2(&ids);
}

fn read_ids() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

fn part1(ids: &Vec<String>) {
    println!("Checksum: {}", gen_checksum(ids));
}

fn part2(ids: &Vec<String>) {
    println!("Common in pair: {:?}", find_common_in_pair(ids));
}
