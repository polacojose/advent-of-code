use std::{collections::HashSet, fs};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut valid_sum = 0;
    for words in file_string
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>()
    {
        let word_set: HashSet<_> = HashSet::from_iter(words.clone());
        let valid = words.len() == word_set.len();

        if valid {
            valid_sum += 1;
        }

        println!("Valid: {} | {:?}", valid, words);
    }

    println!("Valid Sum: {}", valid_sum);
}
