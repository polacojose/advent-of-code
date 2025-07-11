use std::{collections::HashSet, fmt::Display};

use glam::IVec2;

#[derive(Debug)]
pub struct GridNode {
    pub height: u32,
    pub reachable_peaks: HashSet<IVec2>,
    pub trails_to_peaks: Option<usize>,
}

impl From<char> for GridNode {
    fn from(c: char) -> Self {
        Self {
            height: c.to_digit(10).unwrap(),
            reachable_peaks: HashSet::new(),
            trails_to_peaks: None,
        }
    }
}

impl Display for GridNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.height)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use grid::grid::map::Grid;

    const TEST_GRID: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_grid_node_parse() {
        let grid = TEST_GRID.parse::<Grid<GridNode>>().unwrap();
        assert_eq!(format!("{grid}").trim(), TEST_GRID);
    }
}
