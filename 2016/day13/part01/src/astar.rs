use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    thread::sleep,
    time::Duration,
};

use float_ord::FloatOrd;

use crate::office::is_wall;

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct AStarVector {
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

fn reconstruct_path(
    came_from: HashMap<AStarVector, AStarVector>,
    mut current: AStarVector,
) -> Vec<AStarVector> {
    let mut total_path = vec![current.clone()];

    while let Some(previous) = came_from.get(&current) {
        total_path.insert(0, previous.clone());
        current = previous.clone();
    }

    total_path
}

fn neighbors<HF>(node: &AStarVector, heuristic_function: &HF) -> Vec<AStarVector>
where
    HF: Fn(&AStarVector) -> f64,
{
    let mut neighbors = vec![
        AStarVector {
            f_score: Some(FloatOrd {
                0: heuristic_function(&AStarVector {
                    f_score: None,
                    x: node.x + 1,
                    y: node.y,
                }),
            }),
            x: node.x + 1,
            y: node.y,
        },
        AStarVector {
            f_score: Some(FloatOrd {
                0: heuristic_function(&AStarVector {
                    f_score: None,
                    x: node.x,
                    y: node.y + 1,
                }),
            }),
            x: node.x,
            y: node.y + 1,
        },
    ];

    if node.x > 0 {
        neighbors.push(AStarVector {
            f_score: Some(FloatOrd {
                0: heuristic_function(&AStarVector {
                    f_score: None,
                    x: node.x - 1,
                    y: node.y,
                }),
            }),
            x: node.x - 1,
            y: node.y,
        });
    }
    if node.y > 0 {
        neighbors.push(AStarVector {
            f_score: Some(FloatOrd {
                0: heuristic_function(&AStarVector {
                    f_score: None,
                    x: node.x,
                    y: node.y - 1,
                }),
            }),
            x: node.x,
            y: node.y - 1,
        });
    }

    neighbors
}

#[derive(Debug)]
pub struct UnableToFindPath;

pub fn find_path<HF>(
    start: AStarVector,
    end: AStarVector,
    heuristic_function: HF,
) -> Result<Vec<AStarVector>, UnableToFindPath>
where
    HF: Fn(&AStarVector) -> f64,
{
    let mut open_set: BTreeSet<AStarVector> = BTreeSet::from_iter(vec![start.clone()]);

    let mut came_from: HashMap<AStarVector, AStarVector> = Default::default();

    let mut g_score: HashMap<AStarVector, f64> = Default::default();
    g_score.insert(start.clone(), 0.0);

    let mut f_score: HashMap<AStarVector, f64> = Default::default();
    let heuristic = heuristic_function(&start);
    f_score.insert(start, heuristic);

    while let Some(current) = open_set.pop_first() {
        if current == end {
            return Ok(reconstruct_path(came_from, current));
        }

        for neighbor in neighbors(&current, &heuristic_function) {
            let tentative_g_score = g_score.get(&current).unwrap() + distance(&current, &neighbor);
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);

                let heuristic = heuristic_function(&neighbor);
                f_score.insert(neighbor.clone(), tentative_g_score + heuristic);

                open_set.insert(neighbor);
            }
        }
    }

    Err(UnableToFindPath)
}

pub fn distance(start: &AStarVector, end: &AStarVector) -> f64 {
    (((end.x - start.x).pow(2) + (end.y - start.y).pow(2)) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path() {
        let start = AStarVector {
            f_score: Some(FloatOrd { 0: 0.0 }),
            x: 0,
            y: 0,
        };
        let end = AStarVector {
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
