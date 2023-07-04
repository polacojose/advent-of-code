use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut diffs = Vec::new();

    for line in file_string.lines() {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let max = numbers.iter().max().unwrap();
        let min = numbers.iter().min().unwrap();

        let diff = max - min;
        diffs.push(diff);

        println!("Diff: {}", diff);
    }

    println!("Checksum: {}", diffs.iter().sum::<usize>());
}
