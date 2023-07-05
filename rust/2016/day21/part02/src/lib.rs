use std::{collections::VecDeque, fs, str::FromStr};

use lazy_static::lazy_static;

#[derive(Debug)]
pub enum Instruction {
    SwapPosition { x: usize, y: usize },
    SwapLetter { x: char, y: char },
    RotateLeft { x: usize },
    RotateRight { x: usize },
    RotateRightOn { x: char },
    ReverseSubStr { x: usize, y: usize },
    Move { x: usize, y: usize },
}

#[derive(Debug)]
pub struct UnabletoParse;
impl FromStr for Instruction {
    type Err = UnabletoParse;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.split(" ").collect::<Vec<_>>();
        match parts[0] {
            "swap" => match parts[1] {
                "position" => Ok(Instruction::SwapPosition {
                    x: parts[2].parse().unwrap(),
                    y: parts[5].parse().unwrap(),
                }),
                "letter" => Ok(Instruction::SwapLetter {
                    x: *parts[2].chars().collect::<Vec<_>>().first().unwrap(),
                    y: *parts[5].chars().collect::<Vec<_>>().first().unwrap(),
                }),
                _ => Err(UnabletoParse),
            },
            "rotate" => match parts[1] {
                "left" => Ok(Self::RotateLeft {
                    x: parts[2].parse().unwrap(),
                }),
                "right" => Ok(Self::RotateRight {
                    x: parts[2].parse().unwrap(),
                }),
                "based" => Ok(Self::RotateRightOn {
                    x: *parts[6].chars().collect::<Vec<_>>().first().unwrap(),
                }),
                _ => Err(UnabletoParse),
            },
            "reverse" => Ok(Self::ReverseSubStr {
                x: parts[2].parse().unwrap(),
                y: parts[4].parse().unwrap(),
            }),
            "move" => Ok(Self::Move {
                x: parts[2].parse().unwrap(),
                y: parts[5].parse().unwrap(),
            }),
            _ => Err(UnabletoParse),
        }
    }
}

impl Instruction {
    pub fn encrypt(&self, s: impl AsRef<str>) -> String {
        match self {
            Self::SwapPosition { x, y } => {
                let mut s = s.as_ref().chars().collect::<Vec<_>>();
                s.swap(*x, *y);
                String::from_iter(s)
            }
            Self::SwapLetter { x, y } => {
                let mut new_string = s.as_ref().to_string();
                new_string = new_string.replace(*x, "_");
                new_string = new_string.replace(*y, x.to_string().as_str());
                new_string = new_string.replace("_", y.to_string().as_str());
                new_string
            }
            Self::RotateLeft { x } => {
                let mut s = s.as_ref().chars().collect::<VecDeque<_>>();
                s.rotate_left(*x);
                String::from_iter(s)
            }
            Self::RotateRight { x } => {
                let mut s = s.as_ref().chars().collect::<VecDeque<_>>();
                s.rotate_right(*x);
                String::from_iter(s)
            }
            Self::RotateRightOn { x } => {
                let position = s.as_ref().find(*x);

                let mut s = s.as_ref().chars().collect::<VecDeque<_>>();
                let mut rotations = 1;
                if let Some(position) = position {
                    if position >= 4 {
                        rotations += position + 1;
                    } else {
                        rotations += position;
                    }
                }

                for _ in 0..rotations {
                    s.rotate_right(1);
                }
                String::from_iter(s)
            }
            Self::ReverseSubStr { x, y } => {
                let substr = s.as_ref()[*x..=*y].to_string();
                let substr_rev =
                    String::from_iter(s.as_ref()[*x..=*y].chars().rev().collect::<Vec<_>>());
                s.as_ref().replace(&substr, &substr_rev)
            }
            Self::Move { x, y } => {
                let mut new_string = s.as_ref().to_string();
                let character = new_string.remove(*x);
                new_string.insert(*y, character);
                new_string.to_string()
            }
        }
    }

    pub fn decrypt(&self, string: impl AsRef<str>) -> String {
        match self {
            Self::SwapPosition { x, y } => {
                let mut string = string.as_ref().chars().collect::<Vec<_>>();
                string.swap(*x, *y);
                String::from_iter(string)
            }
            Self::SwapLetter { x, y } => {
                let mut new_string = string.as_ref().to_string();
                new_string = new_string.replace(*x, "_");
                new_string = new_string.replace(*y, x.to_string().as_str());
                new_string = new_string.replace("_", y.to_string().as_str());
                new_string
            }
            Self::RotateLeft { x } => {
                let mut s = string.as_ref().chars().collect::<VecDeque<_>>();
                s.rotate_right(*x);
                String::from_iter(s)
            }
            Self::RotateRight { x } => {
                let mut s = string.as_ref().chars().collect::<VecDeque<_>>();
                s.rotate_left(*x);
                String::from_iter(s)
            }
            Self::RotateRightOn { x } => {
                let s = string.as_ref();
                let position = s.find(*x);

                let mut s = s.chars().collect::<VecDeque<_>>();
                let mut rotations = 1;
                if let Some(position) = position {
                    if position >= 4 {
                        rotations += position + 1;
                    } else {
                        rotations += position;
                    }
                }

                for _ in 0..rotations {
                    s.rotate_left(1);
                }
                String::from_iter(s)
            }
            Self::ReverseSubStr { x, y } => {
                let substr = string.as_ref()[*x..=*y].to_string();
                let substr_rev =
                    String::from_iter(string.as_ref()[*x..=*y].chars().rev().collect::<Vec<_>>());
                string.as_ref().replace(&substr, &substr_rev)
            }
            Self::Move { x, y } => {
                let mut new_string = string.as_ref().to_string();
                let character = new_string.remove(*y);
                new_string.insert(*x, character);
                new_string.to_string()
            }
        }
    }
}

lazy_static! {
    pub static ref DEMO_SAME_INSTRUCTIONS: Vec<Instruction> = fs::read_to_string("demo-same.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    pub static ref DEMO_INSTRUCTIONS: Vec<Instruction> = fs::read_to_string("demo-input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    pub static ref INSTRUCTIONS: Vec<Instruction> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decrypt() {
        let mut test_string: String = "decab".to_string();
        for instruction in DEMO_INSTRUCTIONS.iter().rev() {
            println!("{:?}", instruction);
            let old_string = test_string.clone();
            test_string = instruction.decrypt(&test_string);
            println!("{} -> {}", old_string, test_string);
        }
        println!("{}", test_string);
    }

    // #[test]
    // fn test_encrypt() {
    //     let mut test_string: String = "abcde".to_string();
    //     for instruction in DEMO_INSTRUCTIONS.iter() {
    //         test_string = instruction.encrypt(&test_string);
    //         println!("{}", test_string);
    //     }
    //     println!("{}", test_string);
    // }
}
