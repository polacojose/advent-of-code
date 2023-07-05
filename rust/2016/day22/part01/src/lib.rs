use std::{fs, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    position: Vector,
    size: usize,
    pub used: usize,
    pub avail: usize,
}

lazy_static! {
    static ref NODE_REGEX: Regex = Regex::new(
        r"x(?P<x>\d+).*?y(?P<y>\d+?).*?(?P<size>\d+)T.*?(?P<used>\d+)T.*?(?P<avail>\d+)T.*?(?P<use>\d+)%",
    )
    .unwrap();
    pub static ref NODES: Vec<Node> = {
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|x| x.parse().unwrap())
            .collect()
    };
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for Node {
    type Err = UnableToParse;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = NODE_REGEX.captures(line) {
            Ok(Node {
                position: Vector {
                    x: captures["x"].parse().unwrap(),
                    y: captures["y"].parse().unwrap(),
                },
                size: captures["size"].parse().unwrap(),
                used: captures["used"].parse().unwrap(),
                avail: captures["avail"].parse().unwrap(),
            })
        } else {
            Err(UnableToParse)
        }
    }
}
