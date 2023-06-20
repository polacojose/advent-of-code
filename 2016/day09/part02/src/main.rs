use part02::EasterBunnyRecursiveInflator;
use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    for line in file_string.lines() {
        let mut inflator = EasterBunnyRecursiveInflator::new(line);

        let mut size: usize = 0;
        //let mut chars = String::new();
        while let Some(decoded_chars) = inflator.next() {
            size += decoded_chars.len();
            //let decoded_string = decoded_chars.into_iter().collect::<String>();
            //chars.push_str(&decoded_string);
            //println!("size: {}", size);
        }
        //println!("{}", chars);
        println!("size: {}", size);
        return;
    }
}
