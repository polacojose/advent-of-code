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

    let mut src_pattern = Pattern(vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ]);

    println!("{}", &src_pattern);
    for _ in 0..5 {
        src_pattern = transform_pattern(&rules, &src_pattern)
            .expect(format!("Failed to transform pattern:\n{}", src_pattern).as_str());
        println!("{}", src_pattern)
    }

    println!("On: {}", src_pattern.matches('#'));
}
