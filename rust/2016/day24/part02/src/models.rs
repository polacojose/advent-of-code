use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Space,
    Wall,
    PointOfInterest(usize),
}

impl From<char> for NodeType {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Space,
            _ => Self::PointOfInterest(c.to_string().parse().unwrap()),
        }
    }
}

impl Into<char> for NodeType {
    fn into(self) -> char {
        match self {
            NodeType::Space => '.',
            NodeType::Wall => '#',
            NodeType::PointOfInterest(val) => char::from_digit(val as u32, 10).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node {
    pub node_type: NodeType,
    pub position: Vector,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_char: char = self.node_type.into();
        f.write_fmt(format_args!("{} : {}", type_char, self.position))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path<'a> {
    pub crosses_per_node: HashMap<&'a Node, usize>,
    pub points_of_interest: HashSet<&'a Node>,
    pub traveled: Vec<&'a Node>,
}

impl Hash for Path<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.traveled.hash(state);
    }
}

pub fn get_map_string_marked(map: &Vec<Vec<Node>>, path: &Path) -> String {
    let mut map_string = String::new();
    for (y, line) in map.iter().enumerate() {
        for (x, node) in line.iter().enumerate() {
            if path
                .traveled
                .iter()
                .find(|node| node.position.x == x as isize && node.position.y == y as isize)
                .is_some()
            {
                map_string.push('⬤');
            } else {
                map_string.push_str(&match node.node_type {
                    NodeType::Wall => '#'.to_string(),
                    NodeType::Space => '.'.to_string(),
                    NodeType::PointOfInterest(item) => item.to_string(),
                });
            }
        }
        map_string.push('\n');
    }

    map_string
}

pub fn get_map_string(map: &Vec<Vec<Node>>) -> String {
    let mut map_string = String::new();
    for line in map {
        for node in line {
            map_string.push_str(&match node.node_type {
                NodeType::Wall => '#'.to_string(),
                NodeType::Space => '.'.to_string(),
                NodeType::PointOfInterest(item) => item.to_string(),
            });
        }
        map_string.push('\n');
    }

    map_string
}
