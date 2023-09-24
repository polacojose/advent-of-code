use std::{collections::HashMap, fs, str::FromStr, sync::RwLock};

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

        if parts.len() < 3 {
            return Err(UnableToParse);
        }

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
                set_reg(x, val);
                offset_ip(1);
            }
            Instruction::sub { x, y } => {
                let val = get_value(y);
                let reg_val = get_reg(x);
                set_reg(x, reg_val - val);
                offset_ip(1);
            }
            Instruction::mul { x, y } => {
                let val = get_value(y);
                let reg_val = get_reg(x);
                set_reg(x, reg_val * val);
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

fn get_reg(r: impl AsRef<str>) -> isize {
    if let Some(val) = REGISTER.read().unwrap().get(r.as_ref()) {
        return *val;
    }

    REGISTER.write().unwrap().insert(r.as_ref().to_owned(), 0);
    return 0;
}

fn set_reg(r: impl AsRef<str>, val: isize) {
    REGISTER.write().unwrap().insert(r.as_ref().to_owned(), val);
}

fn get_value(r: impl AsRef<str>) -> isize {
    if let Ok(val) = r.as_ref().parse::<isize>() {
        return val;
    }

    get_reg(r)
}

fn offset_ip(offset: isize) {
    let val = get_reg("ip");
    set_reg("ip", val + offset);
}

fn main() {
    let instructions = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<Instruction>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    //part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &Vec<Instruction>) {
    let mut mul_count = 0;
    while let Some(instruction) = instructions.get(get_reg("ip") as usize) {
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
    set_reg("a", 1);
    while let Some(instruction) = instructions.get(get_reg("ip") as usize) {
        println!("{:?}", *REGISTER.read().unwrap());
        match instruction {
            Instruction::jnz { x, y: _ } => {
                if x == "g" && get_reg("g") != 0 {
                    if get_reg("ip") == 19 {
                        set_reg("g", 0);
                        set_reg("e", get_reg("b"));
                    } else if get_reg("ip") == 23 {
                        set_reg("g", 0);
                        set_reg("d", get_reg("b"));

                        let mut f = 1;
                        let b = get_reg("b");
                        for d in 2..b {
                            if b % d == 0 {
                                f = 0;
                                break;
                            }
                        }
                        set_reg("f", f);
                    }
                }
            }
            _ => {}
        }

        println!("{}: {:?}", get_reg("ip"), instruction);
        instruction.execute();
        println!("{:?}", *REGISTER.read().unwrap());
        println!();
    }
    println!("{:?}", *REGISTER.read().unwrap());
}

fn pseudo() {
    let b = 123;
    let mut d = 2;
    let mut f = 1;

    while d < b {
        let mut e = 2;
        while e < b {
            if d * e == b {
                f = 0;
            }
            e += 1;

            if e == b {
                break;
            }
        }
        d += 1;
    }
}
