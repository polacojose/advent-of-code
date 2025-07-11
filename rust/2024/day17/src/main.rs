use std::fs;

use day17::{find_comp_copy, Comp};

fn main() {
    part1();
    //part2();
}

fn part1() {
    let mut comp = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Comp>()
        .unwrap();
    let _ = comp.execute();

    println!(
        "Part1: {:?}",
        comp.memory
            .out
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn part2() {
    let comp = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Comp>()
        .unwrap();

    let reg_a = find_comp_copy(&comp);

    println!(
        "Part2: {:?}",
        reg_a
    );
}
