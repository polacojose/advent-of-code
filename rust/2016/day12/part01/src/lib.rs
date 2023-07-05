use std::{collections::HashMap, fs, str::FromStr, sync::RwLock};

use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Instruction {
    cpy { src: String, dest_register: String },
    inc { register: String },
    dec { register: String },
    jnz { src: String, offset: isize },
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for Instruction {
    type Err = UnableToParse;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        match parts[0].as_str() {
            "cpy" => Ok(Instruction::cpy {
                src: parts[1].to_owned(),
                dest_register: parts[2].to_owned(),
            }),
            "inc" => Ok(Instruction::inc {
                register: parts[1].to_owned(),
            }),
            "dec" => Ok(Instruction::dec {
                register: parts[1].to_owned(),
            }),
            "jnz" => Ok(Instruction::jnz {
                src: parts[1].to_owned(),
                offset: parts[2].parse().unwrap(),
            }),
            _ => Err(UnableToParse),
        }
    }
}

impl Instruction {
    pub fn execute(&self) {
        match &self {
            Instruction::cpy { src, dest_register } => {
                let source_value = if let Ok(source_value) = src.parse::<usize>() {
                    source_value
                } else {
                    REGISTERS.read().unwrap().get(src).unwrap().clone()
                };

                *REGISTERS.write().unwrap().get_mut(dest_register).unwrap() = source_value;
                *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
            }
            Instruction::inc { register } => {
                *REGISTERS.write().unwrap().get_mut(register).unwrap() += 1;
                *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
            }
            Instruction::dec { register } => {
                *REGISTERS.write().unwrap().get_mut(register).unwrap() -= 1;
                *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
            }
            Instruction::jnz { src, offset } => {
                let source_value = if let Ok(source_value) = src.parse::<usize>() {
                    source_value
                } else {
                    REGISTERS.read().unwrap().get(src).unwrap().clone()
                };
                if source_value != 0 {
                    let instruction_pointer = REGISTERS.read().unwrap().get("ip").unwrap().clone();
                    *REGISTERS.write().unwrap().get_mut("ip").unwrap() =
                        (instruction_pointer as isize + *offset) as usize;
                } else {
                    *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
                }
            }
        }
    }
}

lazy_static! {
    pub static ref REGISTERS: RwLock<HashMap<String, usize>> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("a".to_string(), 0);
        hash_map.insert("b".to_string(), 0);
        hash_map.insert("c".to_string(), 1);
        hash_map.insert("d".to_string(), 0);
        hash_map.insert("ip".to_string(), 0);
        RwLock::new(hash_map)
    };
    pub static ref INSTRUCTIONS: Vec<Instruction> = {
        let file_string = fs::read_to_string("input.txt").unwrap();
        let mut instructions = Vec::new();
        for line in file_string.lines() {
            instructions.push(line.parse().unwrap());
        }
        instructions
    };
}
