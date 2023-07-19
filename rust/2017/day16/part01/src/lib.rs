use std::{fs, str::FromStr};

#[derive(Debug)]
pub enum Instruction {
    Spin { x: isize },
    Exchange { a: isize, b: isize },
    Partner { a: char, b: char },
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for Instruction {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s.chars().nth(0).unwrap() {
            's' => Ok(Self::Spin {
                x: s.get(1..).unwrap().parse().unwrap(),
            }),
            'x' => {
                let parts = s.get(1..).unwrap().split("/").collect::<Vec<_>>();
                Ok(Self::Exchange {
                    a: parts[0].parse().unwrap(),
                    b: parts[1].parse().unwrap(),
                })
            }
            'p' => {
                let parts = s.get(1..).unwrap().split("/").collect::<Vec<_>>();
                Ok(Self::Partner {
                    a: parts[0].chars().nth(0).unwrap(),
                    b: parts[1].chars().nth(0).unwrap(),
                })
            }
            _ => Err(UnableToParse),
        }
    }
}

pub fn get_instructions() -> Vec<Instruction> {
    let file_string = fs::read_to_string("input.txt").unwrap();
    file_string
        .split(",")
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect()
}

impl Instruction {
    pub fn execute(&self, operand: &mut Vec<char>) {
        match self {
            Instruction::Spin { x } => {
                let last = (operand.len() as isize - x) as usize;
                let mut last = operand.drain(..last).collect::<Vec<_>>();
                operand.append(&mut last);
            }
            Instruction::Exchange { a, b } => {
                operand.swap(*a as usize, *b as usize);
            }
            Instruction::Partner { a, b } => {
                let a = operand.iter().position(|x| x == a).unwrap();
                let b = operand.iter().position(|x| x == b).unwrap();
                operand.swap(a, b);
            }
        }
    }
}
