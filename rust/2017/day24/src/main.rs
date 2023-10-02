use std::fs;

use crate::conn::Connection;

pub mod conn;

fn main() {
    let connections = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<Connection>().unwrap())
        .collect::<Vec<_>>();

    let mut paths = connections
        .iter()
        .filter(|x| x.ports.iter().find(|p| p.pins == 0).is_some())
        .map(|c| vec![c.clone()])
        .collect::<Vec<Vec<Connection>>>();

    let mut results: Vec<(usize, usize)> = Vec::new();

    while let Some(path) = paths.pop() {
        results.push((path.len(), get_bridge_strength(&path)));

        let mut new_paths = connections
            .iter()
            .filter(|c| !path.contains(c))
            .filter(|c| c.can_connect(&path.last().unwrap()))
            .map(|c| {
                let mut new_path = path.clone();
                let mut new_node = c.clone();
                new_path.last_mut().unwrap().connect(&mut new_node).unwrap();
                new_path.push(new_node);
                new_path
            })
            .collect::<Vec<_>>();
        paths.append(&mut new_paths);
    }

    let max_length = results.iter().map(|x| x.0).max().unwrap();
    let max_strength = results.iter().map(|x| x.1).max().unwrap();
    let max_strength_of_max_length = results
        .iter()
        .filter(|x| x.0 == max_length)
        .map(|x| x.1)
        .max()
        .unwrap();

    println!(
        "Max length: {}, max strength: {}, max strength of max length: {}",
        max_length, max_strength, max_strength_of_max_length
    );
}

fn get_bridge_strength(path: &Vec<Connection>) -> usize {
    path.iter()
        .map(|x| x.ports.iter().map(|p| p.pins as usize).sum::<usize>())
        .sum()
}

fn print_bridge(path: &Vec<Connection>) {
    let mut conns = Vec::new();
    for conn in path {
        conns.push(
            conn.ports
                .iter()
                .map(|x| x.pins.to_string())
                .collect::<Vec<_>>()
                .join("/"),
        );
    }
    println!("{:?}", conns.join("--"));
}
