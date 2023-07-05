use std::{thread::sleep, time::Duration};

use part02::{get_instruction_pointer, INSTRUCTIONS, REGISTERS};

fn main() {
    let mut i: usize = 0;
    while get_instruction_pointer() < INSTRUCTIONS.read().unwrap().len() {
        let instruction = INSTRUCTIONS.read().unwrap()[get_instruction_pointer()].clone();
        instruction.execute();

        i += 1;
        if i % 1 == 0 {
            println!("{:?}", instruction);
            // println!("{:?}", INSTRUCTIONS.read().unwrap());
            println!("{:?}", REGISTERS.read().unwrap());
            println!("=====================");
        }

        //sleep(Duration::from_millis(1000));
    }
    println!("{:?}", REGISTERS.read().unwrap());
}
