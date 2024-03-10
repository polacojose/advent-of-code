use std::fs;

use lcmx::lcmx;
use network::{Dir, Network};

mod network;

fn main() {
    part1();
    part2();
}

fn part1() {
    let instructions = "LLLLRLRLRRLRRRLRRLRLRRLRLLRRRLRRLRRRLRLLLRLRRLRLLRRRLRRLRLRRLLRRRLRRRLRLRRLRRRLRRLRRLLRRRLLLLRRLRRLRRLRRRLLLRLRLRLRRLRRRLRLRRRLRLRRRLRRLRRLLRRLLRLRRRLRLRRRLLLRLRRRLRLRRRLRRLRLRRLRRRLRRRLRRLLLRRRLRRLRRLRRLRRRLLLRRLRLRRRLLLLRRRLRRLRRRLLRLRLRRLLRRRLLRLRLRLRRLRRLRRRLRRLLRLRRLRRLLLLRRLRLRRLLRRLLRRLRRLRRRLLLRRRR".chars().map(|c|c.to_string().parse::<Dir>().unwrap()).collect::<Vec<_>>();

    let network = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Network>()
        .unwrap();

    println!("Steps {:?}", network.steps_from_single_start(&instructions));
}

fn part2() {
    let instructions = "LLLLRLRLRRLRRRLRRLRLRRLRLLRRRLRRLRRRLRLLLRLRRLRLLRRRLRRLRLRRLLRRRLRRRLRLRRLRRRLRRLRRLLRRRLLLLRRLRRLRRLRRRLLLRLRLRLRRLRRRLRLRRRLRLRRRLRRLRRLLRRLLRLRRRLRLRRRLLLRLRRRLRLRRRLRRLRLRRLRRRLRRRLRRLLLRRRLRRLRRLRRLRRRLLLRRLRLRRRLLLLRRRLRRLRRRLLRLRLRRLLRRRLLRLRLRLRRLRRLRRRLRRLLRLRRLRRLLLLRRLRLRRLLRRLLRRLRRLRRRLLLRRRR".chars().map(|c|c.to_string().parse::<Dir>().unwrap()).collect::<Vec<_>>();

    let network = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Network>()
        .unwrap();

    println!(
        "Steps {:?}",
        lcmx(&network.steps_from_all_starts(&instructions)).unwrap()
    );
}
