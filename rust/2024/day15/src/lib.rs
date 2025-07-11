use std::collections::VecDeque;

use glam::IVec2;
use grid::grid::grid::Grid;

mod parse;

#[derive(Debug, Clone, Copy)]
enum GridNode {
    Robot,
    Box,
    Wall,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RobotMove(IVec2);

#[derive(Debug)]
pub struct RobotMover {
    grid: Grid<GridNode>,
    robot_pos: IVec2,
    movements: VecDeque<RobotMove>,
}

impl RobotMover {
    pub fn gps_sum(&self) -> u64 {
        self.grid
            .get_nodes()
            .iter()
            .enumerate()
            .filter(|(_, n)| matches!(n, GridNode::Box))
            .map(|(i, _)| {
                let p = self.grid.index_to_vector(i).unwrap();
                (p.y * 100 + p.x) as u64
            })
            .sum()
    }

    pub fn solve_movements(&mut self) {
        while let Some(d) = self.movements.pop_front() {
            self.object_move(self.robot_pos, d.0);
        }
    }

    fn object_move(&mut self, pos: IVec2, delta: IVec2) -> bool {
        let object = *self.grid.get_by_vector(&pos).expect("What?");
        match object {
            GridNode::Empty => return true,
            GridNode::Wall => return false,
            _ => (),
        };

        let adjacent_moved = self.object_move(pos + delta, delta);
        if adjacent_moved {
            (*self.grid.get_mut_by_vector(&(pos + delta)).expect("what")) = object;
            (*self.grid.get_mut_by_vector(&(pos)).expect("what")) = GridNode::Empty;

            if matches!(object, GridNode::Robot) {
                self.robot_pos = pos + delta;
            }

            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use grid::grid::grid::Grid;

    const TEST_GRID: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

    const TEST_MULTIPLE_MOVEMENTS_A: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST_MULTIPLE_MOVEMENTS_RESULT_A: &str = r"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
";

    const TEST_MULTIPLE_MOVEMENTS_B: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const TEST_MULTIPLE_MOVEMENTS_RESULT_B: &str = r"##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
";

    #[test]
    fn test_grid_node_move() {
        let grid = TEST_GRID.parse::<Grid<GridNode>>().unwrap();
        let mut rm = RobotMover {
            grid,
            robot_pos: IVec2 { x: 2, y: 2 },
            movements: VecDeque::new(),
        };

        rm.object_move(IVec2 { x: 2, y: 2 }, IVec2 { x: 1, y: 0 });
        rm.object_move(IVec2 { x: 3, y: 2 }, IVec2 { x: 1, y: 0 });
        rm.object_move(IVec2 { x: 4, y: 2 }, IVec2 { x: 1, y: 0 });
        rm.object_move(IVec2 { x: 5, y: 2 }, IVec2 { x: 1, y: 0 });
        rm.object_move(IVec2 { x: 5, y: 2 }, IVec2 { x: -1, y: 0 });

        assert!(matches!(
            rm.grid.get_by_vector(&IVec2 { x: 4, y: 2 }),
            Some(GridNode::Robot)
        ));

        assert!(matches!(
            rm.grid.get_by_vector(&IVec2 { x: 5, y: 2 }),
            Some(GridNode::Empty)
        ));

        assert!(matches!(
            rm.grid.get_by_vector(&IVec2 { x: 6, y: 2 }),
            Some(GridNode::Box)
        ));
    }

    #[test]
    fn test_multiple_movements() {
        let mut rm = TEST_MULTIPLE_MOVEMENTS_A.parse::<RobotMover>().unwrap();
        rm.solve_movements();
        assert_eq!(format!("{}", rm.grid), TEST_MULTIPLE_MOVEMENTS_RESULT_A);

        let mut rm = TEST_MULTIPLE_MOVEMENTS_B.parse::<RobotMover>().unwrap();
        rm.solve_movements();
        assert_eq!(format!("{}", rm.grid), TEST_MULTIPLE_MOVEMENTS_RESULT_B);
    }

    #[test]
    fn test_box_gps() {
        let mut rm = TEST_MULTIPLE_MOVEMENTS_A.parse::<RobotMover>().unwrap();
        rm.solve_movements();
        assert_eq!(rm.gps_sum(), 2028);

        let mut rm = TEST_MULTIPLE_MOVEMENTS_B.parse::<RobotMover>().unwrap();
        rm.solve_movements();
        assert_eq!(rm.gps_sum(), 10092);
    }
}
