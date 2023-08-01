use std::{fs, thread::sleep, time::Duration};

use program::Program;

use crate::program::Instruction;

pub mod program;

fn main() {
    let instructions = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let mut program_zero = Program::new(0, &instructions);
    let mut program_one = Program::new(1, &instructions);

    loop {
        program_zero.process();
        program_one.process();

        match program_zero.state {
            program::ProgramState::Stalled => match program_one.state {
                program::ProgramState::Stalled => break,
                _ => {}
            },
            _ => {}
        }
    }
    // loop {
    //     program_zero.process();

    //     match program_zero.state {
    //         program::ProgramState::Stalled => break,
    //         _ => {}
    //     }
    // }

    // loop {
    //     program_one.process();

    //     match program_one.state {
    //         program::ProgramState::Stalled => break,
    //         _ => {}
    //     }
    // }

    println!("0: {:?}", program_zero.registers);
    println!("1: {:?}", program_one.registers);
}
