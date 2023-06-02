use md5;
use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    for line in file_string.lines() {
        let secret = line.trim();

        for answer in 0.. {
            let digest = format!("{:x}", md5::compute(format!("{}{}", secret, answer)));

            if digest.starts_with("000000") {
                println!("{}", answer);
                break;
            };
        }
    }
}
