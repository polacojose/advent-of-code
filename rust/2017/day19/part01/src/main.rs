use std::{fs, thread::sleep, time::Duration};

#[derive(Debug, Clone, Copy)]
enum NodeType {
    Vert,
    Horz,
    Cross,
    Indicator { char: char },
}

#[derive(Debug)]
struct Node {
    vector: (usize, usize),
    nodetype: NodeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct TravelVector {
    pos: (usize, usize),
    dir: Dir,
}

impl Default for TravelVector {
    fn default() -> Self {
        Self {
            pos: (0, 0),
            dir: Dir::Down,
        }
    }
}

fn parse_nodes(file_string: impl AsRef<str>) -> Vec<Node> {
    let lines = file_string.as_ref().lines().collect::<Vec<_>>();

    let mut nodes = Vec::new();
    for y in 0..lines.len() {
        let chars = lines[y].chars().collect::<Vec<_>>();
        for x in 0..chars.len() {
            match chars[x] {
                '-' => nodes.push(Node {
                    vector: (x, y),
                    nodetype: NodeType::Horz,
                }),
                '|' => nodes.push(Node {
                    vector: (x, y),
                    nodetype: NodeType::Vert,
                }),
                '+' => nodes.push(Node {
                    vector: (x, y),
                    nodetype: NodeType::Cross,
                }),
                ' ' => {}
                c => nodes.push(Node {
                    vector: (x, y),
                    nodetype: NodeType::Indicator { char: c },
                }),
            }
        }
    }

    nodes
}

fn dir_to_delta(dir: Dir) -> (isize, isize) {
    match dir {
        Dir::Up => (0, -1),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
    }
}

fn get_node(pos: (usize, usize), map: &Vec<Node>) -> Option<&Node> {
    //println!("{:?}", pos);
    map.iter().find(|x| x.vector == pos)
}

fn get_next_node(travel_vector: &TravelVector, map: &Vec<Node>) -> Option<(Dir, (isize, isize))> {
    let next_node = |dirs: Vec<Dir>| -> Option<(Dir, (isize, isize))> {
        let next_node = dirs
            .into_iter()
            .map(|d| (d, dir_to_delta(d)))
            .map(|delta| {
                (
                    delta.0,
                    (
                        travel_vector.pos.0 as isize + delta.1 .0,
                        travel_vector.pos.1 as isize + delta.1 .1,
                    ),
                )
            })
            .filter(|(_, pos)| if pos.0 < 0 || pos.1 < 1 { false } else { true })
            .filter(|(_, pos)| get_node((pos.0 as usize, pos.1 as usize), map).is_some())
            .last();
        next_node
    };

    let d = next_node(vec![travel_vector.dir]);
    if d.is_some() {
        return d;
    }

    let mut change_dirs = Vec::new();
    match travel_vector.dir {
        Dir::Up | Dir::Down => {
            change_dirs.push(Dir::Left);
            change_dirs.push(Dir::Right);
        }
        Dir::Left | Dir::Right => {
            change_dirs.push(Dir::Up);
            change_dirs.push(Dir::Down);
        }
    }

    next_node(change_dirs)
}

fn main() {
    let nodes = parse_nodes(fs::read_to_string("input.txt").unwrap());
    let mut travel_vector = TravelVector {
        pos: nodes[0].vector,
        dir: Dir::Down,
    };

    let mut steps = 1;
    while let Some(delta) = get_next_node(&travel_vector, &nodes) {
        steps += 1;
        travel_vector.dir = delta.0;
        travel_vector.pos = (delta.1 .0 as usize, delta.1 .1 as usize);
        if let Some(node) = get_node(travel_vector.pos, &nodes) {
            match node.nodetype {
                NodeType::Indicator { char } => {
                    print!("{char}");
                }
                _ => {}
            }
        }
    }
    println!();
    println!("Steps: {steps}");
}
