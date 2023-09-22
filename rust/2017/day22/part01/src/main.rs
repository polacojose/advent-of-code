use std::{collections::HashMap, fs};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Heading {
    Up = 0,
    Right,
    Down,
    Left,
}

impl From<isize> for Heading {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => panic!(),
        }
    }
}

impl Heading {
    fn rotate(&self, mut rotation: isize) -> Self {
        while rotation < 0 {
            rotation += 4
        }
        let rotation = ((*self as isize) + rotation) % 4;
        rotation.into()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Vector {
    position: (isize, isize),
    heading: Heading,
}

impl Vector {
    fn step(&mut self) {
        match self.heading {
            Heading::Up => self.position.1 -= 1,
            Heading::Right => self.position.0 += 1,
            Heading::Down => self.position.1 += 1,
            Heading::Left => self.position.0 -= 1,
        }
    }
}

struct VirusCarrier {
    infect_count: usize,
    vector: Vector,
}

impl VirusCarrier {
    fn progress(&mut self, node_map: &mut NodeMap) {
        let mut cur_node = node_map.get_node(self.vector.position).clone();
        match cur_node.state {
            NodeState::Infected => {
                self.vector.heading = self.vector.heading.rotate(1);
                cur_node.state = NodeState::Clean;
                node_map.set_node(cur_node);
            }
            NodeState::Clean => {
                self.infect_count += 1;
                self.vector.heading = self.vector.heading.rotate(-1);
                cur_node.state = NodeState::Infected;
                node_map.set_node(cur_node);
            }
        };
        self.vector.step();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NodeState {
    Clean,
    Infected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    position: (isize, isize),
    state: NodeState,
}

#[derive(Debug)]
struct NodeMap {
    min: isize,
    max: isize,
    nodes: HashMap<(isize, isize), Node>,
}

impl NodeMap {
    fn print_nodes(&mut self) {
        for y in self.min..self.max {
            for x in self.min..self.max {
                match self.get_node((x, y)).state {
                    NodeState::Clean => print!("."),
                    NodeState::Infected => print!("#"),
                }
            }
            println!();
        }
    }

    fn get_node(&mut self, pos: (isize, isize)) -> Node {
        if let Some(node) = self.nodes.get(&pos) {
            return node.clone();
        }

        self.min = self.min.min(pos.0);
        self.max = self.max.max(pos.0 + 1);
        self.min = self.min.min(pos.1);
        self.max = self.max.max(pos.1 + 1);
        self.nodes.insert(
            pos,
            Node {
                position: pos,
                state: NodeState::Clean,
            },
        );

        self.nodes.get(&pos).unwrap().clone()
    }

    fn set_node(&mut self, node: Node) {
        self.min = self.min.min(node.position.0);
        self.max = self.max.max(node.position.0 + 1);
        self.min = self.min.min(node.position.1);
        self.max = self.max.max(node.position.1 + 1);
        self.nodes.insert(node.position, node.clone());
    }
}

fn main() {
    let mut node_map = get_node_map();

    let center = { (node_map.max / 2, node_map.max / 2) };

    let mut virus = VirusCarrier {
        infect_count: 0,
        vector: Vector {
            position: center,
            heading: Heading::Up,
        },
    };

    for _ in 0..10000 {
        virus.progress(&mut node_map);
        //node_map.print_nodes();
        //println!();
        //sleep(Duration::from_millis(10))
    }
    println!("Infections: {}", virus.infect_count);
}

fn get_node_map() -> NodeMap {
    let file_string = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    let mut nodes = HashMap::new();
    for y in 0..file_string.len() {
        let line = file_string[y].chars().collect::<Vec<_>>();
        for x in 0..line.len() {
            let state = match line[x] {
                '.' => NodeState::Clean,
                '#' => NodeState::Infected,
                _ => panic!(),
            };
            nodes.insert(
                (x as isize, y as isize),
                Node {
                    position: (x as isize, y as isize),
                    state,
                },
            );
        }
    }

    NodeMap {
        min: 0,
        max: file_string.len() as isize,
        nodes,
    }
}
