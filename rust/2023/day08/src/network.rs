use std::str::FromStr;

#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
pub struct UnableToParse;

impl FromStr for Dir {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(UnableToParse),
        }
    }
}

#[derive(Debug)]
struct RawNetworkNode {
    name: String,
    left: String,
    right: String,
}

impl FromStr for RawNetworkNode {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.replace("=", "");
        let cleaned = cleaned.replace("(", "");
        let cleaned = cleaned.replace(")", "");
        let cleaned = cleaned.replace(",", "");

        let parts: Vec<&str> = cleaned.split_whitespace().collect();

        Ok(RawNetworkNode {
            name: parts[0].to_owned(),
            left: parts[1].to_owned(),
            right: parts[2].to_owned(),
        })
    }
}

#[derive(Debug)]
struct NetworkNode {
    name: String,
    left: usize,
    right: usize,
}

#[derive(Debug)]
pub struct Network {
    nodes: Vec<NetworkNode>,
}

impl FromStr for Network {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_network_nodes: Vec<RawNetworkNode> = s
            .lines()
            .map(|l| l.parse::<RawNetworkNode>().unwrap())
            .collect();

        let network_nodes = raw_network_nodes
            .iter()
            .map(|rnn| {
                let left_pos = raw_network_nodes
                    .iter()
                    .position(|irnn| irnn.name == rnn.left)
                    .unwrap();
                let right_pos = raw_network_nodes
                    .iter()
                    .position(|irnn| irnn.name == rnn.right)
                    .unwrap();
                NetworkNode {
                    name: rnn.name.clone(),
                    left: left_pos,
                    right: right_pos,
                }
            })
            .collect();

        Ok(Network {
            nodes: network_nodes,
        })
    }
}

impl Network {
    pub fn steps_from_all_starts(&self, instructions: &Vec<Dir>) -> Vec<u64> {
        let start_indexes = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, nn)| nn.name.ends_with("A"))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let steps = start_indexes
            .iter()
            .map(|mut n| {
                let mut num_steps = 0;
                let mut inst_index = 0;
                loop {
                    let inst = &instructions[inst_index];
                    num_steps += 1;

                    //print!("{:?}=>", self.nodes[*s].name);
                    match inst {
                        Dir::Left => n = &self.nodes[*n].left,
                        Dir::Right => n = &self.nodes[*n].right,
                    };
                    // println!("{:?}", self.nodes[*s].name);

                    if self.nodes[*n].name.ends_with("Z") {
                        return num_steps;
                    }

                    inst_index += 1;
                    inst_index = inst_index % instructions.len();
                }
            })
            .collect();

        steps
    }

    pub fn steps_from_single_start(&self, instructions: &Vec<Dir>) -> u32 {
        let start_index = self.nodes.iter().position(|nn| nn.name == "AAA").unwrap();
        let end_index = self.nodes.iter().position(|nn| nn.name == "ZZZ").unwrap();
        let mut current_index = start_index;

        let mut num_steps = 0;
        loop {
            instructions.iter().find(|inst| {
                num_steps += 1;
                match inst {
                    Dir::Left => current_index = self.nodes[current_index].left,
                    Dir::Right => current_index = self.nodes[current_index].right,
                };
                current_index == end_index
            });
            if current_index == end_index {
                return num_steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use lcmx::lcmx;

    use super::*;

    #[test]
    fn can_parse_network_node_raw() {
        let node_str = r#"AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

        let raw_network_nodes: Vec<RawNetworkNode> = node_str
            .lines()
            .map(|l| l.parse::<RawNetworkNode>().unwrap())
            .collect();

        assert_eq!(raw_network_nodes[0].left, "BBB");
        assert_eq!(raw_network_nodes[0].right, "CCC");

        assert_eq!(raw_network_nodes[1].left, "DDD");
        assert_eq!(raw_network_nodes[1].right, "EEE");

        assert_eq!(raw_network_nodes[2].left, "ZZZ");
        assert_eq!(raw_network_nodes[2].right, "GGG");

        assert_eq!(raw_network_nodes[3].left, "DDD");
        assert_eq!(raw_network_nodes[3].right, "DDD");

        assert_eq!(raw_network_nodes[4].left, "EEE");
        assert_eq!(raw_network_nodes[4].right, "EEE");

        assert_eq!(raw_network_nodes[5].left, "GGG");
        assert_eq!(raw_network_nodes[5].right, "GGG");

        assert_eq!(raw_network_nodes[6].left, "ZZZ");
        assert_eq!(raw_network_nodes[6].right, "ZZZ");
    }

    #[test]
    fn can_parse_network_node() {
        let node_str = r#"AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

        let network = node_str.parse::<Network>().unwrap();

        assert_eq!(network.nodes[0].left, 1);
        assert_eq!(network.nodes[0].right, 2);

        assert_eq!(network.nodes[1].left, 3);
        assert_eq!(network.nodes[1].right, 4);

        assert_eq!(network.nodes[2].left, 6);
        assert_eq!(network.nodes[2].right, 5);

        assert_eq!(network.nodes[3].left, 3);
        assert_eq!(network.nodes[3].right, 3);

        assert_eq!(network.nodes[4].left, 4);
        assert_eq!(network.nodes[4].right, 4);

        assert_eq!(network.nodes[5].left, 5);
        assert_eq!(network.nodes[5].right, 5);
    }

    #[test]
    fn can_find_steps() {
        let node_str = r#"AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

        let network = node_str.parse::<Network>().unwrap();

        assert_eq!(
            network.steps_from_single_start(
                &"RL"
                    .chars()
                    .map(|c| c.to_string().parse::<Dir>().unwrap())
                    .collect()
            ),
            2
        );

        let node_str = r#"AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

        let network = node_str.parse::<Network>().unwrap();

        assert_eq!(
            network.steps_from_single_start(
                &"LLR"
                    .chars()
                    .map(|c| c.to_string().parse::<Dir>().unwrap())
                    .collect()
            ),
            6
        );
    }

    #[test]
    fn can_find_steps_from_all_starts() {
        let node_str = r#"11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let network = node_str.parse::<Network>().unwrap();

        let result = network.steps_from_all_starts(
            &"LR"
                .chars()
                .map(|c| c.to_string().parse::<Dir>().unwrap())
                .collect(),
        );

        assert_eq!(lcmx(&result).unwrap(), 6)
    }
}
