use std::fs;

use day13::smparse::ClawSolver;

fn main() {
    part1();
    part2();
}

fn part1() {
    let total = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| s.parse::<ClawSolver>().unwrap())
        .filter_map(|sm| {
            sm.solve(0.0)
                .map(|x| x.into_iter().map(|(a, b)| a * 3 + b).min())?
        })
        .sum::<u64>();

    println!("Part1: {total}");
}

fn part2() {
    let total = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| s.parse::<ClawSolver>().unwrap())
        .filter_map(|sm| {
            sm.solve(10000000000000.0)
                .map(|x| x.into_iter().map(|(a, b)| a * 3 + b).min())?
        })
        .sum::<u64>();

    println!("Part2: {total}");
}
