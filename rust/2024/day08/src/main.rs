use std::fs;

use day08::map::map::Map;

fn main() {
    let antenna_map = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Map>()
        .unwrap();

    let antinode_count = antenna_map.num_antinodes(false);
    println!("Part1: {antinode_count}");

    let antinode_count = antenna_map.num_antinodes(true);
    println!("Part2: {antinode_count}");
}
