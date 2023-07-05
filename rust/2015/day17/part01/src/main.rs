#[allow(dead_code, unused_variables)]
use lazy_static::lazy_static;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Container {
    id: usize,
    size: u32,
}

const EGGNOG: u32 = 150;

lazy_static! {
    static ref CONTAINERS: Vec<Container> = {
        let containers: Vec<Container> = fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .enumerate()
            .map(|(id, line)| Container {
                id,
                size: line.parse().unwrap(),
            })
            .collect();

        containers
    };
}

fn main() {
    let possible_containers = get_possible_containers_combinations(vec![]);

    let mut unique_containers = Vec::new();
    for mut possible in possible_containers {
        possible.sort_by(|a, b| a.id.cmp(&b.id));
        if !unique_containers.contains(&possible) {
            unique_containers.push(possible);
        }
    }

    println!("{}", unique_containers.len());
}

fn container_combination_value(containers: &Vec<Container>) -> u32 {
    containers.iter().map(|c| c.size).sum()
}

fn get_possible_containers_combinations(used_containers: Vec<Container>) -> Vec<Vec<Container>> {
    let mut container_combinations = Vec::new();
    let containers_left = CONTAINERS.iter().filter(|c| !used_containers.contains(c));

    let last_id = used_containers.iter().map(|a| a.id).max().unwrap_or(0);

    let used_containers_value = container_combination_value(&used_containers);
    for container in containers_left {
        if container.id < last_id {
            continue;
        }

        if used_containers_value + container.size < EGGNOG {
            let mut new_used_containers = used_containers.clone();
            new_used_containers.push(container.clone());

            let possibles = get_possible_containers_combinations(new_used_containers.clone());

            for mut possible in possibles.into_iter().filter(|p| !p.is_empty()) {
                container_combinations.push(possible);
            }
        } else if used_containers_value + container.size == EGGNOG {
            let mut new_used_containers = used_containers.clone();
            new_used_containers.push(container.clone());
            container_combinations.push(new_used_containers.clone());
        }
    }

    container_combinations
}
