use std::fs;

use part01::GroupScoreStringCounter;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    for line in file_string.lines() {
        let mut scanner = GroupScoreStringCounter::new(line);
        while scanner.scan() {}
        println!("Score: {}", scanner.score);
        println!("Garbage Count: {}", scanner.garbage_count);
    }
}
