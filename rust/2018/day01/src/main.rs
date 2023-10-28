use std::{collections::HashSet, fs};

fn main() {
    let frequency_changes = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    part1(&frequency_changes);
    part2(&frequency_changes);
}

fn part1(frequency_changes: &Vec<isize>) {
    println!(
        "Frequencies Sum: {}",
        frequency_changes.iter().sum::<isize>()
    );
}

fn part2(frequency_changes: &Vec<isize>) {
    let mut change_set = HashSet::new();
    let mut starting_frequency = 0;
    loop {
        for fc in frequency_changes {
            starting_frequency += fc;
            if change_set.contains(&starting_frequency) {
                println!("Duplicate Frequency: {}", starting_frequency);
                return;
            }

            change_set.insert(starting_frequency);
        }
    }
}
