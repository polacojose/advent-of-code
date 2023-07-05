use part01::{self, EasterBunnyDeflater};
use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    for line in file_string.lines() {
        let decoded_line = EasterBunnyDeflater::decode_line(line);
        println!("{} = {}", decoded_line, decoded_line.len());
    }
}
