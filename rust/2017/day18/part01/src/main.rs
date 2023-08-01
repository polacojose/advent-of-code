use std::{collections::HashMap, fs, str::FromStr, sync::RwLock};

use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Instruction {
    snd { reg: String },
    set { reg: String, val: String },
    add { reg: String, val: String },
    mul { reg_a: String, val: String },
    r_mod { reg: String, val: String },
    rcv { reg: String },
    jgz { reg: String, val: String },
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for Instruction {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        let instruction = match parts[0] {
            "snd" => Instruction::snd {
                reg: parts[1].to_owned(),
            },
            "set" => Instruction::set {
                reg: parts[1].to_owned(),
                val: parts[2].to_owned(),
            },
            "add" => Instruction::add {
                reg: parts[1].to_owned(),
                val: parts[2].to_owned(),
            },
            "mul" => Instruction::mul {
                reg_a: parts[1].to_owned(),
                val: parts[2].to_owned(),
            },
            "mod" => Instruction::r_mod {
                reg: parts[1].to_owned(),
                val: parts[2].to_owned(),
            },
            "rcv" => Instruction::rcv {
                reg: parts[1].to_owned(),
            },
            "jgz" => Instruction::jgz {
                reg: parts[1].to_owned(),
                val: parts[2].to_owned(),
            },
            _ => return Err(UnableToParse),
        };

        Ok(instruction)
    }
}

impl Instruction {
    fn execute(&self) {
        match self {
            Instruction::snd { reg } => {
                set_register("snd", get_register(reg));
                offset_ip(1);
            }
            Instruction::set { reg, val } => {
                set_register(reg, get_val(val));
                offset_ip(1);
            }
            Instruction::add { reg, val } => {
                set_register(reg, get_register(reg) + get_val(val));
                offset_ip(1);
            }
            Instruction::mul { reg_a, val } => {
                set_register(reg_a, get_register(reg_a) * get_val(val));
                offset_ip(1);
            }
            Instruction::r_mod { reg, val } => {
                set_register(reg, get_register(reg) % get_val(val));
                offset_ip(1);
            }
            Instruction::rcv { reg } => {
                let val = get_register(reg);
                if val != 0 {
                    set_register("recover", get_register("snd"));
                    set_register("break", 1);
                }
                offset_ip(1);
            }
            Instruction::jgz { reg, val } => {
                let reg_val = get_register(reg);
                if reg_val > 0 {
                    offset_ip(get_val(val));
                } else {
                    offset_ip(1);
                }
            }
        }
    }
}

lazy_static! {
    static ref REGISTERS: RwLock<HashMap<String, isize>> = Default::default();
}

fn offset_ip(val: isize) {
    let ip = get_register("ip");
    set_register("ip", ip + val);
}

fn get_val(val: impl AsRef<str>) -> isize {
    if let Ok(num) = val.as_ref().parse::<isize>() {
        return num;
    } else {
        return get_register(val);
    }
}

fn set_register(reg: impl AsRef<str>, val: isize) {
    REGISTERS
        .write()
        .unwrap()
        .entry(reg.as_ref().to_owned())
        .and_modify(|v| *v = val)
        .or_insert(val);
}

fn get_register(reg: impl AsRef<str>) -> isize {
    *REGISTERS
        .write()
        .unwrap()
        .entry(reg.as_ref().to_owned())
        .or_insert(0)
}

fn main() {
    let instructions = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    while let Some(instruction) = instructions.get(get_register("ip") as usize) {
        instruction.execute();
        println!("{:?}", instruction);
        println!("{:?}", *REGISTERS.read().unwrap());

        if get_register("break") == 1 {
            break;
        }
    }

    println!("{:?}", *REGISTERS.read().unwrap());
}
