use crate::intcode::{IntCode, OpCode};

#[derive(Debug, PartialEq, Eq)]
enum IntCodeProgramState {
    RUN,
    STP,
}

#[derive(Debug)]
pub struct IntCodeProgram {
    state: IntCodeProgramState,

    ip: usize,
    program: Vec<u32>,
}

impl IntCodeProgram {
    pub fn new(program: Vec<u32>) -> Self {
        Self {
            state: IntCodeProgramState::RUN,
            ip: 0,
            program,
        }
    }

    pub fn execute(&mut self) -> &Vec<u32> {
        while self.state != IntCodeProgramState::STP {
            let curr_intcode = self.curr_intcode();
            self.ip += 4;

            if let Ok(val) = curr_intcode.value(&self.program) {
                self.program[curr_intcode.dest] = val;
            } else {
                self.state = IntCodeProgramState::STP;
            }
        }

        return &self.program;
    }

    fn curr_intcode(&self) -> IntCode {
        let opcode = OpCode::from(self.program[self.ip] as u8);
        match opcode {
            OpCode::STP => IntCode::new(opcode, 0, 0, 0),
            _ => IntCode::new(
                opcode,
                self.program[self.ip + 1] as usize,
                self.program[self.ip + 2] as usize,
                self.program[self.ip + 3] as usize,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_program {
        ($in:expr, $exp:expr) => {
            let s = $in.to_owned();
            let mut intcode_program =
                IntCodeProgram::new(s.split(",").map(|l| l.parse::<u32>().unwrap()).collect());
            let result = intcode_program.execute();
            assert_eq!(result, $exp);
        };
    }

    #[test]
    fn test_iterator() {
        test_program!(
            "1,9,10,3,2,3,11,0,99,30,40,50",
            &vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        test_program!("1,0,0,0,99", &vec![2, 0, 0, 0, 99]);
        test_program!("2,3,0,3,99", &vec![2, 3, 0, 6, 99]);
        test_program!("2,4,4,5,99,0", &vec![2, 4, 4, 5, 99, 9801]);
        test_program!("1,1,1,4,99,5,6,0,99", &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
