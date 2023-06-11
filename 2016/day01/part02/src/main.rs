use std::{fs, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Heading {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl From<u32> for Heading {
    fn from(value: u32) -> Self {
        match value {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _ => panic!("Unknown value"),
        }
    }
}

impl Heading {
    fn add_turn(&mut self, turn: Turn) {
        let mut heading_num: i32 = *self as i32;

        heading_num += turn as i32;

        while heading_num < 0 {
            heading_num += 4;
        }

        while heading_num >= 4 {
            heading_num -= 4;
        }

        *self = Heading::from(heading_num as u32)
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left = -1,
    Right = 1,
}

#[derive(Debug)]
struct Instruction {
    turn: Turn,
    steps: u16,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for Instruction {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        match s.chars().into_iter().next().unwrap() {
            'l' => Ok(Instruction {
                turn: Turn::Left,
                steps: s[1..].parse().unwrap(),
            }),
            'r' => Ok(Instruction {
                turn: Turn::Right,
                steps: s[1..].parse().unwrap(),
            }),
            _ => Err(UnableToParse),
        }
    }
}

fn main() {
    let instruction_sets = fs::read_to_string("input.txt").unwrap();

    for line in instruction_sets.lines() {
        let instructions = line
            .split(",")
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();

        let mut coord: (i16, i16) = (0, 0);
        let mut visited_coords = Vec::new();
        visited_coords.push(coord);
        let mut heading = Heading::North;
        for instruction in instructions {
            println!("{:?}", instruction);

            heading.add_turn(instruction.turn);

            let mut found = false;
            for _ in 0..instruction.steps {
                match heading {
                    Heading::North => coord.1 += 1,
                    Heading::South => coord.1 -= 1,
                    Heading::East => coord.0 += 1,
                    Heading::West => coord.0 -= 1,
                }

                if visited_coords.contains(&coord) {
                    found = true;
                    break;
                } else {
                    visited_coords.push(coord);
                }
            }

            if found {
                break;
            }
        }

        println!("Coord: {:?}", coord);
        println!("Blocks away = {}", (coord.0).abs() + (coord.1).abs());
    }
}
