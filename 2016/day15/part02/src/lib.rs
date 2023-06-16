use lazy_static::lazy_static;
use std::{fs, str::FromStr};

#[derive(Debug)]
pub struct Disc {
    pub offset: u32,
    pub positions: u32,
    pub starting_position: u32,
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for Disc {
    type Err = UnableToParse;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line
            .split(" ")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        Ok(Disc {
            offset: parts[1].replace("#", "").parse().unwrap(),
            positions: parts[3].replace(".", "").parse().unwrap(),
            starting_position: parts[11].replace(".", "").parse().unwrap(),
        })
    }
}

impl Disc {
    pub fn get_position_relative_start(&self, seconds: u32) -> u32 {
        (seconds + self.starting_position + self.offset) % self.positions
    }
}

lazy_static! {
    pub static ref DISCS: Vec<Disc> = {
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .into_iter()
            .map(|line| line.parse().unwrap())
            .collect()
    };
}
