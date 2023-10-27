pub mod pattern;
pub mod rule;

use std::fs;

use pattern::Pattern;

use crate::rule::{transform_pattern, Rule};

fn main() {
    let rules: Vec<Rule> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    run(5, &rules);
    run(18, &rules);
}

fn run(iterations: i32, rules: &Vec<Rule>) {
    let mut src_pattern = Pattern(vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ]);

    for _ in 0..iterations {
        src_pattern = transform_pattern(&rules, &src_pattern)
            .expect(format!("Failed to transform pattern:\n{}", src_pattern).as_str());
    }
    println!(
        "On After {iterations} iterations: {}",
        src_pattern.matches('#')
    );
}
