use std::fs;

use network::{Dir, Network};

mod network;

fn main() {
    part1();
}

fn part1() {
    let instructions = "LLLLRLRLRRLRRRLRRLRLRRLRLLRRRLRRLRRRLRLLLRLRRLRLLRRRLRRLRLRRLLRRRLRRRLRLRRLRRRLRRLRRLLRRRLLLLRRLRRLRRLRRRLLLRLRLRLRRLRRRLRLRRRLRLRRRLRRLRRLLRRLLRLRRRLRLRRRLLLRLRRRLRLRRRLRRLRLRRLRRRLRRRLRRLLLRRRLRRLRRLRRLRRRLLLRRLRLRRRLLLLRRRLRRLRRRLLRLRLRRLLRRRLLRLRLRLRRLRRLRRRLRRLLRLRRLRRLLLLRRLRLRRLLRRLLRRLRRLRRRLLLRRRR".chars().map(|c|c.to_string().parse::<Dir>().unwrap()).collect();

    let network = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Network>()
        .unwrap();

    println!("Steps {}", network.steps(instructions));
}
