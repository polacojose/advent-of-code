use lazy_static::lazy_static;
use part01::{Distance, DistanceMap};
use std::{collections::HashSet, fs, str::FromStr};

const INPUT_FILE: &str = "input.txt";

lazy_static! {
    #[derive(Debug)]
    static ref LOCATIONS: HashSet::<String> = {
    let mut locations = HashSet::new();
        let file_string = fs::read_to_string(INPUT_FILE).unwrap();
    for line in file_string.lines() {
        let distance = Distance::from_str(line).unwrap();
        locations.insert(distance.source.clone());
        locations.insert(distance.destination.clone());
    }
    locations
    };
}

lazy_static! {
    #[derive(Debug)]
    static ref MAP: DistanceMap = {
        let mut map = DistanceMap::new();
        let file_string = fs::read_to_string(INPUT_FILE).unwrap();
        for line in file_string.lines() {
            let distance = Distance::from_str(line).unwrap();
            map.insert(distance);
        }
        map
    };
}

fn main() {
    let file_string = fs::read_to_string(INPUT_FILE).unwrap();

    let mut cities = HashSet::new();
    for line in file_string.lines() {
        let distance = Distance::from_str(line).unwrap();
        cities.insert(distance.source.clone());
        cities.insert(distance.destination.clone());
    }

    let mut results = Vec::new();
    for city in cities {
        let mut visited = Vec::new();
        visited.push(city.clone());
        let result = find_paths(city, visited, 0);
        results.push(result);
    }

    let mut results = results
        .iter()
        .map(|o| o.clone().unwrap())
        .collect::<Vec<(Vec<String>, u32)>>();
    results.sort_by(|a, b| a.1.cmp(&b.1));

    for result in results {
        println!("{:?}", result);
    }
}

fn find_paths(city: String, visited: Vec<String>, length: u32) -> Option<(Vec<String>, u32)> {
    let connected_cities = MAP.get(&city);

    let mut min_trip = Vec::new();
    let mut min_path = u32::MAX;
    for (connected_city, l) in connected_cities {
        if !visited.contains(&connected_city) {
            let mut visited = visited.clone();
            visited.push(connected_city.clone());
            let combined_length = length + l;
            if visited.len() == LOCATIONS.len() {
                if combined_length < min_path {
                    min_path = combined_length;
                    min_trip = visited.clone();
                }
            }

            let val = find_paths(connected_city, visited.clone(), combined_length);

            if let Some(v) = val {
                if v.1 < min_path {
                    min_path = v.1;
                    min_trip = v.0;
                }
            }
        }
    }

    if min_path != u32::MAX {
        Some((min_trip, min_path))
    } else {
        None
    }
}
