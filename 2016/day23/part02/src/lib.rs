use std::{collections::HashMap, fs, str::FromStr, sync::RwLock};

use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Instruction {
    cpy { src: String, dest_register: String },
    inc { register: String },
    dec { register: String },
    jnz { src: String, offset: String },
    tgl { src: String },
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
            "tgl" => Ok(Instruction::tgl {
                src: parts[1].to_owned(),
            }),
            _ => Err(UnableToParse),
        }
    }
}

impl Instruction {
    pub fn execute(&self) {
        match &self {
            Instruction::cpy { src, dest_register } => {
                let instruction_a = Self::get_instruction(get_instruction_pointer() + 0);
                let instruction_b = Self::get_instruction(get_instruction_pointer() + 1);
                let instruction_c = Self::get_instruction(get_instruction_pointer() + 2);
                let instruction_d = Self::get_instruction(get_instruction_pointer() + 3);
                let instruction_e = Self::get_instruction(get_instruction_pointer() + 4);
                match (
                    &instruction_a,
                    &instruction_b,
                    &instruction_c,
                    &instruction_d,
                    &instruction_e,
                ) {
                    (
                        Instruction::cpy {
                            src: src_a,
                            dest_register: dest_register_a,
                        },
                        Instruction::inc {
                            register: register_b,
                        },
                        Instruction::dec {
                            register: register_c,
                        },
                        Instruction::jnz {
                            src: src_d,
                            offset: offset_d,
                        },
                        Instruction::dec {
                            register: register_e,
                        },
                    ) => {
                        let inner_offset = Self::register_source_value(&offset_d);

                        if inner_offset == -2 {
                            Self::set_register_value(&dest_register_a, 0);

                            let product_a = Self::register_source_value(&src_a);
                            let product_b = Self::register_source_value(&register_e);
                            let org_register_b = Self::register_source_value(&register_b);
                            let result = org_register_b as usize + (product_a * product_b) as usize;

                            Self::set_register_value(&register_b, result);
                            Self::set_register_value(&register_e, 0);

                            Self::increment_instruction_pointer();
                            Self::increment_instruction_pointer();
                            Self::increment_instruction_pointer();
                            Self::increment_instruction_pointer();
                            Self::increment_instruction_pointer();
                            Self::increment_instruction_pointer();
                            return;
                        }
                    }
                    _ => {}
                }
                //     println!("{} && {}", source_value, offset_value);

                let source_value = Self::register_source_value(src) as usize;

                assert!(dest_register.parse::<usize>().is_err());

                *REGISTERS.write().unwrap().get_mut(dest_register).unwrap() = source_value;
                Self::increment_instruction_pointer();
            }
            Instruction::inc { register } => {
                *REGISTERS.write().unwrap().get_mut(register).unwrap() += 1;
                Self::increment_instruction_pointer();
            }
            Instruction::dec { register } => {
                let source_value = Self::register_source_value(register) as usize;

                *REGISTERS.write().unwrap().get_mut(register).unwrap() =
                    source_value.saturating_sub(1);
                Self::increment_instruction_pointer();
            }
            Instruction::jnz { src, offset } => {
                let mut source_value = Self::register_source_value(src);
                let mut offset_value = Self::register_source_value(offset);

                if offset_value == -5 {
                    let instruction_a = Self::get_instruction(get_instruction_pointer() - 5);
                    let instruction_b = Self::get_instruction(get_instruction_pointer() - 4);
                    let instruction_c = Self::get_instruction(get_instruction_pointer() - 3);
                    let instruction_d = Self::get_instruction(get_instruction_pointer() - 2);
                    let instruction_e = Self::get_instruction(get_instruction_pointer() - 1);
                    match (
                        &instruction_a,
                        &instruction_b,
                        &instruction_c,
                        &instruction_d,
                        &instruction_e,
                    ) {
                        (
                            Instruction::cpy {
                                src: src_a,
                                dest_register: dest_register_a,
                            },
                            Instruction::inc {
                                register: register_b,
                            },
                            Instruction::dec {
                                register: register_c,
                            },
                            Instruction::jnz {
                                src: src_d,
                                offset: offset_d,
                            },
                            Instruction::dec {
                                register: register_e,
                            },
                        ) => {
                            let inner_offset = Self::register_source_value(&offset_d);

                            if inner_offset == -2 {
                                println!("Before: {:?}", REGISTERS.read().unwrap());

                                Self::set_register_value(&dest_register_a, 0);

                                let product_a = Self::register_source_value(&src_a);
                                let product_b = Self::register_source_value(&register_e);
                                Self::set_register_value(
                                    &register_b,
                                    (product_a * product_b) as usize,
                                );
                                Self::set_register_value(&register_e, 0);

                                // println!("{:?} -> {:?}", a, b);
                                // println!("{:?} -> {:?}", instruction_b, instruction_a);
                                // println!("Before: {:?}", REGISTERS.read().unwrap());

                                // println!("{:?} -> {:?}", a, b);
                                // println!("After: {:?}", REGISTERS.read().unwrap());

                                // println!("Optimize!!!!");

                                println!("After: {:?}", REGISTERS.read().unwrap());
                                source_value = Self::register_source_value(src);
                                offset_value = Self::register_source_value(offset);
                            }
                        }
                        _ => {}
                    }
                    //     println!("{} && {}", source_value, offset_value);
                }

                if source_value != 0 && offset_value != 0 {
                    let instruction_pointer = REGISTERS.read().unwrap().get("ip").unwrap().clone();
                    *REGISTERS.write().unwrap().get_mut("ip").unwrap() =
                        (instruction_pointer as isize + offset_value) as usize;
                } else {
                    Self::increment_instruction_pointer();
                }
            }
            Instruction::tgl { src } => {
                Self::toggle_instruction(src);
                Self::increment_instruction_pointer();
            }
        }
    }

    #[inline]
    fn set_register_value(register: &String, val: usize) {
        *REGISTERS.write().unwrap().get_mut(register).unwrap() = val;
    }

    fn get_instruction(index: usize) -> Instruction {
        INSTRUCTIONS.read().unwrap().get(index).unwrap().clone()
    }

    #[inline]
    fn increment_instruction_pointer() {
        *REGISTERS.write().unwrap().get_mut("ip").unwrap() += 1;
    }

    fn toggle_instruction(src: &String) {
        let source_value = Self::register_source_value(src) as usize + get_instruction_pointer();
        if let Some(instruction) = INSTRUCTIONS.try_write().unwrap().get_mut(source_value) {
            print!("{:?} -> ", instruction);
            match instruction {
                Instruction::inc { register } => {
                    *instruction = Instruction::dec {
                        register: register.clone(),
                    };
                }
                Instruction::dec { register } => {
                    *instruction = Instruction::inc {
                        register: register.clone(),
                    };
                }
                Instruction::tgl { src } => {
                    *instruction = Instruction::inc {
                        register: src.clone(),
                    };
                }
                Instruction::jnz { src, offset } => {
                    *instruction = Instruction::cpy {
                        src: src.clone(),
                        dest_register: offset.clone(),
                    };
                }
                Instruction::cpy { src, dest_register } => {
                    *instruction = Instruction::jnz {
                        src: src.clone(),
                        offset: dest_register.clone(),
                    };
                }
            }
            println!("{:?}", instruction);
        }
    }

    #[inline]
    fn register_source_value(src: &String) -> isize {
        if let Ok(source_value) = src.parse::<isize>() {
            source_value
        } else {
            REGISTERS.read().unwrap().get(src).unwrap().clone() as isize
        }
    }
}

pub fn get_instruction_pointer() -> usize {
    REGISTERS.read().unwrap().get("ip").unwrap().clone()
}

lazy_static! {
    pub static ref REGISTERS: RwLock<HashMap<String, usize>> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("a".to_string(), 12);
        hash_map.insert("b".to_string(), 0);
        hash_map.insert("c".to_string(), 0);
        hash_map.insert("d".to_string(), 0);
        hash_map.insert("ip".to_string(), 0);
        RwLock::new(hash_map)
    };
    pub static ref INSTRUCTIONS: RwLock<Vec<Instruction>> = {
        let file_string = fs::read_to_string("input.txt").unwrap();
        let mut instructions = Vec::new();
        for line in file_string.lines() {
            instructions.push(line.parse().unwrap());
        }
        RwLock::new(instructions)
    };
}
