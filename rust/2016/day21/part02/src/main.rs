use part02::INSTRUCTIONS;

fn main() {
    let mut test_string: String = "fbgdceah".to_string();

    for i in 0.. {
        for instruction in INSTRUCTIONS.iter() {
            let old_string = test_string.clone();
            test_string = instruction.encrypt(&test_string);
        }
        println!("{i} = {}", test_string);
        assert_ne!(test_string, "fbgdceah", "{i}");
    }
}
