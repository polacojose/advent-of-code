use lazy_static::lazy_static;
use std::{fs, sync::RwLock};

lazy_static! {
    static ref MINIMUM_QUANTIUM_ENTANGLEMENT: RwLock<u64> = RwLock::new(u64::MAX);
    static ref MINIMUM_LENGTH: RwLock<usize> = RwLock::new(usize::MAX);
}

fn main() {
    let available_items = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .rev()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let target_sum = available_items.iter().sum::<u64>() / 4;
    let mut max_length = available_items.len() / 4;

    for first_length in 1..=max_length {
        let first_combinations =
            get_combinations(&available_items, &vec![], target_sum, first_length);

        let first_combinations = first_combinations.into_iter().filter(|x| {
            let qe = x.iter().map(|x| *x).product::<u64>();
            if x.len() <= *MINIMUM_LENGTH.read().unwrap() {
                if qe < *MINIMUM_QUANTIUM_ENTANGLEMENT.read().unwrap() {
                    println!("Testing: {:?}", x);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });

        for first_set in first_combinations {
            let new_available_items: Vec<u64> = available_items
                .clone()
                .into_iter()
                .filter(|x| !first_set.contains(x))
                .collect();

            let mut found = false;
            for second_length in first_set.len()..new_available_items.len() {
                let second_combinations =
                    get_combinations(&new_available_items, &vec![], target_sum, second_length);

                for second_set in second_combinations {
                    let new_available_items: Vec<u64> = new_available_items
                        .clone()
                        .into_iter()
                        .filter(|x| !second_set.contains(x))
                        .collect();

                    for third_length in first_set.len()..new_available_items.len() {
                        let third_combinations = get_combinations(
                            &new_available_items,
                            &vec![],
                            target_sum,
                            third_length,
                        );

                        for third_set in third_combinations {
                            *MINIMUM_LENGTH.write().unwrap() = first_set.len();
                            max_length = max_length.min(*MINIMUM_LENGTH.read().unwrap());

                            let first_qe =
                                first_set.iter().map(|x| *x).reduce(|a, b| a * b).unwrap();

                            *MINIMUM_QUANTIUM_ENTANGLEMENT.write().unwrap() = first_qe;
                            println!("qe: {}", *MINIMUM_QUANTIUM_ENTANGLEMENT.read().unwrap());
                            let fourth_set = new_available_items
                                .clone()
                                .into_iter()
                                .filter(|x| !third_set.contains(x))
                                .collect::<Vec<u64>>();
                            println!(
                                "1: {:?} = {:.3e}\n2: {:?} = {:.3e}\n3: {:?} = {:.3e}\n4: {:?} = {:.3e}",
                                first_set,
                                first_set.iter().product::<u64>(),
                                second_set,
                                second_set.iter().product::<u64>(),
                                third_set,
                                third_set.iter().product::<u64>(),
                                fourth_set,
                                fourth_set.iter().product::<u64>(),
                            );
                            println!();
                            found = true;
                            break;
                        }

                        if found {
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
    }
}

fn get_combinations(
    available_items: &Vec<u64>,
    used_items: &Vec<u64>,
    target_sum: u64,
    target_length: usize,
) -> Vec<Vec<u64>> {
    let mut combinations = Vec::new();

    for item_index in 0..available_items.len() {
        let mut new_available_items = available_items.clone();
        let item = new_available_items.remove(item_index);

        if !used_items.is_empty() && &item > used_items.last().unwrap() {
            continue;
        }

        let mut new_used_items = used_items.clone();
        new_used_items.push(item);

        if new_used_items.len() == target_length {
            let used_items_sum = new_used_items.iter().sum::<u64>();
            if used_items_sum != target_sum {
                continue;
            }

            combinations.push(new_used_items.clone());
        }

        let mut combos = get_combinations(
            &new_available_items.clone(),
            &new_used_items,
            target_sum,
            target_length,
        );
        combinations.append(&mut combos);
    }

    combinations
}
