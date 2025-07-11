use std::{error::Error, str::FromStr};

use crate::{ComboOperand, Comp, Memory, Opcode};

impl TryFrom<u64> for ComboOperand {
    type Error = Box<dyn Error>;

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(match v {
            0 => ComboOperand::Zero,
            1 => ComboOperand::One,
            2 => ComboOperand::Two,
            3 => ComboOperand::Three,
            4 => ComboOperand::Four,
            5 => ComboOperand::Five,
            6 => ComboOperand::Six,
            7 => ComboOperand::Seven,
            _ => return Err(Box::from("Invalid operand")),
        })
    }
}

impl TryFrom<&[u64]> for Opcode {
    type Error = Box<dyn Error>;

    fn try_from(v: &[u64]) -> Result<Self, Self::Error> {
        if v.len() != 2 {
            panic!("Opcode must be of length 2");
        }

        let (a, b) = (v[0], v[1]);

        return Ok(match a {
            0 => Opcode::ADV(b.try_into()?),
            1 => Opcode::BXL(b.try_into()?),
            2 => Opcode::BST(b.try_into()?),
            3 => Opcode::JNZ(b.try_into()?),
            4 => Opcode::BXC(),
            5 => Opcode::OUT(b.try_into()?),
            6 => Opcode::BDV(b.try_into()?),
            7 => Opcode::CDV(b.try_into()?),
            _ => return Err(Box::from("String must be of length 2")),
        });
    }
}

impl FromStr for Comp {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (registers_str, program_str) = s.trim().split_once("\n\n").ok_or("Invalid string")?;

        let registers = registers_str
            .lines()
            .map(|l| {
                Ok(l.split_whitespace()
                    .last()
                    .ok_or("Invalid register data")?
                    .parse::<u64>()?)
            })
            .collect::<Result<Vec<u64>, Self::Err>>()?;

        let program = program_str
            .split_whitespace()
            .last()
            .ok_or("Invalid program data")?
            .split(",")
            .collect::<String>()
            .chars()
            .map(|c| c.to_digit(10).map(|i| i as u64))
            .collect::<Option<Vec<u64>>>()
            .ok_or("Invalid program")?;

        Ok(Self {
            program,
            memory: Memory {
                ip: 0,
                register_a: registers[0],
                register_b: registers[1],
                register_c: registers[2],
                out: vec![],
            },
        })
    }
}
