use std::{
    collections::BTreeMap,
    str::FromStr,
    sync::{Mutex, RwLock},
};

#[derive(Debug)]
pub struct UnableToParseStr;

#[derive(Debug, Clone)]
pub enum BitOperation {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

impl FromStr for BitOperation {
    type Err = UnableToParseStr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "and" => Ok(BitOperation::AND),
            "or" => Ok(BitOperation::OR),
            "lshift" => Ok(BitOperation::LSHIFT),
            "rshift" => Ok(BitOperation::RSHIFT),
            _ => Err(UnableToParseStr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperationClass {
    Assignment {
        source: String,
        dest: String,
    },
    Logic {
        source_a: String,
        source_b: String,
        bit_operation: BitOperation,
        dest: String,
    },
    Shift {
        source_a: String,
        source_b: u16,
        bit_operation: BitOperation,
        dest: String,
    },
    Not {
        source: String,
        dest: String,
    },
}

impl FromStr for OperationClass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').map(|x| x.trim()).collect();

        if parts.len() < 3 {
            return Err("Not enough parts".to_string());
        }

        if parts.len() > 5 {
            return Err("Too many parts".to_string());
        }

        let o = parts.iter().rev().take(2).collect::<Vec<_>>();
        if parts
            .iter()
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            != &&"->"
        {
            return Err(format!("{:#?} Not a valid operation", o));
        }

        match parts.len() {
            3 => Ok(OperationClass::Assignment {
                source: parts[0].to_string(),
                dest: parts[2].to_string(),
            }),
            4 => Ok(OperationClass::Not {
                source: parts[1].to_string(),
                dest: parts[3].to_string(),
            }),
            5 => {
                let source_b_result = parts[2].parse::<u16>();

                match source_b_result {
                    Ok(source_b) => Ok(OperationClass::Shift {
                        source_a: parts[0].to_string(),
                        source_b,
                        bit_operation: BitOperation::from_str(parts[1]).unwrap(),
                        dest: parts[4].to_string(),
                    }),
                    Err(_) => Ok(OperationClass::Logic {
                        source_a: parts[0].to_string(),
                        source_b: parts[2].to_string(),
                        bit_operation: BitOperation::from_str(parts[1]).unwrap(),
                        dest: parts[4].to_string(),
                    }),
                }
            }

            _ => Err("Unable to parse operation class".to_string()),
        }
    }
}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref OPERATIONS: RwLock<Vec<OperationClass>> = RwLock::new(Vec::new());
}

lazy_static! {
    pub static ref WIRE_VALUES: RwLock<BTreeMap<String, u16>> = RwLock::new(BTreeMap::new());
}

impl OperationClass {
    pub fn store_operations(operations: Vec<OperationClass>) {
        OPERATIONS.write().unwrap().extend(operations);
    }

    pub fn resolve_operations() {
        for operation in OPERATIONS.read().unwrap().iter() {
            operation.execute();
        }
    }

    pub fn find_operation_by_destination(destination: String) -> Option<OperationClass> {
        for operation in OPERATIONS.read().unwrap().iter() {
            let dest = match operation {
                OperationClass::Assignment { source: _, dest } => dest,
                OperationClass::Logic {
                    source_a: _,
                    source_b: _,
                    bit_operation: _,
                    dest,
                } => dest,
                OperationClass::Shift {
                    source_a: _,
                    source_b: _,
                    bit_operation: _,
                    dest,
                } => dest,
                OperationClass::Not { source: _, dest } => dest,
            };

            if dest == &destination {
                return Some(operation.clone());
            }
        }

        println!("None found for {:#?}", destination);

        None
    }

    pub fn get_value(dest: &str) -> u16 {
        let parse_result = dest.parse::<u16>();
        if let Ok(dest) = parse_result {
            return dest;
        }

        let value = {
            if WIRE_VALUES.read().unwrap().contains_key(dest) {
                Some(WIRE_VALUES.read().unwrap().get(dest).unwrap().clone())
            } else {
                None
            }
        };
        if let Some(value) = value {
            value
        } else {
            OperationClass::find_operation_by_destination(dest.to_string())
                .unwrap()
                .execute()
        }
    }

    pub fn execute(&self) -> u16 {
        match self {
            OperationClass::Assignment { source, dest } => {
                println!("Assigning {} to {}", source, dest);
                let source = OperationClass::get_value(source);

                let mut wire_values = WIRE_VALUES.write().unwrap();
                wire_values.insert(dest.clone(), source);
                source
            }
            OperationClass::Logic {
                source_a,
                source_b,
                bit_operation,
                dest,
            } => {
                println!(
                    "Logic {} {} {:?} {}",
                    source_a, source_b, bit_operation, dest
                );
                let source_a = OperationClass::get_value(source_a);
                let source_b = OperationClass::get_value(source_b);

                let result = perform_bitwise(bit_operation, source_a, source_b);

                let mut wire_values = WIRE_VALUES.write().unwrap();
                wire_values.insert(dest.clone(), result);
                result
            }
            OperationClass::Shift {
                source_a,
                source_b,
                bit_operation,
                dest,
            } => {
                println!(
                    "Shift {} {} {:?} {}",
                    source_a, source_b, bit_operation, dest
                );
                let source_a = OperationClass::get_value(source_a);

                let result = perform_bitwise(bit_operation, source_a, *source_b);

                let mut wire_values = WIRE_VALUES.write().unwrap();
                wire_values.insert(dest.clone(), result);
                result
            }

            OperationClass::Not { source, dest } => {
                println!("Not {} {}", source, dest);
                let source = OperationClass::get_value(source);

                let result = u16::MAX - source;
                let mut wire_values = WIRE_VALUES.write().unwrap();
                wire_values.insert(dest.clone(), result);

                result
            }
        }
    }
}

fn perform_bitwise(bit_operation: &BitOperation, source_a: u16, source_b: u16) -> u16 {
    match bit_operation {
        BitOperation::AND => source_a & source_b,
        BitOperation::OR => source_a | source_b,
        BitOperation::LSHIFT => source_a << source_b,
        BitOperation::RSHIFT => source_a >> source_b,
    }
}
