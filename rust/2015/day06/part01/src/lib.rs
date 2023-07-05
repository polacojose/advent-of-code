use std::str::FromStr;

use regex::{Captures, Regex};

#[derive(Debug)]
pub struct Vector {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub upper_left: Vector,
    pub lower_right: Vector,
}

#[derive(Debug)]
pub enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
pub struct Instruction {
    pub command: Command,
    pub rect: Rectangle,
}

trait ParsableInstruction {
    fn parse_command(caps: &Captures) -> Command;
    fn parse_rectangle(caps: &Captures) -> Rectangle;
}

impl ParsableInstruction for Instruction {
    fn parse_command(caps: &Captures) -> Command {
        let command: Option<Command> = if caps["command"].to_lowercase() == "turn on" {
            return Command::TurnOn;
        } else if caps["command"].to_lowercase() == "turn off" {
            return Command::TurnOff;
        } else if caps["command"].to_lowercase() == "toggle" {
            return Command::Toggle;
        } else {
            None
        };

        assert!(command.is_some());

        command.unwrap()
    }

    fn parse_rectangle(caps: &Captures) -> Rectangle {
        let vec1 = Vector {
            x: caps["x1"].parse().unwrap(),
            y: caps["y1"].parse().unwrap(),
        };

        let vec2 = Vector {
            x: caps["x2"].parse().unwrap(),
            y: caps["y2"].parse().unwrap(),
        };

        Rectangle {
            upper_left: vec1,
            lower_right: vec2,
        }
    }
}

#[derive(Debug)]
pub struct UnableToParseInstruction;
impl FromStr for Instruction {
    type Err = UnableToParseInstruction;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(UnableToParseInstruction);
        }
        let re = Regex::new(
            r"(?x)(?P<command>turn\son|turn\soff|toggle)\s(?P<x1>\d+),(?P<y1>\d+)\s(through)\s(?P<x2>\d+),(?P<y2>\d+)",
        ).unwrap();
        let caps = re.captures(s).unwrap();

        Ok(Instruction {
            command: Instruction::parse_command(&caps),
            rect: Instruction::parse_rectangle(&caps),
        })
    }
}
