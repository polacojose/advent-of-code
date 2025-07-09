use std::{
    fs,
    time::{self},
};

use day14::RobotPlotter;

fn main() {
    //part1();
    let _ = part2();
    //let _ = test();
}

fn part1() {
    let safety = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<RobotPlotter>()
        .unwrap()
        .safety_at_steps(100, 101, 103);

    println!("Part1: {safety}");
}

fn part2() {
    let rp = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<RobotPlotter>()
        .unwrap();

    let n: u64 = 10000;
    let start = time::Instant::now();
    let (steps, output) = rp.get_most_symetrical_at_steps(n, 101, 103);
    //clearscreen::clear().expect("failed to clear screen");
    println!("******{steps} in {n}******");
    println!("{output}");
    let elapsed = start.elapsed().as_micros();
    println!("Micros: {elapsed}");
}
