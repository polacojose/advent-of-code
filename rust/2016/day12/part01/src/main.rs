use std::{thread::sleep, time::Duration};

use part01::{INSTRUCTIONS, REGISTERS};

fn main() {
    while let Some(instruction) = INSTRUCTIONS.get(get_instruction_pointer()) {
        //println!("{:?}", instruction);
        instruction.execute();
        //sleep(Duration::from_millis(1));
    }
    println!("{:?}", REGISTERS.read().unwrap());
}

fn get_instruction_pointer() -> usize {
    REGISTERS.read().unwrap().get("ip").unwrap().clone()
}
