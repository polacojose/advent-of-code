use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs, mem,
};

use futures::future::BoxFuture;
use grid::{char::TryFromChar, grid::Grid};
use tokio::join;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum NodeType {
    Start,
    Empty,
    Splitter,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeType::Start => "S",
                NodeType::Empty => ".",
                NodeType::Splitter => "^",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    row: usize,
    col: usize,
    r_type: NodeType,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r_type)
    }
}

impl TryFromChar for Node {
    fn from_char(c: char, row: usize, col: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            row,
            col,
            r_type: match c {
                'S' => NodeType::Start,
                '.' => NodeType::Empty,
                '^' => NodeType::Splitter,
                c => panic!("Unable to parse Node: {:?}", c),
            },
        })
    }
}

#[tokio::main]
async fn main() {
    let grid = fs::read_to_string("input")
        .unwrap()
        .trim()
        .parse::<Grid<Node>>()
        .unwrap();

    let count = count_splits(&grid);
    println!("Part 1: {count}");

    let count = count_paths(&grid);
    println!("Part 2: {count}");
}

fn count_splits(grid: &Grid<Node>) -> usize {
    if let Some(start) = grid.iter().find(|n| matches!(n.r_type, NodeType::Start)) {
        return count_splits_recur(grid, start, &mut HashSet::default()).len();
    }
    0
}

fn count_splits_recur(
    grid: &Grid<Node>,
    beam_node: &Node,
    visited_nodes: &mut HashSet<Node>,
) -> HashSet<Node> {
    visited_nodes.insert(*beam_node);
    let mut hit_splitter_nodes = HashSet::default();
    if let Some(down_node) = grid.get(beam_node.row + 1, beam_node.col) {
        if visited_nodes.contains(down_node) {
            return hit_splitter_nodes;
        }
        match down_node.r_type {
            NodeType::Empty => {
                count_splits_recur(grid, down_node, visited_nodes)
                    .drain()
                    .for_each(|n| {
                        hit_splitter_nodes.insert(n);
                    });
            }
            NodeType::Splitter => {
                hit_splitter_nodes.insert(*down_node);
                visited_nodes.insert(*down_node);
                if down_node.col > 0
                    && let Some(l) = grid.get(down_node.row, down_node.col - 1)
                {
                    count_splits_recur(grid, l, visited_nodes)
                        .drain()
                        .for_each(|n| {
                            hit_splitter_nodes.insert(n);
                        });
                }

                if let Some(r) = grid.get(down_node.row, down_node.col + 1) {
                    count_splits_recur(grid, r, visited_nodes)
                        .drain()
                        .for_each(|n| {
                            hit_splitter_nodes.insert(n);
                        });
                }
            }
            _ => {}
        }
    }
    hit_splitter_nodes
}

fn count_paths(grid: &Grid<Node>) -> usize {
    if let Some(start) = grid.iter().find(|n| matches!(n.r_type, NodeType::Start)) {
        return count_paths_recur(grid, start, &mut HashMap::default()) + 1;
    }
    0
}

fn count_paths_recur(
    grid: &Grid<Node>,
    beam_node: &Node,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(m) = memo.get(&(beam_node.row, beam_node.col)) {
        return *m;
    }

    let mut row = beam_node.row + 1;
    let mut paths = 0;
    loop {
        if let Some(down_node) = grid.get(row, beam_node.col) {
            match down_node.r_type {
                NodeType::Empty => {
                    row += 1;
                }
                NodeType::Splitter => {
                    let mut split_paths = 0;
                    if down_node.col > 0
                        && let Some(l) = grid.get(down_node.row, down_node.col - 1)
                    {
                        split_paths += 1;
                        paths += count_paths_recur(grid, l, memo);
                    }

                    if let Some(r) = grid.get(down_node.row, down_node.col + 1) {
                        split_paths += 1;
                        paths += count_paths_recur(grid, r, memo);
                    }

                    paths += split_paths - 1;
                    break;
                }
                _ => break,
            }
        } else {
            break;
        }
    }

    memo.insert((beam_node.row, beam_node.col), paths);

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_SHORT_A: &str = r#".S.
...
.^.
...
^.^
...
.^."#;

    const TEST_DATA: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
    #[test]

    fn test_spread_short() {
        let grid = TEST_DATA_SHORT_A.trim().parse::<Grid<Node>>().unwrap();
        println!("{grid}");

        let count = count_splits(&grid);

        assert_eq!(count, 4);
    }

    #[test]
    fn test_spread() {
        let grid = TEST_DATA.trim().parse::<Grid<Node>>().unwrap();
        println!("{grid}");

        let count = count_splits(&grid);

        assert_eq!(count, 21);
    }

    #[tokio::test]
    async fn test_paths() {
        let grid = TEST_DATA_SHORT_A.trim().parse::<Grid<Node>>().unwrap();
        println!("{grid}");

        let count = count_paths(&grid);

        assert_eq!(count, 4);

        let grid = TEST_DATA.trim().parse::<Grid<Node>>().unwrap();
        println!("{grid}");

        let count = count_paths(&grid);

        assert_eq!(count, 40);
    }
}
