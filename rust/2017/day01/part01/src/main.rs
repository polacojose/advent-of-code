use std::{fs, usize};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    for line in file_string.lines() {
        let line_chars = line.chars().collect::<Vec<_>>();

        let mut sum = 0;

        let mut i = 1;
        while i < line_chars.len() {
            if line_chars[i - 1] == line_chars[i] {
                sum += line_chars[i].to_string().parse::<usize>().unwrap();
            }
            i += 1;
        }

        if line_chars.first() == line_chars.last() {
            sum += line_chars[0].to_string().parse::<usize>().unwrap();
        }

        println!("Sum: {}", sum);
    }
}
