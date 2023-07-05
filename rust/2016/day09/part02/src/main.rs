use std::fs;

use inflator::EasterBunnyRecursiveInflator;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    for line in file_string.lines() {
        println!("{}", line);

        let mut inflator = EasterBunnyRecursiveInflator::new(line);
        let mut size: usize = 0;

        while let Some(decoded_chars) = inflator.next() {
            size += decoded_chars.len();
            if size % 100000 == 0 {
                println!("size: {}", size);
            }
        }
        println!("size: {}", size);
    }
}
