use std::{io, str::FromStr};

use crate::{
    map::map::{Map, MapNodeKind},
    vector::IVec2,
};

impl From<char> for MapNodeKind {
    fn from(c: char) -> Self {
        match c {
            '^' => MapNodeKind::GuardUp,
            '>' => MapNodeKind::GuardRight,
            '<' => MapNodeKind::GuardLeft,
            'v' => MapNodeKind::GuardDown,
            'X' => MapNodeKind::Path,
            '#' => MapNodeKind::Barrier,
            _ => MapNodeKind::Empty,
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

        let move_vector = nodes
            .iter()
            .find_map(|g| match g {
                MapNodeKind::GuardUp => Some(IVec2 { x: 0, y: -1 }),
                MapNodeKind::GuardDown => Some(IVec2 { x: 0, y: 1 }),
                MapNodeKind::GuardLeft => Some(IVec2 { x: -1, y: 0 }),
                MapNodeKind::GuardRight => Some(IVec2 { x: 1, y: 0 }),
                _ => None,
            })
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "Guard Node Missing",
            ))?;

        Ok(Map {
            width,
            height,
            nodes,
            guard_move_vector: move_vector,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn test_map_node_kind_parse() {
        assert!(matches!('^'.into(), MapNodeKind::GuardUp));
        assert!(matches!('>'.into(), MapNodeKind::GuardRight));
        assert!(matches!('<'.into(), MapNodeKind::GuardLeft));
        assert!(matches!('v'.into(), MapNodeKind::GuardDown));
        assert!(matches!('#'.into(), MapNodeKind::Barrier));
        assert!(matches!('.'.into(), MapNodeKind::Empty));
    }
}
