use std::fs;

use part01::{Instruction, REGISTERS};

fn main() {
    let instructions = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    let mut largest_during_process = 0;

    for instruction in instructions {
        println!("{:?}", instruction);
        instruction.execute();
        largest_during_process =
            largest_during_process.max(*REGISTERS.read().unwrap().values().max().unwrap());
    }

    println!("{:?}", REGISTERS.read().unwrap());
    println!("Largest During Process: {largest_during_process}");
    println!(
        "Largest Final Value: {}",
        REGISTERS.read().unwrap().values().max().unwrap()
    )
}
