use std::{collections::HashMap, fs};

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
struct Relationship {
    happiness: i32,
}

#[derive(Debug, Clone)]
struct Guest {
    relationships: HashMap<String, Relationship>,
}

lazy_static! {
#[derive(Debug, Clone)]
    static ref GUESTS: HashMap<String, Guest> = {
        let file_string = fs::read_to_string("input.txt").unwrap();

        let mut guests: HashMap<String, Guest> = HashMap::new();
        for line in file_string.lines() {
            let split = line.split(" ").collect::<Vec<_>>();

            let name = split[0].to_uppercase();
            guests.entry(name.clone()).or_insert(Guest {
                relationships: HashMap::new(),
            });

            //Get mutable reference to relationships
            let ref mut relationships = guests.get_mut(&name).unwrap().relationships;
            let happiness = match split[2]{
                "gain" => {
                    split[3].parse::<i32>().unwrap()
                }
                "lose" => {
                    0 - split[3].parse::<i32>().unwrap()
                }
                _=> {
                    0
                }
            };

            relationships.insert(split[10].replace(".", "").to_uppercase(), Relationship { happiness });
        }
        guests
    };
}

fn main() {
    for (name, _) in GUESTS.iter() {
        let length = find_paths(name.to_string(), vec![name.to_string()], 0);
        println!("{}: {}", name, length.unwrap().1);
        return;
    }
}

fn find_paths(name_a: String, seated: Vec<String>, happiness: i32) -> Option<(Vec<String>, i32)> {
    println!("comparison");
    let relationships = GUESTS.get(&name_a).unwrap().relationships.clone();

    let mut max_relationship_seating_arrangment = Vec::new();
    let mut max_relationship = i32::MIN;

    for (name_b, relationship) in relationships.iter() {
        if !seated.contains(&name_b) {
            let mut seated = seated.clone();
            seated.push(name_b.clone());

            let receieved_happiness = GUESTS
                .get(name_b)
                .unwrap()
                .relationships
                .get(&name_a)
                .unwrap()
                .happiness;

            let combined_happiness = happiness + relationship.happiness + receieved_happiness;

            if seated.len() == GUESTS.keys().len() {
                // Get first and last seated
                let last_seated = seated[seated.len() - 1].clone();
                let first_seated = seated[0].clone();

                let happiness_a = GUESTS
                    .get(&last_seated)
                    .unwrap()
                    .relationships
                    .get(&first_seated)
                    .unwrap()
                    .happiness;

                let happiness_b = GUESTS
                    .get(&first_seated)
                    .unwrap()
                    .relationships
                    .get(&last_seated)
                    .unwrap()
                    .happiness;

                let combined_happiness = combined_happiness + happiness_a + happiness_b;

                //println!("{:?}: {}", seated, combined_happiness);
                //println!("===============================");

                if combined_happiness > max_relationship {
                    max_relationship = combined_happiness;
                    max_relationship_seating_arrangment = seated.clone();
                }
            }

            let val = find_paths(name_b.clone(), seated.clone(), combined_happiness);

            if let Some(v) = val {
                if v.1 > max_relationship {
                    max_relationship = v.1;
                    max_relationship_seating_arrangment = v.0;
                }
            }
        }
    }

    if max_relationship != i32::MAX {
        Some((max_relationship_seating_arrangment, max_relationship))
    } else {
        None
    }
}
