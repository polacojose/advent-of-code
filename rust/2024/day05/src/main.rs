use std::{collections::HashSet, fs};

use day05::rule::{Rule, order_set, pages_in_order};

fn main() {
    let (rule_string, page_strings) = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split_once("\n\n")
        .into_iter()
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .last()
        .unwrap();

    part1(rule_string.clone(), page_strings.clone());
    part2(rule_string.clone(), page_strings.clone());
}

fn part1(rule_string: String, page_strings: String) {
    let rules: HashSet<Rule> = rule_string
        .trim()
        .lines()
        .map(|s| s.trim().parse::<Rule>().unwrap())
        .collect();

    let correctly_ordered: Vec<String> = page_strings
        .lines()
        .filter(|ps| pages_in_order(rules.clone(), ps).unwrap())
        .map(|s| s.to_owned())
        .collect();

    let middle_sum = correctly_ordered
        .into_iter()
        .map(|s| {
            let a = s.split(",").collect::<Vec<_>>();
            a.get(a.len() / 2).unwrap().parse::<u64>().unwrap()
        })
        .sum::<u64>();

    println!("Part1: {middle_sum}");
}

fn part2(rule_string: String, page_strings: String) {
    let rules: HashSet<Rule> = rule_string
        .trim()
        .lines()
        .map(|s| s.trim().parse::<Rule>().unwrap())
        .collect();

    let incorrectly_ordered: Vec<String> = page_strings
        .lines()
        .filter(|ps| !pages_in_order(rules.clone(), ps).unwrap())
        .map(|s| s.to_owned())
        .collect();

    let middle_sum = incorrectly_ordered
        .into_iter()
        .map(|pages| {
            let a = order_set(rules.clone(), pages).unwrap();
            a.get(a.len() / 2).unwrap().parse::<u64>().unwrap()
        })
        .sum::<u64>();

    println!("Part1: {middle_sum}");
}
