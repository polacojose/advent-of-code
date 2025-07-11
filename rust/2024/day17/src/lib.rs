use std::{
    cmp,
    error::Error,
    ops::{Range, RangeInclusive},
};

mod parse;

#[allow(non_camel_case_types)]
enum Opcode {
    ADV(ComboOperand),
    BXL(u64),
    BST(ComboOperand),
    JNZ(u64),
    BXC(),
    OUT(ComboOperand),
    BDV(ComboOperand),
    CDV(ComboOperand),
}

impl Opcode {
    pub fn exec(&self, mem: &mut Memory) {
        match self {
            Opcode::ADV(combo_operand) => {
                mem.register_a = mem.register_a / 2_u64.pow(combo_operand.get(mem) as u32);
                mem.ip += 2;
            }
            Opcode::BXL(x) => {
                mem.register_b = mem.register_b ^ x;
                mem.ip += 2;
            }
            Opcode::BST(combo_operand) => {
                mem.register_b = combo_operand.get(mem) % 8;
                mem.ip += 2;
            }
            Opcode::JNZ(x) => {
                if mem.register_a == 0 {
                    mem.ip += 2;
                } else {
                    mem.ip = *x as usize;
                }
            }
            Opcode::BXC() => {
                mem.register_b = mem.register_b ^ mem.register_c;
                mem.ip += 2;
            }
            Opcode::OUT(combo_operand) => {
                mem.out.push(combo_operand.get(mem) % 8);
                mem.ip += 2;
            }
            Opcode::BDV(combo_operand) => {
                mem.register_b = mem.register_a / 2_u64.pow(combo_operand.get(mem) as u32);
                mem.ip += 2;
            }
            Opcode::CDV(combo_operand) => {
                mem.register_c = mem.register_a / 2_u64.pow(combo_operand.get(mem) as u32);
                mem.ip += 2;
            }
        }
    }
}

enum ComboOperand {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl ComboOperand {
    #[inline]
    fn get(&self, mem: &Memory) -> u64 {
        match self {
            ComboOperand::Zero => 0,
            ComboOperand::One => 1,
            ComboOperand::Two => 2,
            ComboOperand::Three => 3,
            ComboOperand::Four => mem.register_a,
            ComboOperand::Five => mem.register_b,
            ComboOperand::Six => mem.register_c,
            ComboOperand::Seven => panic!("Reserved combo operand"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    ip: usize,
    register_a: u64,
    register_b: u64,
    register_c: u64,
    pub out: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct Comp {
    program: Vec<u64>,
    pub memory: Memory,
}

impl Comp {
    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let c: Opcode = self
                .program
                .get(self.memory.ip..self.memory.ip + 2)
                .ok_or("IP out of bounds.")?
                .try_into()?;
            c.exec(&mut self.memory);
        }
    }
}

pub fn find_comp_copy(init_comp: &Comp) -> Result<u64, &str> {
    binary_reg_search(init_comp, 0..u64::MAX)
}

pub fn binary_reg_search(init_comp: &Comp, reg_a_space: Range<u64>) -> Result<u64, &str> {
    let midpoint_reg = reg_a_space.start / 2 + reg_a_space.end / 2;

    let result = {
        let mut comp = init_comp.clone();
        comp.memory.register_a = midpoint_reg;
        let _ = comp.execute();
        comp.memory.out
    };

    if result == init_comp.program {
        return Ok(midpoint_reg);
    }

    if reg_a_space.end == reg_a_space.start {
        return Err("Search space exhausted.");
    }

    if reg_a_space.end - reg_a_space.start < 2 {
        return reg_a_space
            .clone()
            .find(|i| binary_reg_search(init_comp, *i..*i).is_ok())
            .ok_or("Search Space exhausted");
    }

    if result.len() < init_comp.program.len() {
        return binary_reg_search(init_comp, midpoint_reg..reg_a_space.end);
    }

    if result.len() > init_comp.program.len() {
        return binary_reg_search(init_comp, reg_a_space.start..midpoint_reg);
    }

    let cmp = result
        .clone()
        .into_iter()
        .zip(&init_comp.program)
        .find_map(|(r, ic)| match r.cmp(&ic) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
        })
        .ok_or("Invalid comparison: Equal?")?;

    match cmp {
        cmp::Ordering::Less => {
            binary_reg_search(init_comp, midpoint_reg..reg_a_space.end)
        }
        cmp::Ordering::Greater => {
            binary_reg_search(init_comp, reg_a_space.start..midpoint_reg)
        }
        _ => Err("Unknown state"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_COMP: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_exec() {
        let program = "2,6"
            .split(",")
            .collect::<String>()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let memory = Memory {
            ip: 0,
            register_a: 0,
            register_b: 0,
            register_c: 9,
            out: vec![],
        };
        let mut comp = Comp { program, memory };
        let _ = comp.execute();
        println!("{:?}", comp.memory);

        assert_eq!(comp.memory.register_b, 1);

        let program = "5,0,5,1,5,4"
            .split(",")
            .collect::<String>()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let memory = Memory {
            ip: 0,
            register_a: 10,
            register_b: 0,
            register_c: 0,
            out: vec![],
        };
        let mut comp = Comp { program, memory };
        let _ = comp.execute();
        println!("{:?}", comp.memory);

        assert_eq!(comp.memory.out, vec![0, 1, 2]);

        let program = "0,1,5,4,3,0"
            .split(",")
            .collect::<String>()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let memory = Memory {
            ip: 0,
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            out: vec![],
        };
        let mut comp = Comp { program, memory };
        let _ = comp.execute();
        println!("{:?}", comp.memory);

        assert_eq!(comp.memory.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(comp.memory.register_a, 0);

        let program = "1,7"
            .split(",")
            .collect::<String>()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let memory = Memory {
            ip: 0,
            register_a: 0,
            register_b: 29,
            register_c: 0,
            out: vec![],
        };
        let mut comp = Comp { program, memory };
        let _ = comp.execute();
        println!("{:?}", comp.memory);

        assert_eq!(comp.memory.register_b, 26);

        let program = "4,0"
            .split(",")
            .collect::<String>()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let memory = Memory {
            ip: 0,
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            out: vec![],
        };
        let mut comp = Comp { program, memory };
        let _ = comp.execute();
        println!("{:?}", comp.memory);

        assert_eq!(comp.memory.register_b, 44354);
    }

    #[test]
    fn test_parse() {
        let mut comp = TEST_COMP.parse::<Comp>().unwrap();
        let _ = comp.execute();
        assert_eq!(comp.memory.out, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
