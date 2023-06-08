use std::{collections::HashSet, fs};

use lazy_static::lazy_static;

const FILE_NAME: &str = "input.txt";

#[derive(Debug)]
struct Assignment {
    from: String,
    to: String,
}

lazy_static! {
    static ref ASSIGNMENTS: Vec<Assignment> = {
        fs::read_to_string(FILE_NAME)
            .unwrap()
            .lines()
            .filter(|line| line.contains("=>"))
            .map(|line| {
                let (from, to) = line
                    .split_once("=>")
                    .map(|(a, b)| (a.trim(), b.trim()))
                    .unwrap();
                Assignment {
                    from: from.to_string(),
                    to: to.to_string(),
                }
            })
            .collect()
    };

#[derive(Debug)]
    static ref MOLECULE: String = fs::read_to_string(FILE_NAME).unwrap().lines().into_iter().last().unwrap().trim().to_string();
}

fn main() {
    let mut possibles = HashSet::new();
    for assignment in ASSIGNMENTS.iter() {
        println!("{:?}", assignment);

        for (position, sub_str) in MOLECULE.match_indices(assignment.from.as_str()) {
            let mut new_string = MOLECULE.clone();
            println!("{}", new_string);

            for _ in 0..sub_str.len() {
                new_string.remove(position);
            }

            println!("{}", new_string);
            new_string.replace_range(position..position, assignment.to.as_str());
            println!("{}", new_string);
            possibles.insert(new_string);
        }
    }

    println!("{:?}", possibles);
    println!("{}", possibles.len());
}
