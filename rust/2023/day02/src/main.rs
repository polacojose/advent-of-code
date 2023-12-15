use std::{collections::HashMap, fs};

use game_result::GameResult;

mod game_result;

fn main() {
    let game_results = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<GameResult>().unwrap())
        .collect::<Vec<_>>();

    part1(&game_results);
    part2(&game_results);
}

fn part1(game_results: &Vec<GameResult>) {
    let mut components = HashMap::new();
    components.insert("red".to_owned(), 12);
    components.insert("green".to_owned(), 13);
    components.insert("blue".to_owned(), 14);

    let mut id_total = 0;
    for g in game_results {
        if g.valid(&components) {
            id_total += g.id;
        }
    }
    println!("Total id num: {}", id_total);
}

fn part2(game_results: &Vec<GameResult>) {
    let mut power_total = 0;
    for g in game_results {
        power_total += g.minimum_component_power();
    }
    println!("Minimum power total: {}", power_total);
}
