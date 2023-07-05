use std::{thread::sleep, time::Duration};

use part01::{get_instruction_pointer, INSTRUCTIONS, REGISTERS};

fn main() {
    while get_instruction_pointer() < INSTRUCTIONS.read().unwrap().len() {
        let instruction = INSTRUCTIONS.read().unwrap()[get_instruction_pointer()].clone();
        println!("{:?}", instruction);
        instruction.execute();
        // println!("{:?}", INSTRUCTIONS.read().unwrap());
        println!("{:?}", REGISTERS.read().unwrap());
        println!("=====================");

        //sleep(Duration::from_millis(1000));
    }
    println!("{:?}", REGISTERS.read().unwrap());
}
