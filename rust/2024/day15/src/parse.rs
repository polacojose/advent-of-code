use std::{error::Error, fmt::Display, str::FromStr};

use glam::IVec2;
use grid::grid::grid::Grid;

use crate::{GridNode, RobotMove, RobotMover};

impl From<char> for GridNode {
    fn from(c: char) -> Self {
        match c {
            '@' => GridNode::Robot,
            'O' => GridNode::Box,
            '#' => GridNode::Wall,
            '.' => GridNode::Empty,
            _ => panic!("Invalid GridNode char"),
        }
    }
}

impl Display for GridNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            GridNode::Robot => '@',
            GridNode::Box => 'O',
            GridNode::Wall => '#',
            GridNode::Empty => '.',
        };
        write!(f, "{}", c)
    }
}

impl From<char> for RobotMove {
    fn from(c: char) -> Self {
        let delta = match c {
            '^' => [0, -1],
            '>' => [1, 0],
            '<' => [-1, 0],
            'v' => [0, 1],
            _ => panic!("Invalid GridNode char"),
        };
        RobotMove(IVec2::from_array(delta))
    }
}

impl Display for RobotMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            IVec2 { x: 0, y: -1 } => '^',
            IVec2 { x: 1, y: 0 } => '>',
            IVec2 { x: -1, y: 0 } => '<',
            IVec2 { x: 0, y: 1 } => 'v',
            _ => panic!("Invalid RobotMove"),
        };
        write!(f, "{}", c)
    }
}

impl FromStr for RobotMover {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (grid_str, movements_str) = s.trim().split_once("\n\n").unwrap();

        let grid = grid_str.parse::<Grid<GridNode>>().unwrap();
        let robot_pos = grid
            .get_nodes()
            .iter()
            .enumerate()
            .find_map(|(i, n)| {
                if matches!(n, GridNode::Robot) {
                    Some(grid.index_to_vector(i)?)
                } else {
                    None
                }
            })
            .ok_or("Robot not found.")?;

        let movements = movements_str
            .trim()
            .lines()
            .flat_map(|l| {
                l.trim()
                    .chars()
                    .map(|c| c.into())
                    .collect::<Vec<RobotMove>>()
            })
            .collect();

        Ok(Self {
            grid,
            robot_pos,
            movements,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_GRID: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

    const TEST_DELTAS: &str = r"^^><v";

    #[test]
    fn test_grid_node_parse() {
        let moves = TEST_DELTAS
            .trim()
            .chars()
            .map(|c| c.into())
            .collect::<Vec<RobotMove>>();
        assert_eq!(
            moves,
            [
                RobotMove(IVec2 { x: 0, y: -1 }),
                RobotMove(IVec2 { x: 0, y: -1 }),
                RobotMove(IVec2 { x: 1, y: 0 }),
                RobotMove(IVec2 { x: -1, y: 0 }),
                RobotMove(IVec2 { x: 0, y: 1 })
            ]
        );
    }
}
