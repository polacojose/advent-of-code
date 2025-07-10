use std::{io, str::FromStr};

use crate::map::map::{Map, MapNodeKind};

impl From<char> for MapNodeKind {
    fn from(c: char) -> Self {
        match c {
            '.' => MapNodeKind::Empty,
            '#' => MapNodeKind::AntiNode,
            other => MapNodeKind::Antenna(other),
        }
    }
}

impl FromStr for Map {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.trim().lines().count();

        let (width, nodes) = s.trim().lines().fold(
            (0, Vec::new()),
            |(_, mut acc): (usize, Vec<MapNodeKind>), l| {
                let mut map_nodes: Vec<MapNodeKind> = l.trim().chars().map(|c| c.into()).collect();
                let width = map_nodes.len();
                acc.append(&mut map_nodes);
                (width, acc)
            },
        );

        Ok(Map {
            width,
            height,
            nodes,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn test_map_node_kind_parse() {
        assert!(matches!('#'.into(), MapNodeKind::AntiNode));
        assert!(matches!('A'.into(), MapNodeKind::Antenna('A')));
        assert!(matches!('.'.into(), MapNodeKind::Empty));
    }
}
