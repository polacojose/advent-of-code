use std::{collections::HashSet, fs};

fn main() {
    let mut memory_banks = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace())
        .flatten()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut configurations = HashSet::new();

    let mut loop_detected = false;
    loop {
        if configurations.contains(&memory_banks) {
            if loop_detected {
                break;
            }

            configurations.clear();
            loop_detected = true;
        }
        configurations.insert(memory_banks.clone());

        let mut max = memory_banks.iter().max().unwrap().to_owned();
        let max_pos = memory_banks.iter().position(|b| b == &max).unwrap();
        *memory_banks.get_mut(max_pos).unwrap() = 0;

        for i in max_pos + 1.. {
            let pos = i % memory_banks.len();

            *memory_banks.get_mut(pos).unwrap() += 1;
            max -= 1;

            if max == 0 {
                break;
            }
        }

        println!("{:?}", memory_banks);
    }

    println!("Loop Size: {}", configurations.len());
}
