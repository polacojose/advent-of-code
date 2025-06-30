use std::fs;

use day06::map::{
    barriertester::barrier_solve,
    map::{Map, SolveCompletion},
};

fn main() {
    let map: Map = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    println!("Part1: {}", map.clone().num_walked_positions());

    let count = barrier_solve(&map)
        .into_iter()
        .filter(|sc| matches!(sc, SolveCompletion::InALoop))
        .count();
    println!("Part2: {}", count);
}
