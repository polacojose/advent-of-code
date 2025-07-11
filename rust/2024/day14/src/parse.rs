use std::{error::Error, str::FromStr};

use glam::I64Vec2;
use regex::Regex;

use crate::{Robot, RobotPlotter};

impl FromStr for Robot {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(r"p=(?<pos>-?[0-9]+,-?[0-9]+) v=(?<vel>-?[0-9]+,-?[0-9]+)")
            .unwrap()
            .captures(s.trim())
            .unwrap();

        let pos = captures["pos"]
            .trim()
            .split_once(",")
            .map(|(x, y)| -> Result<I64Vec2, Box<dyn Error>> {
                Ok(I64Vec2 {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            })
            .ok_or("Malformed Data")??;

        let vel = captures["vel"]
            .trim()
            .split_once(",")
            .map(|(x, y)| -> Result<I64Vec2, Box<dyn Error>> {
                Ok(I64Vec2 {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            })
            .ok_or("Malformed Data")??;

        Ok(Robot { pos, vel })
    }
}

impl FromStr for RobotPlotter {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let robots = s
            .trim()
            .lines()
            .map(|l| l.parse::<Robot>())
            .collect::<Result<_, _>>()?;

        Ok(RobotPlotter { robots })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_robot_parse() {
        assert_eq!(
            "p=0,4 v=3,-3".parse::<Robot>().unwrap(),
            Robot {
                pos: I64Vec2 { x: 0, y: 4 },
                vel: I64Vec2 { x: 3, y: -3 }
            }
        );
    }
}
