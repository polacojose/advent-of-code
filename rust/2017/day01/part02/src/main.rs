use std::{fs, usize};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    for line in file_string.lines() {
        let line_chars = line.chars().collect::<Vec<_>>();

        let mut sum = 0;

        let jump_size = line_chars.len() / 2;

        for i in 0..line_chars.len() {
            if line_chars[i] == line_chars[(i + jump_size) % line_chars.len()] {
                sum += line_chars[i].to_string().parse::<usize>().unwrap();
            }
        }

        // if line_chars.first() == line_chars.last() {
        //     sum += line_chars[0].to_string().parse::<usize>().unwrap();
        // }

        println!("Sum: {}", sum);
    }
}
