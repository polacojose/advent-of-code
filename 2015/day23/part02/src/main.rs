use part01::{INSTRUCTIONS, REGISTERS};

fn main() {
    while *REGISTERS.read().unwrap().get("ip").unwrap() < INSTRUCTIONS.len() as i64 {
        let ip = *REGISTERS.read().unwrap().get("ip").unwrap();
        let instruction = INSTRUCTIONS.get(ip as usize).unwrap();

        println!("{}: {:?}", ip, instruction);
        instruction.execute();
        println!(
            "a:{} b:{}",
            *REGISTERS.read().unwrap().get("a").unwrap(),
            *REGISTERS.read().unwrap().get("b").unwrap()
        );
        println!();
    }
}
