use part01::{INSTRUCTIONS, REGISTERS};

fn main() {
    while *REGISTERS.read().unwrap().get("ip").unwrap() < INSTRUCTIONS.len() as i32 {
        let ip = *REGISTERS.read().unwrap().get("ip").unwrap();
        let instruction = INSTRUCTIONS.get(ip as usize).unwrap();
        instruction.execute();
        println!("{:#?}", *REGISTERS);
    }
}
