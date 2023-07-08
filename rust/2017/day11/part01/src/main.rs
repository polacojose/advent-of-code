use std::fs;

use part01::{Dir, HexCoord};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut furthest_distance = 0;
    for line in file_string.lines() {
        let directions = line
            .split(",")
            .map(|s| s.parse::<Dir>().unwrap())
            .collect::<Vec<_>>();

        let origin = HexCoord::new(0, 0, 0);
        let mut navigation = HexCoord::new(0, 0, 0);
        for dir in directions {
            navigation.shift(&dir);

            let distance = navigation.distance(origin);
            furthest_distance = furthest_distance.max(distance);
        }

        println!("Location {:?}", navigation);
        println!("Final Distance: {}", navigation.distance(origin));
        println!("Furthest Distance: {}", furthest_distance);
    }
}
