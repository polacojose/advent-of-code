use crate::{office::is_wall, INITIAL_HASH};
use float_ord::FloatOrd;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::RwLock,
};

#[derive(Debug, Clone)]
pub struct AStarVector {
    pub door_hash: String,
    pub f_score: Option<FloatOrd<f64>>,
    pub x: isize,
    pub y: isize,
}

impl Hash for AStarVector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Eq for AStarVector {}
impl PartialEq for AStarVector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn neighbors<HF>(node: &AStarVector, heuristic_function: &HF) -> Vec<AStarVector>
where
    HF: Fn(&AStarVector) -> f64,
{
    let mut neighbors = Vec::new();

    let open_directions = get_open_directions(node);

    for direction in open_directions {
        match direction {
            Direction::Up => {
                neighbors.push(AStarVector {
                    door_hash: format!("{}U", node.door_hash),
                    f_score: Some(FloatOrd {
                        0: heuristic_function(&AStarVector {
                            door_hash: format!("{}U", node.door_hash),
                            f_score: None,
                            x: node.x,
                            y: node.y - 1,
                        }),
                    }),
                    x: node.x,
                    y: node.y - 1,
                });
            }

            Direction::Down => {
                neighbors.push(AStarVector {
                    door_hash: format!("{}D", node.door_hash),
                    f_score: Some(FloatOrd {
                        0: heuristic_function(&AStarVector {
                            door_hash: format!("{}D", node.door_hash),
                            f_score: None,
                            x: node.x,
                            y: node.y + 1,
                        }),
                    }),
                    x: node.x,
                    y: node.y + 1,
                });
            }
            Direction::Left => {
                neighbors.push(AStarVector {
                    door_hash: format!("{}L", node.door_hash),
                    f_score: Some(FloatOrd {
                        0: heuristic_function(&AStarVector {
                            door_hash: format!("{}L", node.door_hash),
                            f_score: None,
                            x: node.x - 1,
                            y: node.y,
                        }),
                    }),
                    x: node.x - 1,
                    y: node.y,
                });
            }
            Direction::Right => neighbors.push(AStarVector {
                door_hash: format!("{}R", node.door_hash),
                f_score: Some(FloatOrd {
                    0: heuristic_function(&AStarVector {
                        door_hash: format!("{}R", node.door_hash),
                        f_score: None,
                        x: node.x + 1,
                        y: node.y,
                    }),
                }),
                x: node.x + 1,
                y: node.y,
            }),
        }
    }

    neighbors = neighbors
        .into_iter()
        .filter(|node| !is_wall(node.x, node.y))
        .collect();

    neighbors
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_open_hash_char(character: char) -> bool {
    match character {
        'b' | 'c' | 'd' | 'e' | 'f' => true,
        _ => false,
    }
}

fn get_open_directions(node: &AStarVector) -> Vec<Direction> {
    let digest = md5::compute(node.door_hash.to_string());
    let direction_chars = format!("{:?}", digest)[..4]
        .to_string()
        .chars()
        .into_iter()
        .collect::<Vec<_>>();

    if node.door_hash == "ulqzkmivDRURDRUDD" {
        println!("chars {:?}", direction_chars);
    }

    let mut directions = Vec::new();

    if is_open_hash_char(direction_chars[0]) {
        directions.push(Direction::Up);
    }
    if is_open_hash_char(direction_chars[1]) {
        directions.push(Direction::Down);
    }
    if is_open_hash_char(direction_chars[2]) {
        directions.push(Direction::Left);
    }
    if is_open_hash_char(direction_chars[3]) {
        directions.push(Direction::Right);
    }

    directions
}

use lazy_static::lazy_static;

lazy_static! {
    static ref LARGEST_HASH_LEN: RwLock<usize> = Default::default();
    static ref LARGEST_HASH: RwLock<String> = Default::default();
}

pub fn find_path<HF>(start: AStarVector, end: AStarVector, heuristic_function: HF)
where
    HF: Fn(&AStarVector) -> f64,
{
    let mut open_set: Vec<AStarVector> = vec![start.clone()];

    let mut came_from: HashMap<String, String> = Default::default();

    let mut g_score: HashMap<AStarVector, f64> = Default::default();
    g_score.insert(start.clone(), 0.0);

    let mut h_score = HashSet::new();
    h_score.insert(start.door_hash);

    while let Some(current) = open_set.pop() {
        if current == end {
            if current.door_hash.len() > *LARGEST_HASH_LEN.read().unwrap() {
                if *LARGEST_HASH_LEN.read().unwrap() > 0 {
                    *LARGEST_HASH.write().unwrap() = current.door_hash.clone();
                }
                *LARGEST_HASH_LEN.write().unwrap() = current.door_hash.len();
                println!(
                    "Success Hash: {}",
                    current.door_hash.len() - INITIAL_HASH.len()
                );
            }
            continue;
        }

        for neighbor in neighbors(&current, &heuristic_function) {
            let tentative_g_score = g_score.get(&current).unwrap() + distance(&current, &neighbor);
            if !h_score.contains(&neighbor.door_hash) {
                came_from.insert(neighbor.door_hash.clone(), current.door_hash.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                h_score.insert(neighbor.door_hash.clone());
                open_set.push(neighbor);
                open_set.sort_by(|a, b| a.door_hash.len().cmp(&b.door_hash.len()));
            }
        }
    }
}

pub fn distance(start: &AStarVector, end: &AStarVector) -> f64 {
    (((end.x - start.x).pow(2) + (end.y - start.y).pow(2)) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::INITIAL_HASH;

    use super::*;

    #[test]
    fn test_find_path() {
        let start = AStarVector {
            door_hash: INITIAL_HASH.to_string(),
            f_score: Some(FloatOrd { 0: 0.0 }),
            x: 0,
            y: 0,
        };
        let end = AStarVector {
            door_hash: INITIAL_HASH.to_string(),
            f_score: Some(FloatOrd { 0: 0.0 }),
            x: 10,
            y: 10,
        };

        println!(
            "{:?}",
            find_path(start, end.clone(), move |node| {
                if node.x == 4 && node.y == 1 {
                    return f64::INFINITY;
                }
                let s = distance(node, &end);
                s
            })
        )
    }
}
