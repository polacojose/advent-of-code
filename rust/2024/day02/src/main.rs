use std::fs;

use day02::report::{RawReport, Safety};

fn main() {
    let reports: Vec<RawReport> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part1(reports.clone());
    part2(reports.clone());
}

fn part1(reports: Vec<RawReport>) {
    println!(
        "Part1: {}",
        reports
            .into_iter()
            .map(|mut r| r.process_report(false))
            .filter(|r| matches!(r.safety, Safety::Safe))
            .count()
    )
}

fn part2(reports: Vec<RawReport>) {
    println!(
        "Part2: {}",
        reports
            .into_iter()
            .map(|mut r| r.process_report(true))
            .filter(|r| matches!(r.safety, Safety::Safe))
            .count()
    )
}
