use std::{collections::HashSet, fs};

fn get_pairs() -> HashSet<(usize, usize)> {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut pairs = HashSet::new();

    for line in file_string.lines() {
        let (source, destinations) = line.split_once("<->").unwrap();
        let source_number = source.trim().parse::<usize>().unwrap();
        let destinations = destinations
            .split(",")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        for dest_number in destinations {
            pairs.insert((source_number, dest_number));
            pairs.insert((dest_number, source_number));
        }
    }

    pairs
}

fn unique_ids(space: &HashSet<(usize, usize)>) -> HashSet<usize> {
    let mut unique = HashSet::new();
    for set in space {
        unique.insert(set.0);
        unique.insert(set.1);
    }
    unique
}

fn main() {
    let space = get_pairs();

    let mut unique_ids = unique_ids(&space).into_iter().collect::<Vec<_>>();

    let mut groups = Vec::new();
    while let Some(id) = unique_ids.pop() {
        let connected = get_connection(id, HashSet::from_iter(vec![id]), &space);

        for connection in &connected {
            if let Some(position) = unique_ids.iter().position(|x| x == connection) {
                unique_ids.remove(position);
            }
        }

        groups.push(connected);
    }

    println!("Total Groups: {}", groups.len());
}

fn get_connection(
    id: usize,
    mut connected: HashSet<usize>,
    space: &HashSet<(usize, usize)>,
) -> HashSet<usize> {
    for set in space {
        if set.1 == id && !connected.contains(&set.0) {
            connected.insert(set.0);
            for connection in get_connection(set.0, connected.clone(), space) {
                connected.insert(connection);
            }
        }
        if set.0 == id && !connected.contains(&set.1) {
            connected.insert(set.1);
            for connection in get_connection(set.1, connected.clone(), space) {
                connected.insert(connection);
            }
        }
    }

    return connected;
}
