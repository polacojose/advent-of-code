use std::{collections::HashSet, fs};

use lazy_static::lazy_static;

const FILE_NAME: &str = "input.txt";
const MAX_STEPS: usize = 1000;

#[derive(Debug, Clone)]
struct Assignment {
    from: String,
    to: String,
}

lazy_static! {
    static ref ASSIGNMENTS: Vec<Assignment> = {
        let mut assignments = fs::read_to_string(FILE_NAME)
            .unwrap()
            .lines()
            .filter(|line| line.contains("=>"))
            .map(|line| {
                let (to, from) = line
                    .split_once("=>")
                    .map(|(a, b)| (a.trim(), b.trim()))
                    .unwrap();
                Assignment {
                    from: from.to_string(),
                    to: to.to_string(),
                }
            })
            .collect::<Vec<Assignment>>();
        assignments.sort_by(|a, b| b.from.len().cmp(&a.from.len()));
        assignments
    };
    static ref STARTING_MOLECULE: String = "e".to_string();
    static ref MEDICINE_MOLECULE: String = fs::read_to_string(FILE_NAME)
        .unwrap()
        .lines()
        .into_iter()
        .last()
        .unwrap()
        .trim()
        .to_string();
}

fn main() {
    let mut molecule = MEDICINE_MOLECULE.clone();
    println!("{:?}", molecule);

    let mut steps = 0;
    while molecule != STARTING_MOLECULE.to_string() && steps < MAX_STEPS {
        if let Some(assignment) = find_match(&molecule) {
            if let Some((position, sub_str)) =
                molecule.match_indices(assignment.from.as_str()).last()
            {
                for _ in 0..sub_str.len() {
                    molecule.remove(position);
                }
                molecule.replace_range(position..position, assignment.to.as_str());
            }
            println!("{:?} = {}", assignment, molecule);
        }
        steps += 1;
    }
    println!("{}", steps);
}

fn find_match(molecule: &String) -> Option<Assignment> {
    //println!("{:?}", molecule);
    for assignment in ASSIGNMENTS.iter() {
        //println!("{:?}", assignment);
        if let Some(_) = molecule.find(assignment.from.as_str()) {
            //println!("match");
            return Some(assignment.clone());
        }
    }
    None
}
