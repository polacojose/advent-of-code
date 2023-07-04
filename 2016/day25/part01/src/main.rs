use part01::{get_instruction_pointer, set_start_registers, INSTRUCTIONS, REGISTERS};

fn main() {
    for a in 0.. {
        let mut test = 0;

        set_start_registers(a);
        println!("\n=={a}==\n");

        while get_instruction_pointer() < INSTRUCTIONS.read().unwrap().len() {
            let instruction = INSTRUCTIONS.read().unwrap()[get_instruction_pointer()].clone();
            if instruction.execute().is_err() {
                break;
            }

            test += 1;
            if test == 100000 {
                println!("!!{a}!!");
                return;
            }
        }
    }

    println!("{:?}", REGISTERS.read().unwrap());
}
