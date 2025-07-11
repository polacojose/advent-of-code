use crate::vector::IVec2;

#[derive(Clone)]
pub(super) enum MapNodeKind {
    Empty,
    AntiNode,
    Antenna(char),
}

#[derive(Clone)]
pub struct Map {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) nodes: Vec<MapNodeKind>,
}

impl Map {
    pub fn num_antinodes(&self, linear: bool) -> usize {
        let antinode_map = Map::construct_antinode_map(&self, linear);
        antinode_map
            .nodes
            .iter()
            .filter(|n| matches!(n, MapNodeKind::AntiNode))
            .count()
    }

    fn construct_antinode_map(antenna_map: &Map, linear: bool) -> Map {
        let mut antinode_map = antenna_map.clone();

        let antenna_nodes: Vec<(usize, char)> = antinode_map
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| match n {
                MapNodeKind::Antenna(x) => Some((i, *x)),
                _ => None,
            })
            .collect();

        let antenna_nodes_len = antenna_nodes.len();

        for a in 0..antenna_nodes_len {
            let a_vec = antinode_map.index_to_vector(antenna_nodes[a].0).unwrap();
            for b in (a + 1)..antenna_nodes_len {
                if antenna_nodes[a].1 != antenna_nodes[b].1 {
                    continue;
                }

                let b_vec = antinode_map.index_to_vector(antenna_nodes[b].0).unwrap();

                let (mut left_vec, mut right_vec) = if a_vec.x < b_vec.x {
                    (a_vec, b_vec)
                } else {
                    (b_vec, a_vec)
                };

                let diff_vec = left_vec - right_vec;

                if linear {
                    while let Some(left_idx) = antinode_map.vector_to_index(left_vec - diff_vec) {
                        antinode_map.nodes[left_idx] = MapNodeKind::AntiNode;
                        left_vec = left_vec - diff_vec;
                    }
                    while let Some(right_idx) = antinode_map.vector_to_index(right_vec + diff_vec) {
                        antinode_map.nodes[right_idx] = MapNodeKind::AntiNode;
                        right_vec = right_vec + diff_vec;
                    }
                } else {
                    if let Some(left_idx) = antinode_map.vector_to_index(left_vec + diff_vec) {
                        antinode_map.nodes[left_idx] = MapNodeKind::AntiNode;
                    }
                    if let Some(right_idx) = antinode_map.vector_to_index(right_vec - diff_vec) {
                        antinode_map.nodes[right_idx] = MapNodeKind::AntiNode;
                    }
                }
            }
        }

        antinode_map
    }
}

#[derive(Debug)]
pub struct SolveError(&'static str);
impl Map {
    fn index_to_vector(&self, idx: usize) -> Option<IVec2> {
        if idx >= self.nodes.len() {
            return None;
        }

        let x = (idx % self.width) as isize;
        let y = (idx / self.width) as isize;

        Some(IVec2 { x, y })
    }

    fn vector_to_index(&self, p: IVec2) -> Option<usize> {
        let ux = p.x as usize;
        let uy = p.y as usize;

        if p.x < 0 || ux >= self.width || p.y < 0 || uy >= self.height {
            return None;
        }
        Some((uy * self.width + ux) as usize)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const ANTENNA_MAP_1: &str = r"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";

    const ANTINODE_MAP_1: &str = r"..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........
";

    const ANTENNA_MAP_2: &str = r"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........
";

    const ANTINODE_MAP_2: &str = r"..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........
";

    const ANTENNA_MAP_3: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_construct_antinode_map() {
        let antenna_map = ANTENNA_MAP_1.parse::<Map>().unwrap();
        let antinode_map = Map::construct_antinode_map(&antenna_map, false);

        println!("{antenna_map}");
        println!("{antinode_map}");

        assert_eq!(format!("{}", antinode_map), ANTINODE_MAP_1);

        let antenna_map = ANTENNA_MAP_2.parse::<Map>().unwrap();
        let antinode_map = Map::construct_antinode_map(&antenna_map, false);

        println!("{antenna_map}");
        println!("{antinode_map}");

        assert_eq!(format!("{}", antinode_map), ANTINODE_MAP_2);

        let antenna_map = ANTENNA_MAP_3.parse::<Map>().unwrap();
        let antinode_map = Map::construct_antinode_map(&antenna_map, false);

        let anti_node_count = antinode_map
            .nodes
            .iter()
            .filter(|n| matches!(n, MapNodeKind::AntiNode))
            .count();

        assert_eq!(anti_node_count, 14);
    }

    #[test]
    fn test_construct_antinode_map_linear() {
        let antenna_map = ANTENNA_MAP_3.parse::<Map>().unwrap();
        let antinode_map = Map::construct_antinode_map(&antenna_map, true);

        let anti_node_count = antinode_map
            .nodes
            .iter()
            .filter(|n| matches!(n, MapNodeKind::AntiNode))
            .count();

        println!("{antinode_map}");

        assert_eq!(anti_node_count, 34);
    }
}
