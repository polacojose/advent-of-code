use lazy_static::lazy_static;
use std::{collections::HashMap, fs, str::FromStr, sync::RwLock};

lazy_static! {
    #[allow(non_upper_case_globals)]
    pub static ref REGISTERS: RwLock<HashMap<String, i32>> = {
        let mut registers_map = HashMap::new();
        registers_map.insert("a".to_string(), 0);
        registers_map.insert("b".to_string(), 0);
        registers_map.insert("ip".to_string(), 0);
        RwLock::new(registers_map)
    };

    pub static ref INSTRUCTIONS: Vec<Instruction> = {
        fs::read_to_string("input.txt").unwrap().lines().map(|line|line.parse().unwrap()).collect()
    };
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Instruction {
    hlf { register: String },
    tpl { register: String },
    inc { register: String },
    jmp { offset: i32 },
    jie { register: String, offset: i32 },
    jio { register: String, offset: i32 },
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for Instruction {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .replace(",", "")
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        match parts[0].as_str() {
            "hlf" => Ok(Instruction::hlf {
                register: parts[1].to_string(),
            }),
            "tpl" => Ok(Instruction::tpl {
                register: parts[1].to_string(),
            }),
            "inc" => Ok(Instruction::inc {
                register: parts[1].to_string(),
            }),
            "jmp" => Ok(Instruction::jmp {
                offset: parts[1].parse().unwrap(),
            }),
            "jie" => Ok(Instruction::jie {
                register: parts[1].to_string(),
                offset: parts[2].parse().unwrap(),
            }),
            "jio" => Ok(Instruction::jio {
                register: parts[1].to_string(),
                offset: parts[2].parse().unwrap(),
            }),
            _ => Err(UnableToParse),
        }
    }
}

impl Instruction {
    pub fn execute(&self) {
        match self {
            Instruction::hlf { register } => {
                *REGISTERS.write().unwrap().get_mut(register).unwrap() /= 2;
                *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
            }
            Instruction::tpl { register } => {
                *REGISTERS.write().unwrap().get_mut(register).unwrap() *= 3;
                *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
            }
            Instruction::inc { register } => {
                *REGISTERS.write().unwrap().get_mut(register).unwrap() += 1;
                *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
            }
            Instruction::jmp { offset } => {
                *REGISTERS.write().unwrap().get_mut("ip").unwrap() += offset;
                assert!(REGISTERS.read().unwrap().get("ip").unwrap().is_positive());
            }
            Instruction::jie { register, offset } => {
                let register_value = *REGISTERS.read().unwrap().get(register).unwrap();

                if register_value % 2 == 0 {
                    *REGISTERS.write().unwrap().get_mut("ip").unwrap() += offset;
                } else {
                    *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
                }

                assert!(REGISTERS.read().unwrap().get("ip").unwrap().is_positive());
            }
            Instruction::jio { register, offset } => {
                let register_value = *REGISTERS.read().unwrap().get(register).unwrap();

                if register_value == 1 {
                    *REGISTERS.write().unwrap().get_mut("ip").unwrap() += offset;
                } else {
                    *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
                }

                assert!(REGISTERS.read().unwrap().get("ip").unwrap().is_positive());
            }
        }
    }
}
