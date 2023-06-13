use lazy_static::lazy_static;
use std::{collections::HashMap, fs, isize, str::FromStr, sync::RwLock};

#[derive(Debug, Clone)]
pub enum Instruction {
    Assign {
        value: u8,
        destination: String,
    },
    Comparison {
        source: String,
        destination_low: String,
        destination_high: String,
    },
}

fn get_endpoint_values_length(endpoint: &String) -> Option<usize> {
    if let Some(values) = FACTORY.read().unwrap().get(endpoint) {
        return Some(values.len());
    }
    None
}

fn store_instruction(source: impl Into<String>, instruction: Instruction) {
    INSTRUCTION_BUFFER.write().unwrap().push(instruction);
}

impl Instruction {
    pub fn execute_stored() {
        let mut i = 0;
        while i < INSTRUCTION_BUFFER.read().unwrap().len() {
            let instruction = INSTRUCTION_BUFFER.read().unwrap()[i].clone();
            i += 1;
            match &instruction {
                Instruction::Comparison {
                    source,
                    destination_low: _,
                    destination_high: _,
                } => {
                    if let Some(length) = get_endpoint_values_length(&source) {
                        if length >= 2 {
                            instruction.execute();
                            let mut instruction_buffer = INSTRUCTION_BUFFER.write().unwrap();
                            instruction_buffer.remove(i - 1);
                            i = 0;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    pub fn store_or_execute(&self) {
        match self {
            Instruction::Assign {
                value: _,
                destination: _,
            } => {
                self.execute();
            }
            Instruction::Comparison {
                source,
                destination_low: _,
                destination_high: _,
            } => {
                let source_values_length = FACTORY.read().unwrap().get(source).cloned();
                match source_values_length {
                    Some(source_values_length) => {
                        if source_values_length.len() < 2 {
                            store_instruction(source, self.clone());
                        } else {
                            self.execute();
                        }
                    }
                    None => {
                        store_instruction(source, self.clone());
                    }
                }
            }
        }
    }
    fn execute(&self) {
        println!("{:?}", self);
        match self {
            Instruction::Assign { value, destination } => {
                FACTORY
                    .write()
                    .unwrap()
                    .entry(destination.clone())
                    .and_modify(|x| x.push(*value))
                    .or_insert(vec![*value]);
                assert!(FACTORY.read().unwrap().get(destination).unwrap().len() <= 2);
            }
            Instruction::Comparison {
                source,
                destination_low,
                destination_high,
            } => {
                let (_, values) = { FACTORY.write().unwrap().remove_entry(source).unwrap() };
                let (high_value, low_value) =
                    (values.iter().max().unwrap(), values.iter().min().unwrap());

                if high_value == &61 && low_value == &17 {
                    println!("!!!!!!!!!!!!!!");
                    println!("{}", source);
                    println!("!!!!!!!!!!!!!!");
                }

                FACTORY
                    .write()
                    .unwrap()
                    .entry(destination_high.clone())
                    .and_modify(|x| x.push(*high_value))
                    .or_insert(vec![*high_value]);

                FACTORY
                    .write()
                    .unwrap()
                    .entry(destination_low.clone())
                    .and_modify(|x| x.push(*low_value))
                    .or_insert(vec![*low_value]);
            }
        }
    }
}

lazy_static! {
    pub static ref FACTORY: RwLock<HashMap<String, Vec<u8>>> = RwLock::new(HashMap::new());
    pub static ref INSTRUCTION_BUFFER: RwLock<Vec<Instruction>> = RwLock::new(Vec::new());
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for Instruction {
    type Err = UnableToParse;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line_parts = line
            .to_lowercase()
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        if line.to_lowercase().contains("gives") {
            Ok(Self::Comparison {
                source: format!("{}{}", line_parts[0], line_parts[1]),
                destination_low: format!("{}{}", line_parts[5], line_parts[6]),
                destination_high: format!("{}{}", line_parts[10], line_parts[11]),
            })
        } else {
            Ok(Self::Assign {
                value: line_parts[1].parse().unwrap(),
                destination: format!("{}{}", line_parts[4], line_parts[5]),
            })
        }
    }
}
