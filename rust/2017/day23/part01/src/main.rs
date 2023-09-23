use std::{borrow::BorrowMut, collections::HashMap, fs, str::FromStr, sync::RwLock};

use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Instruction {
    set { x: String, y: String },
    sub { x: String, y: String },
    mul { x: String, y: String },
    jnz { x: String, y: String },
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for Instruction {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        let x = parts[1].to_owned();
        let y = parts[2].to_owned();

        match parts[0] {
            "set" => Ok(Instruction::set { x, y }),
            "sub" => Ok(Instruction::sub { x, y }),
            "mul" => Ok(Instruction::mul { x, y }),
            "jnz" => Ok(Instruction::jnz { x, y }),
            _ => return Err(UnableToParse),
        }
    }
}

impl Instruction {
    fn execute(&self) {
        match self {
            Instruction::set { x, y } => {
                let val = get_value(y);
                set_register(x, val);
                offset_ip(1);
            }
            Instruction::sub { x, y } => {
                let val = get_value(y);
                let reg_val = get_register(x);
                set_register(x, reg_val - val);
                offset_ip(1);
            }
            Instruction::mul { x, y } => {
                let val = get_value(y);
                let reg_val = get_register(x);
                set_register(x, reg_val * val);
                offset_ip(1);
            }
            Instruction::jnz { x, y } => {
                if get_value(x) != 0 {
                    offset_ip(get_value(y))
                } else {
                    offset_ip(1)
                }
            }
        }
    }
}

lazy_static! {
    static ref REGISTER: RwLock<HashMap<String, isize>> = Default::default();
}

fn get_register(r: impl AsRef<str>) -> isize {
    if let Some(val) = REGISTER.read().unwrap().get(r.as_ref()) {
        return *val;
    }

    REGISTER.write().unwrap().insert(r.as_ref().to_owned(), 0);
    return 0;
}

fn set_register(r: impl AsRef<str>, val: isize) {
    REGISTER.write().unwrap().insert(r.as_ref().to_owned(), val);
}

fn get_value(r: impl AsRef<str>) -> isize {
    if let Ok(val) = r.as_ref().parse::<isize>() {
        return val;
    }

    get_register(r)
}

fn offset_ip(offset: isize) {
    let val = get_register("ip");
    set_register("ip", val + offset);
}

fn main() {
    let instructions = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    //part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &Vec<Instruction>) {
    let mut mul_count = 0;
    while let Some(instruction) = instructions.get(get_register("ip") as usize) {
        instruction.execute();

        match instruction {
            Instruction::mul { x: _, y: _ } => mul_count += 1,
            _ => {}
        }
    }
    println!("Multiply Count: {}", mul_count);
    println!("{:?}", *REGISTER.read().unwrap());
}

fn part2(instructions: &Vec<Instruction>) {
    set_register("a", 1);
    while let Some(instruction) = instructions.get(get_register("ip") as usize) {
        instruction.execute();
        println!("{:?}", instruction);
        println!("{:?}", *REGISTER.read().unwrap());
        println!();
    }
    println!("{:?}", *REGISTER.read().unwrap());
}
