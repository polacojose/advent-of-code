use crate::{plot::PlotNode, util::sides_from_angles};
use glam::IVec2;
use grid::grid::map::Grid;
use std::{collections::HashSet, ops::Add};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RegionLayout {
    area: u64,
    parameter: u64,
    angles: u64,
}

impl RegionLayout {
    pub fn new(area: u64, parameter: u64, angles: u64) -> Self {
        Self {
            area,
            parameter,
            angles,
        }
    }
    pub fn area(&self) -> u64 {
        self.area
    }
    pub fn parameter(&self) -> u64 {
        self.parameter
    }
    pub fn angles(&self) -> u64 {
        self.angles
    }
    pub fn sides(&self) -> u64 {
        sides_from_angles(self.angles)
    }
}

impl Add for RegionLayout {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.area + rhs.area,
            self.parameter + rhs.parameter,
            self.angles + rhs.angles,
        )
    }
}

pub fn region_layout(
    p: IVec2,
    grid: &Grid<PlotNode>,
    counted_area: &mut HashSet<IVec2>,
) -> RegionLayout {
    let mut region_layout = r_l(p, grid, counted_area);
    region_layout.area += 1;
    region_layout
}

/// Get the total angles of a partial polygon from its perimeter
/// @param perimeter the perimeter of the partial (Direction, In Region)
fn angles_from_perimeter(perimeter: &[bool]) -> u64 {
    if perimeter.len() != 8 {
        return 0;
    }

    let mut angles = perimeter
        .windows(3)
        .step_by(2)
        .into_iter()
        .map(|vals| {
            let (a, b, c) = (vals[0], vals[1], vals[2]);
            if !a && !c {
                return 90;
            } else if a && !b && c {
                return 90;
            }

            0
        })
        .sum::<u64>();

    angles += {
        let (a, b, c) = (perimeter[6], perimeter[7], perimeter[0]);
        if !a && !c {
            90
        } else if a && !b && c {
            90
        } else {
            0
        }
    };
    angles
}

fn r_l(p: IVec2, grid: &Grid<PlotNode>, counted_area: &mut HashSet<IVec2>) -> RegionLayout {
    counted_area.insert(p);
    let valid_adjs = [[-1, 0], [0, -1], [1, 0], [0, 1]]
        .into_iter()
        .filter_map(|d| {
            let adj_p = p + IVec2::from_array(d);
            if counted_area.get(&adj_p).is_some() {
                None
            } else {
                if let (Some(adj_n), Some(n)) = (grid.get_by_vector(&adj_p), grid.get_by_vector(&p))
                {
                    if adj_n.0 == n.0 {
                        counted_area.insert(adj_p);
                    }
                }
                Some(adj_p)
            }
        })
        .collect::<Vec<_>>();

    let angles = {
        let region_sides = [
            [-1, 0],
            [-1, -1],
            [0, -1],
            [1, -1],
            [1, 0],
            [1, 1],
            [0, 1],
            [-1, 1],
        ]
        .into_iter()
        .map(|d| {
            let d = IVec2::from_array(d);
            let adj_p = p + d;
            if let (Some(adj_n), Some(n)) = (grid.get_by_vector(&adj_p), grid.get_by_vector(&p)) {
                if adj_n.0 == n.0 {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<_>>();
        angles_from_perimeter(&region_sides)
    };

    let mut other_layout_sum = valid_adjs
        .into_iter()
        .map(|adj_p| {
            if let (Some(adj_n), Some(n)) = (grid.get_by_vector(&adj_p), grid.get_by_vector(&p)) {
                if adj_n.0 == n.0 {
                    let mut region_layout = r_l(adj_p, grid, counted_area);
                    region_layout.area += 1;
                    return region_layout;
                }
            }
            RegionLayout::new(0, 1, 0)
        })
        .reduce(|a, b| a + b)
        .unwrap_or(RegionLayout::new(0, 0, 0));

    other_layout_sum.angles += angles;
    other_layout_sum
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PLOTS: &str = r"AAAA
                               BBCD
                               BBCC
                               EEEC";

    #[test]
    fn test_angles_from_perimeter() {
        let angles = angles_from_perimeter(&[]);
        assert_eq!(angles, 0);

        let angles = angles_from_perimeter(&[false]);
        assert_eq!(angles, 0);

        let angles = angles_from_perimeter(&[false, false]);
        assert_eq!(angles, 0);

        let angles =
            angles_from_perimeter(&[false, false, false, false, false, false, false, false]);
        assert_eq!(angles, 360);

        let angles =
            angles_from_perimeter(&[false, false, false, false, false, false, true, false]);
        assert_eq!(angles, 180);

        let angles = angles_from_perimeter(&[true, true, true, true]);
        assert_eq!(angles, 0);
    }
}
