use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::map::map::{Map, MapNodeKind, SolveCompletion};

pub fn barrier_solve(root_map: &Map) -> Vec<SolveCompletion> {
    let node_count = root_map.width * root_map.height;

    (0..node_count)
        .into_par_iter()
        .filter(|i| {
            if let Some(n) = root_map.nodes.get(*i) {
                match n {
                    crate::map::map::MapNodeKind::Empty => true,
                    _ => false,
                }
            } else {
                false
            }
        })
        .filter_map(|i| {
            let mut m = root_map.clone();

            if let Some(n) = m.nodes.get_mut(i) {
                *n = MapNodeKind::Barrier;
                if let Ok(sc) = m.solve() {
                    Some(sc)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    const TEST_MAP_STR: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    const TEST_MAP_SOLVED_STR: &str = r"....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..
";

    #[test]
    fn test_barrier_solve() {
        let root_map: Map = TEST_MAP_STR.parse().unwrap();
        let count = barrier_solve(&root_map)
            .into_iter()
            .filter(|sc| matches!(sc, SolveCompletion::InALoop))
            .count();
        assert_eq!(count, 6);
    }
}
