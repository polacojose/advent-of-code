use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
enum LocState {
    COORD(usize),
    Neutral,
}

#[derive(Debug, PartialEq, Eq)]
struct Location {
    pos: (usize, usize),
    state: LocState,
    distances: Vec<(usize, usize)>,
}

#[derive(Debug)]
pub struct Grid {
    origin: (usize, usize),
    width: usize,
    height: usize,
    locations: Vec<Location>,
}

impl From<Vec<(usize, usize)>> for Grid {
    fn from(coordinates: Vec<(usize, usize)>) -> Self {
        let ul = coordinates
            .iter()
            .cloned()
            .reduce(|(aa, bb), (a, b)| (a.min(aa), b.min(bb)))
            .unwrap();

        let lr = coordinates
            .iter()
            .cloned()
            .reduce(|(aa, bb), (a, b)| (a.max(aa), b.max(bb)))
            .unwrap();

        let origin = ul;
        let width = lr.0 - ul.0;
        let height = lr.1 - ul.1;

        let locations = Grid::calculate_locations(origin, height, width, coordinates);

        Grid {
            origin,
            width,
            height,
            locations,
        }
    }
}

impl Grid {
    fn calculate_locations(
        origin: (usize, usize),
        height: usize,
        width: usize,
        coordinates: Vec<(usize, usize)>,
    ) -> Vec<Location> {
        let mut locations = Vec::new();
        for y in origin.1..=origin.1 + height {
            for x in origin.0..=origin.0 + width {
                let c = (x, y);

                let distances_from_poi: Vec<(usize, usize)> = coordinates
                    .iter()
                    .enumerate()
                    .map(|(i, x)| (i, distance(&c, x)))
                    .collect();
                let min = distances_from_poi.iter().min_by_key(|x| x.1).unwrap().1;
                let min_distances_from_poi = distances_from_poi
                    .iter()
                    .filter(|x| x.1 == min)
                    .collect::<Vec<_>>();

                let state = if min_distances_from_poi.len() > 1 {
                    LocState::Neutral
                } else {
                    LocState::COORD(min_distances_from_poi.first().unwrap().0)
                };

                locations.push(Location {
                    pos: c,
                    state,
                    distances: distances_from_poi,
                });
            }
        }
        locations
    }

    fn finite_poi(&self) -> Vec<usize> {
        let poi = self
            .locations
            .iter()
            .filter_map(|x| match x.state {
                LocState::COORD(a) => Some(a),
                _ => None,
            })
            .collect::<HashSet<_>>();
        let infinite_poi = self
            .locations
            .iter()
            .filter_map(|x| match x.state {
                LocState::COORD(a) => {
                    if x.pos.0 == self.origin.0
                        || x.pos.1 == self.origin.1
                        || x.pos.0 == self.origin.0 + self.width
                        || x.pos.1 == self.origin.1 + self.height
                    {
                        Some(a)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<HashSet<_>>();
        return (&poi - &infinite_poi).into_iter().collect();
    }

    pub fn largest_area(&self) -> Option<usize> {
        Some(
            self.finite_poi()
                .into_iter()
                .map(|i| {
                    let mut count = 0;
                    for p in &self.locations {
                        match p.state {
                            LocState::COORD(j) => {
                                if i == j {
                                    count += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    (i, count)
                })
                .max_by_key(|x| x.1)?
                .1,
        )
    }

    pub fn central_location_area(&self, target_size: usize) -> usize {
        self.locations
            .iter()
            .filter(|l| l.distances.iter().map(|d| d.1).sum::<usize>() < target_size)
            .count()
    }
}

fn distance(from: &(usize, usize), to: &(usize, usize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

/*
* A Coordinate has infinite area if it has nodes along the edges (defined by the furthest coordinates
* beyond one of it's sides
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn demo_grid() -> Grid {
        let coordinates = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
        coordinates.into()
    }

    #[test]
    fn can_determine_grid() {
        let grid = demo_grid();
        assert_eq!(grid.origin, (1, 1));
        assert_eq!(grid.width, 7);
        assert_eq!(grid.height, 8);
        assert!(grid.locations.len() == 72);
    }

    #[test]
    fn locations_can_determine_state() {
        let grid = demo_grid();
        for l in grid.locations {
            if l.pos == (1, 4) {
                assert_eq!(l.state, LocState::Neutral);
            } else if l.pos == (2, 4) {
                assert_eq!(l.state, LocState::COORD(3));
            } else if l.pos == (1, 5) {
                assert_eq!(l.state, LocState::COORD(1));
            }
        }
    }

    #[test]
    fn can_identify_finite() {
        let grid = demo_grid();
        for x in [3, 4] {
            assert!(grid.finite_poi().contains(&x));
        }
    }

    #[test]
    fn can_get_largest_area() {
        let grid = demo_grid();
        assert_eq!(grid.largest_area(), Some(17));
    }

    #[test]
    fn can_get_central_location_area() {
        let grid = demo_grid();
        assert_eq!(grid.central_location_area(32), 16);
    }
}
