pub mod lib;
use crate::lib::get_instructions;

const INPUT: &str = "abcdefghijklmnop";

fn main() {
    let instructions = get_instructions();

    let mut start = INPUT.to_string().chars().collect::<Vec<_>>();
    for i in &instructions {
        i.execute(&mut start);
    }

    println!("{:?}", start);

    for i in 0..(1000000000 - 1) % 60 {
        if i % 60 == 0 {
            println!("{:?}", start);
        }
        for i in &instructions {
            i.execute(&mut start);
        }
    }

    println!("{:?}", start.into_iter().collect::<String>());
}
