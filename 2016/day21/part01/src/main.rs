use part01::INSTRUCTIONS;

fn main() {
    let mut test_string: String = "abcdefgh".to_string();
    for instruction in INSTRUCTIONS.iter() {
        test_string = instruction.execute(&test_string);
    }
    println!("{}", test_string);
}
