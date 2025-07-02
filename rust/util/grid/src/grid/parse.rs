use std::{io, str::FromStr};

use crate::grid::map::Grid;

impl<T> FromStr for Grid<T>
where
    T: From<char>,
{
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.trim().lines().count();

        let (width, nodes) =
            s.trim()
                .lines()
                .fold((0, Vec::new()), |(_, mut acc): (usize, Vec<T>), l| {
                    let mut nodes: Vec<T> = l.trim().chars().map(|c| c.into()).collect();
                    let width = nodes.len();
                    acc.append(&mut nodes);
                    (width, acc)
                });

        Ok(Grid {
            width: width as i32,
            height: height as i32,
            nodes,
        })
    }
}

#[cfg(test)]
mod test {

    //use super::*;

    #[test]
    pub fn test_map_node_kind_parse() {
        //assert!(matches!('#'.into(), MapNode::AntiNode));
        //assert!(matches!('A'.into(), MapNode::Antenna('A')));
        //assert!(matches!('.'.into(), MapNode::Empty));
    }
}
