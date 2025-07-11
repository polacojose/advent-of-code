use std::fs;

use day15::RobotMover;

fn main() {
    let mut rm = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<RobotMover>()
        .unwrap();
    rm.solve_movements();
    println!("Part1: {}", rm.gps_sum());
}
