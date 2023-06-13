use std::fs;

use part01::{Instruction, FACTORY, INSTRUCTION_BUFFER};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    for line in file_string.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        match &instruction {
            Instruction::Comparison {
                source,
                destination_low,
                destination_high,
            } => {
                instruction.store_or_execute();
            }
            _ => {}
        }
    }

    println!("{:?}", FACTORY.read().unwrap());
    for line in file_string.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        match &instruction {
            Instruction::Assign { value, destination } => {
                instruction.store_or_execute();
                Instruction::execute_stored();
                println!("{:?}", FACTORY.read().unwrap());
                println!("========================");
            }
            _ => {}
        }
    }
    println!("{:?}", FACTORY.read().unwrap());
}
