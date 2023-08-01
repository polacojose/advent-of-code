use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    sync::RwLock,
};

use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Instruction {
    snd { reg: String },
    set { reg: String, val: String },
    add { reg: String, val: String },
    mul { reg_a: String, val: String },
    r_mod { reg: String, val: String },
    rcv { reg: String },
    jgz { reg: String, val: String },
}

#[derive(Debug)]
pub struct UnableToParse;
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

pub enum ProgramState {
    Executing,
    Stalled,
    Done,
}

pub struct Program<'a> {
    id: isize,
    instructions: &'a Vec<Instruction>,
    pub registers: HashMap<String, isize>,
    pub state: ProgramState,
}

impl<'a> Program<'a> {
    pub fn new(id: isize, instructions: &'a Vec<Instruction>) -> Self {
        let mut registers: HashMap<String, isize> = HashMap::new();
        registers.insert("p".to_owned(), id);

        Self {
            id,
            instructions,
            registers,
            state: ProgramState::Executing,
        }
    }
}

impl Program<'_> {
    pub fn process(&mut self) {
        match self.state {
            ProgramState::Done => return,
            _ => {}
        }

        if let Some(instruction) = self.instructions.get(self.get_register("ip") as usize) {
            println!("======{}======", self.id);
            println!("{:?}", instruction);
            println!("{:?}", self.registers);
            println!("==============");
            self.execute(instruction);
        } else {
            self.state = ProgramState::Done;
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::snd { reg } => {
                if self.id == 0 {
                    MESSAGE_QUEUE_ONE
                        .write()
                        .unwrap()
                        .push_back(self.get_val(reg));
                } else {
                    MESSAGE_QUEUE_ZERO
                        .write()
                        .unwrap()
                        .push_back(self.get_val(reg));
                }

                let snd_val = self.get_register("snd");
                self.set_register("snd", snd_val + 1);

                self.offset_ip(1);
            }
            Instruction::set { reg, val } => {
                let new_val = self.get_val(val);
                self.set_register(reg, new_val);
                self.offset_ip(1);
            }
            Instruction::add { reg, val } => {
                let new_val = &self.get_register(reg) + &self.get_val(val);
                self.set_register(reg, new_val);
                self.offset_ip(1);
            }
            Instruction::mul { reg_a, val } => {
                let new_val = self.get_register(reg_a) * self.get_val(val);
                self.set_register(reg_a, new_val);
                self.offset_ip(1);
            }
            Instruction::r_mod { reg, val } => {
                let new_val = self.get_register(reg) % self.get_val(val);
                self.set_register(reg, new_val);
                self.offset_ip(1);
            }
            Instruction::rcv { reg } => {
                let queue_item_option = {
                    if self.id == 0 {
                        MESSAGE_QUEUE_ZERO.write().unwrap().pop_front()
                    } else {
                        MESSAGE_QUEUE_ONE.write().unwrap().pop_front()
                    }
                };

                if let Some(queue_item) = queue_item_option {
                    self.set_register(reg, queue_item);
                    self.offset_ip(1);
                } else {
                    self.state = ProgramState::Stalled;
                }
            }
            Instruction::jgz { reg, val } => {
                let reg_val = self.get_val(reg);
                if reg_val > 0 {
                    let offset = self.get_val(val);
                    self.offset_ip(offset);
                } else {
                    self.offset_ip(1);
                }
            }
        }
    }

    fn offset_ip(&mut self, val: isize) {
        let ip = self.get_register("ip");
        self.set_register("ip", ip + val);
    }

    fn get_val(&mut self, val: impl AsRef<str>) -> isize {
        if let Ok(num) = val.as_ref().parse::<isize>() {
            return num;
        } else {
            return self.get_register(val);
        }
    }

    fn set_register(&mut self, reg: impl AsRef<str>, val: isize) {
        assert!(
            reg.as_ref().parse::<isize>().is_err(),
            "{}, {}",
            reg.as_ref(),
            val
        );
        self.registers
            .entry(reg.as_ref().to_owned())
            .and_modify(|v| *v = val)
            .or_insert(val);
    }

    fn get_register(&mut self, reg: impl AsRef<str>) -> isize {
        assert!(
            reg.as_ref().parse::<isize>().is_err(),
            "!!{}!!",
            reg.as_ref(),
        );
        *self.registers.entry(reg.as_ref().to_owned()).or_insert(0)
    }
}

lazy_static! {
    static ref MESSAGE_QUEUE_ZERO: RwLock<VecDeque<isize>> = Default::default();
    static ref MESSAGE_QUEUE_ONE: RwLock<VecDeque<isize>> = Default::default();
}
