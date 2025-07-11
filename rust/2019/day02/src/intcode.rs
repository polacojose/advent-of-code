use core::panic;

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    ADD,
    MUL,
    STP,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            1 => OpCode::ADD,
            2 => OpCode::MUL,
            99 => OpCode::STP,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct IntCode {
    opcode: OpCode,
    operand_a_pos: usize,
    operand_b_pos: usize,
    pub dest: usize,
}

impl IntCode {
    pub fn new(opcode: OpCode, operand_a: usize, operand_b: usize, dest: usize) -> Self {
        Self {
            opcode,
            operand_a_pos: operand_a,
            operand_b_pos: operand_b,
            dest,
        }
    }

    pub fn value(&self, program: &Vec<u32>) -> Result<u32, ()> {
        match self.opcode {
            OpCode::ADD => Ok(program[self.operand_a_pos] + program[self.operand_b_pos]),
            OpCode::MUL => Ok(program[self.operand_a_pos] * program[self.operand_b_pos]),
            OpCode::STP => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_value_execution() {
        let s: Vec<u32> = "1,2,3,30"
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let int_code = IntCode::new(
            OpCode::from(s[0] as u8),
            s[1] as usize,
            s[2] as usize,
            s[3] as usize,
        );

        assert_eq!(int_code.value(&s), Ok(33));
    }
}
