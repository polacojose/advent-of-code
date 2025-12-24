use std::fs;

use grid::{char::TryFromChar, grid::Grid};

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Spool,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    row: usize,
    col: usize,
    n_type: NodeType,
}

impl TryFrom<char> for NodeType {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '.' => Self::Empty,
            '@' => Self::Spool,
            _ => return Err("Invalid node type".to_string()),
        })
    }
}

impl TryFromChar for Node {
    fn from_char(c: char, row: usize, col: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            row,
            col,
            n_type: c.try_into()?,
        })
    }
}

fn main() {
    let mut grid = fs::read_to_string("input")
        .unwrap()
        .trim()
        .parse::<Grid<Node>>()
        .expect("Unable to parse grid.");

    let count = get_free_spool_count(&grid);

    println!("Part 1: {count}");

    let mut total_removed = 0;
    loop {
        let removed = removed_accessible_spools(&mut grid);
        if removed == 0 {
            break;
        }
        total_removed += removed;
    }
    println!("Part 2: {total_removed}");
}

fn free_spool(grid: &Grid<Node>, node: &Node) -> bool {
    if !matches!(node.n_type, NodeType::Spool) {
        return false;
    }

    let mut adj_spools = 0;
    for row in -1..=1 {
        for col in -1..=1 {
            if row == 0 && col == 0 {
                continue;
            }

            if let Some(n) = grid.get(node.row as i32 + row, node.col as i32 + col) {
                if matches!(n.n_type, NodeType::Spool) {
                    adj_spools += 1;
                }
            }
        }
    }

    if adj_spools < 4 { true } else { false }
}

fn get_free_spool_count(grid: &Grid<Node>) -> usize {
    grid.iter().filter(|node| free_spool(&grid, &node)).count()
}

fn removed_accessible_spools(grid: &mut Grid<Node>) -> i32 {
    let mut freed = 0;
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            if !free_spool(&grid, &grid[(i, j)]) {
                continue;
            }

            freed += 1;
            unsafe { grid.get_unchecked_mut(i, j).n_type = NodeType::Empty }
        }
    }
    freed
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "..@@.@@@@.
                            @@@.@.@.@@
                            @@@@@.@.@@
                            @.@@@@..@.
                            @@.@@@@.@@
                            .@@@@@@@.@
                            .@.@.@.@@@
                            @.@@@.@@@@
                            .@@@@@@@@.
                            @.@.@@@.@.";

    #[test]
    fn test_grid_parse() {
        let grid = TEST_DATA
            .parse::<Grid<Node>>()
            .expect("Unable to parse grid.");
        println!("{grid:?}");
        assert!(matches!(grid[(2, 0)].n_type, NodeType::Spool));
    }
    #[test]

    fn test_grid_surround() {
        let grid = TEST_DATA
            .parse::<Grid<Node>>()
            .expect("Unable to parse grid.");

        let count = get_free_spool_count(&grid);

        assert_eq!(count, 13);
    }

    #[test]
    fn test_grid_remove() {
        let mut grid = TEST_DATA
            .parse::<Grid<Node>>()
            .expect("Unable to parse grid.");

        let mut total_removed = 0;
        loop {
            let removed = removed_accessible_spools(&mut grid);
            if removed == 0 {
                break;
            }
            total_removed += removed;
        }

        assert_eq!(total_removed, 43);
    }
}
