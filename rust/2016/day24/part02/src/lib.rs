pub mod models;

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use models::{Node, NodeType, Vector};

pub fn gen_map(file_contents: String) -> Vec<Vec<Node>> {
    let mut map = Vec::new();
    for (y, line) in file_contents.lines().into_iter().enumerate() {
        let mut map_line = Vec::new();
        for (x, character) in line.chars().into_iter().enumerate() {
            map_line.push(Node {
                node_type: character.into(),
                position: Vector {
                    x: x as isize,
                    y: y as isize,
                },
            });
        }
        map.push(map_line);
    }
    map
}

pub fn find_steps<'a>(input_map: &'a Vec<Vec<Node>>, from: &Node, to: &Node) -> Result<usize, ()> {
    let mut working_set: VecDeque<(usize, &Node)> = VecDeque::from_iter([(0, from)]);
    let mut visited: HashSet<&Node> = Default::default();
    visited.insert(from);

    while let Some((steps, current_node)) = working_set.pop_back() {
        if current_node == to {
            return Ok(steps);
        }

        for neighbor in get_neighbors(current_node, input_map)
            .into_iter()
            .filter(|n| !visited.contains(n))
            .collect::<Vec<_>>()
        {
            working_set.push_front((steps + 1, neighbor));
            visited.insert(neighbor);
        }
    }

    Err(())
}

pub fn solve(map: &Vec<Vec<Node>>) -> Result<usize, ()> {
    let first_node = get_start_node(map).unwrap();
    let priority_nodes = get_non_zero_nodes(map);

    let distances_from_zero = priority_nodes
        .iter()
        .map(|node| (find_steps(map, first_node, node).unwrap()))
        .collect::<Vec<_>>();

    let mut distances: Vec<Vec<_>> = vec![vec![0; priority_nodes.len()]; priority_nodes.len()];
    for from_node_index in 0..priority_nodes.len() - 1 {
        for to_node_index in from_node_index + 1..priority_nodes.len() {
            let result = find_steps(
                map,
                priority_nodes[from_node_index],
                priority_nodes[to_node_index],
            )
            .unwrap();
            distances[from_node_index][to_node_index] = result;
            distances[to_node_index][from_node_index] = result;
        }
    }

    let mut min_steps = usize::MAX;
    for path in (0..priority_nodes.len())
        .into_iter()
        .permutations(priority_nodes.len())
    {
        let mut distance = distances_from_zero[path[0]];

        distance += path
            .windows(2)
            .map(|vals| {
                let result = distances[vals[0]][vals[1]];
                result
            })
            .sum::<usize>();

        distance += distances_from_zero[*path.iter().last().unwrap()];

        min_steps = distance.min(min_steps);
    }

    if min_steps < usize::MAX {
        Ok(min_steps)
    } else {
        Err(())
    }
}

#[inline]
fn get_start_node(input_map: &Vec<Vec<Node>>) -> Option<&Node> {
    input_map
        .iter()
        .flatten()
        .find(|node| node.node_type == NodeType::PointOfInterest(0))
}

#[inline]
fn get_non_zero_nodes(input_map: &Vec<Vec<Node>>) -> Vec<&Node> {
    input_map
        .iter()
        .flatten()
        .filter(|node| {
            node.node_type != NodeType::PointOfInterest(0)
                && match node.node_type {
                    NodeType::PointOfInterest(_) => true,
                    _ => false,
                }
        })
        .collect()
}

#[inline]
fn get_neighbors<'a>(node: &Node, map: &'a Vec<Vec<Node>>) -> Vec<&'a Node> {
    [
        Vector { x: 1, y: 0 },
        Vector { x: -1, y: 0 },
        Vector { x: 0, y: 1 },
        Vector { x: 0, y: -1 },
    ]
    .into_iter()
    .map(|v| {
        if let Some(y) = map.get((node.position.y + v.y) as usize) {
            y.get((node.position.x + v.x) as usize)
        } else {
            None
        }
    })
    .filter(|O| {
        if let Some(O) = O {
            O.node_type != NodeType::Wall
        } else {
            false
        }
    })
    .map(|x| x.unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{get_map_string, Path};
    use std::{collections::HashSet, fs};

    #[test]
    fn test_map_parse() {
        let result = get_map_string(&gen_map(fs::read_to_string("demo-input.txt").unwrap()));

        let expected_result = "###########\n\
            #0.1.....2#\n\
            #.#######.#\n\
            #4.......3#\n\
            ###########\n";

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_path_next_node() {
        let input_map = gen_map(fs::read_to_string("demo-input.txt").unwrap());
        let path = Path {
            crosses_per_node: Default::default(),
            points_of_interest: HashSet::new(),
            traveled: vec![&Node {
                node_type: NodeType::Space,
                position: Vector { x: 2, y: 1 },
            }],
        };

        let neighbors = get_neighbors(&path.traveled.iter().last().unwrap(), &input_map);

        assert_eq!(
            neighbors,
            vec![
                &Node {
                    node_type: NodeType::PointOfInterest(0,),
                    position: Vector { x: 1, y: 1 },
                },
                &Node {
                    node_type: NodeType::PointOfInterest(1,),
                    position: Vector { x: 3, y: 1 },
                }
            ]
        );
    }
}
