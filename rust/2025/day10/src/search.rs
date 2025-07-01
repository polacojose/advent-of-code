use std::collections::HashSet;

use glam::IVec2;
use grid::grid::map::Grid;

use crate::gridnode::GridNode;

pub fn get_node_peaks(grid: &mut Grid<GridNode>, node_pos: &IVec2) -> HashSet<IVec2> {
    let peaks: HashSet<IVec2> = [[-1, 0], [0, -1], [1, 0], [0, 1]]
        .into_iter()
        .flat_map(|[x, y]| {
            let adj_pos = node_pos + IVec2 { x, y };
            if let (Some(n), Some(adj_n)) =
                (grid.get_by_vector(node_pos), grid.get_by_vector(&adj_pos))
            {
                if n.height + 1 == adj_n.height {
                    return if adj_n.height == 9 {
                        HashSet::from([adj_pos])
                    } else {
                        if !adj_n.reachable_peaks.is_empty() {
                            return adj_n.reachable_peaks.clone();
                        }
                        get_node_peaks(grid, &adj_pos)
                    };
                }
            }
            HashSet::new()
        })
        .collect();

    grid.get_mut_by_vector(node_pos).unwrap().reachable_peaks = peaks.clone();

    peaks
}

pub fn get_trail_counts(grid: &mut Grid<GridNode>, node_pos: &IVec2) -> usize {
    let trails_to_peaks: usize = [[-1, 0], [0, -1], [1, 0], [0, 1]]
        .into_iter()
        .map(|[x, y]| {
            let adj_pos = node_pos + IVec2 { x, y };
            if let (Some(n), Some(adj_n)) =
                (grid.get_by_vector(node_pos), grid.get_by_vector(&adj_pos))
            {
                //Ignore lessthan or equal heights
                if n.height + 1 != adj_n.height {
                    return 0;
                }

                //Return 1 on trail end
                if adj_n.height == 9 {
                    return 1;
                }

                //Return a trail score for known (previously traveled) branches
                if let Some(trails_to_peaks) = adj_n.trails_to_peaks {
                    return trails_to_peaks;
                }

                //Calculate and return the result for unknown branches
                return get_trail_counts(grid, &adj_pos);
            }

            0
        })
        .sum();

    grid.get_mut_by_vector(node_pos).unwrap().trails_to_peaks = Some(trails_to_peaks.clone());

    trails_to_peaks
}

#[cfg(test)]
mod test {
    use super::*;
    use grid::grid::map::Grid;

    const TEST_GRID: &str = r"0123
1234
8765
9876";

    const TEST_GRID_2: &str = r"89010123
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

    #[test]
    fn test_get_node_peaks() {
        let mut grid = TEST_GRID_2.parse::<Grid<GridNode>>().unwrap();
        let trail_heads = grid
            .get_nodes()
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                if n.height == 0 {
                    Some(grid.index_to_vector(i)?)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let peak_count = trail_heads
            .iter()
            .map(|v| get_node_peaks(&mut grid, v).len())
            .sum::<usize>();
        assert_eq!(peak_count, 36);
    }

    #[test]
    fn test_get_trail_counts() {
        let mut grid = TEST_GRID_2.parse::<Grid<GridNode>>().unwrap();
        let trail_heads = grid
            .get_nodes()
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                if n.height == 0 {
                    Some(grid.index_to_vector(i)?)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let peak_count = trail_heads
            .iter()
            .map(|v| get_trail_counts(&mut grid, v))
            .sum::<usize>();
        assert_eq!(peak_count, 81);
    }
}
