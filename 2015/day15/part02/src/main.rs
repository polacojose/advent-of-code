use std::{
    collections::{HashMap, HashSet},
    fs,
};

use lazy_static::lazy_static;
use tokio::task::JoinSet;

#[derive(Debug, Clone)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

const MAX_COOKIE_CAPACITY: usize = 100;

lazy_static! {
    static ref INGREDIENTS: HashMap<String, Ingredient> = {
        let file_string = fs::read_to_string("input.txt").unwrap();

        let mut ingredients = HashMap::new();
        for line in file_string.lines() {
            let split = line.split(" ").collect::<Vec<&str>>();
            ingredients.insert(
                split[0].replace(":", ""),
                Ingredient {
                    capacity: split[2].replace(",", "").parse::<i32>().unwrap(),
                    durability: split[4].replace(",", "").parse::<i32>().unwrap(),
                    flavor: split[6].replace(",", "").parse::<i32>().unwrap(),
                    texture: split[8].replace(",", "").parse::<i32>().unwrap(),
                    calories: split[10].replace(",", "").parse::<i32>().unwrap(),
                },
            );
        }

        ingredients
    };
}

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let mut join_set = JoinSet::new();
    let first_ingredient_name = INGREDIENTS.keys().next().unwrap().clone();
    for amount in 1..(MAX_COOKIE_CAPACITY - (INGREDIENTS.keys().len() - 1)) {
        let mut used_ingredients = HashMap::new();
        used_ingredients.insert(first_ingredient_name.clone(), amount as i32);

        join_set.spawn(async move {
            let (score, used_ingredients) = get_max_cookie(amount as u32, used_ingredients);
            assert!(used_ingredients.iter().map(|(_, v)| v).sum::<i32>() == 100);
            (score, used_ingredients)
        });
    }

    let mut largest_score = 0;
    let mut best_ingredients = HashMap::new();
    while let Some(res) = join_set.join_next().await {
        let (score, used_ingredients) = res.unwrap();
        if score > largest_score {
            largest_score = score;
            best_ingredients = used_ingredients;
        }
    }

    println!("Largest score: {}", largest_score);
    println!("Best ingredients: {:?}", best_ingredients);
}

fn get_cookie_score(ingredients: Vec<(i32, Ingredient)>) -> i64 {
    let ingredients = ingredients
        .iter()
        .map(|(amount, ingredient)| (*amount as i64, ingredient.clone()))
        .collect::<Vec<(i64, Ingredient)>>();

    let total_calories: i64 = ingredients
        .iter()
        .map(|(amount, ingredient)| amount * ingredient.calories as i64)
        .sum::<i64>()
        .max(0);

    if total_calories != 500 {
        return 0;
    }

    let total_capacity: i64 = ingredients
        .iter()
        .map(|(amount, ingredient)| amount * ingredient.capacity as i64)
        .sum::<i64>()
        .max(0);

    //println!("Total capacity: {}", total_capacity);

    let total_durability: i64 = ingredients
        .iter()
        .map(|(amount, ingredient)| amount * ingredient.durability as i64)
        .sum::<i64>()
        .max(0);

    //println!("Total durability: {}", total_durability);

    let total_flavor: i64 = ingredients
        .iter()
        .map(|(amount, ingredient)| amount * ingredient.flavor as i64)
        .sum::<i64>()
        .max(0);

    //println!("Total flavor: {}", total_flavor);

    let total_texture: i64 = ingredients
        .iter()
        .map(|(amount, ingredient)| amount * ingredient.texture as i64)
        .sum::<i64>()
        .max(0);

    //println!("Total texture: {}", total_texture);

    total_capacity * total_durability * total_flavor * total_texture
}

fn get_max_cookie(
    amounts_used: u32,
    used_ingredients: HashMap<String, i32>,
) -> (i64, HashMap<String, i32>) {
    //If this is the remaining ingredient
    if used_ingredients.len() == INGREDIENTS.keys().len() {
        let amounts_to_cookies = used_ingredients
            .clone()
            .into_iter()
            .map(|(name, amount)| (amount, INGREDIENTS.get(&name).cloned().unwrap()))
            .collect();
        return (get_cookie_score(amounts_to_cookies), used_ingredients);
    }

    let mut max_score = 0;
    let mut max_used_ingredients = used_ingredients.clone();
    let (next_ingredient_name, _) = INGREDIENTS
        .iter()
        .filter(|(name, _)| !used_ingredients.contains_key(*name))
        .next()
        .unwrap();

    if used_ingredients.len() == INGREDIENTS.keys().len() - 1 {
        let mut used_ingredients = used_ingredients.clone();
        used_ingredients.insert(
            next_ingredient_name.clone(),
            MAX_COOKIE_CAPACITY as i32 - amounts_used as i32,
        );
        let amounts_to_cookies = used_ingredients
            .clone()
            .into_iter()
            .map(|(name, amount)| (amount, INGREDIENTS.get(&name).cloned().unwrap()))
            .collect();
        return (get_cookie_score(amounts_to_cookies), used_ingredients);
    }

    let amount_left = MAX_COOKIE_CAPACITY
        - amounts_used as usize
        - (INGREDIENTS.keys().len() - used_ingredients.keys().len() - 1);

    for amount in 1..=(amount_left) {
        let mut used_ingredients = used_ingredients.clone();
        used_ingredients.insert(next_ingredient_name.clone(), amount as i32);
        let (score, used_ingredients) =
            get_max_cookie(amounts_used + amount as u32, used_ingredients.clone());
        if score >= max_score {
            max_score = score;
            max_used_ingredients = used_ingredients.clone();
        }
    }

    assert!(
        max_used_ingredients.keys().len() == INGREDIENTS.keys().len(),
        "{:?} {:?}",
        amounts_used,
        amount_left
    );

    (max_score, max_used_ingredients)
}
