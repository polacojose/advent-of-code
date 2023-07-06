use std::{collections::HashMap, str::FromStr, sync::RwLock};

use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum ConditionType {
    gt,
    lt,
    eq,
    gte,
    lte,
    neq,
}

impl FromStr for ConditionType {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(ConditionType::gt),
            "<" => Ok(ConditionType::lt),
            "==" => Ok(ConditionType::eq),
            ">=" => Ok(ConditionType::gte),
            "<=" => Ok(ConditionType::lte),
            "!=" => Ok(ConditionType::neq),
            _ => Err(UnableToParse),
        }
    }
}

#[derive(Debug)]
struct Condition {
    register: String,
    amount: isize,
    condition_type: ConditionType,
}

impl Condition {
    fn check(&self) -> bool {
        match self.condition_type {
            ConditionType::gt => get_register(&self.register) as isize > self.amount,
            ConditionType::lt => (get_register(&self.register) as isize) < self.amount,
            ConditionType::eq => get_register(&self.register) as isize == self.amount,
            ConditionType::gte => get_register(&self.register) as isize >= self.amount,
            ConditionType::lte => get_register(&self.register) as isize <= self.amount,
            ConditionType::neq => get_register(&self.register) as isize != self.amount,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum InstructionType {
    inc,
    dec,
}

#[derive(Debug)]
pub struct Instruction {
    register: String,
    amount: isize,
    instruction_type: InstructionType,
    condition: Condition,
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for Instruction {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let instruction_type = match parts[1] {
            "inc" => InstructionType::inc,
            "dec" => InstructionType::dec,
            _ => return Err(UnableToParse),
        };

        Ok(Instruction {
            register: parts[0].to_string(),
            amount: parts[2].parse().unwrap(),
            instruction_type,
            condition: Condition {
                register: parts[4].to_string(),
                amount: parts[6].parse().unwrap(),
                condition_type: parts[5].parse().unwrap(),
            },
        })
    }
}

impl Instruction {
    pub fn execute(&self) {
        match self.instruction_type {
            InstructionType::inc => {
                if self.condition.check() {
                    inc_register(&self.register, self.amount);
                }
            }
            InstructionType::dec => {
                if self.condition.check() {
                    dec_register(&self.register, self.amount);
                }
            }
        };
    }
}

fn inc_register(register: &String, val: isize) {
    REGISTERS
        .write()
        .unwrap()
        .entry(register.clone())
        .and_modify(|v| *v += val)
        .or_insert(val);
}

fn dec_register(register: &String, val: isize) {
    REGISTERS
        .write()
        .unwrap()
        .entry(register.clone())
        .and_modify(|v| *v -= val)
        .or_insert(0 - val);
}

fn get_register(register: &String) -> isize {
    REGISTERS
        .write()
        .unwrap()
        .entry(register.clone())
        .or_insert(0)
        .clone()
}

lazy_static! {
    pub static ref REGISTERS: RwLock<HashMap<String, isize>> = Default::default();
}
