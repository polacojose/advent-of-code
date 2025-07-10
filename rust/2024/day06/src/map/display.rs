use std::fmt::Display;

use crate::map::map::{Map, MapNodeKind};

impl Display for MapNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            MapNodeKind::GuardUp => "^",
            MapNodeKind::GuardRight => ">",
            MapNodeKind::GuardLeft => "<",
            MapNodeKind::GuardDown => "v",
            MapNodeKind::Path => "X",
            MapNodeKind::Barrier => "#",
            MapNodeKind::Empty => ".",
        };
        write!(f, "{}", c)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.nodes
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let i = i + 1;
                if i % self.width == 0 {
                    writeln!(f, "{}", n)
                } else {
                    write!(f, "{}", n)
                }
            })
            .collect::<Result<(), _>>()
    }
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

    #[test]
    pub fn test_map_display() {
        let map = TEST_MAP_STR.parse::<Map>().unwrap();

        assert_eq!(format!("{}", map), TEST_MAP_STR);
    }
}
