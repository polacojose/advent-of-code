use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub id: usize,
    pub position: Vector,
    pub size: usize,
    pub used: usize,
    pub goal_data: bool,
}

pub struct InvalidTransfer(&'static str);
impl Node {
    pub fn avail(&self) -> usize {
        self.size - self.used
    }

    pub fn transfer_to(&mut self, other: &mut Self) -> Result<(), InvalidTransfer> {
        if other.avail() < self.used {
            return Err(InvalidTransfer("Insufficient Space Available"));
        }

        if other.goal_data {
            return Err(InvalidTransfer("Overriding Goal Data!"));
        }

        other.used += self.used;
        other.goal_data = self.goal_data;

        self.used = 0;
        self.goal_data = false;
        Ok(())
    }
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for Node {
    type Err = UnableToParse;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let node_regex: Regex = Regex::new(
            r"x(?P<x>\d+).*?y(?P<y>\d+?).*?(?P<size>\d+)T.*?(?P<used>\d+)T.*?(?P<avail>\d+)T.*?(?P<use>\d+)%",
        ).map_err(|_| UnableToParse)?;

        if let Some(captures) = node_regex.captures(line) {
            let size = captures["size"].parse().unwrap();
            let used = captures["used"].parse().unwrap();

            Ok(Node {
                id: 0,
                position: Vector {
                    x: captures["x"].parse().unwrap(),
                    y: captures["y"].parse().unwrap(),
                },
                size: size,
                used: used,
                goal_data: false,
            })
        } else {
            Err(UnableToParse)
        }
    }
}
