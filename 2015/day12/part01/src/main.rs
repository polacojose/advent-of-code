use std::fs;

use regex::Regex;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    for line in file_string.lines() {
        let total = get_line_total(line);
        println!("{}", total);
    }
}

fn get_line_total(line: &str) -> i32 {
    let mut total = 0;
    let pattern = Regex::new(r"([-0-9]+)+").unwrap();
    let captures = pattern.find_iter(line);

    for capture in captures {
        total += capture.as_str().parse::<i32>().unwrap();
    }
    total
}
