use std::fs;

use rects::Claim;

use crate::rects::{non_overlapping_claim_id, overlapping_areas_count};

mod rects;

fn main() {
    let claims: Vec<Claim> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    part1(&claims);
    part2(&claims);
}

fn part1(claims: &Vec<Claim>) {
    println!("Overlapping Areas: {}", overlapping_areas_count(claims))
}

fn part2(claims: &Vec<Claim>) {
    println!(
        "Non-Overlapping Claim: {:?}",
        non_overlapping_claim_id(claims)
    )
}
