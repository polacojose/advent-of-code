use std::{
    collections::{hash_set, HashMap, HashSet},
    fs,
    str::FromStr,
};

#[derive(Debug)]
struct Tower {
    name: String,
    weight: usize,
    children: Vec<String>,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for Tower {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((prefix, suffix)) = s.split_once("->") {
            let (name, weight_section) = prefix.trim().split_once(" ").unwrap();
            Ok(Self {
                name: name.to_string(),
                weight: weight_section[1..weight_section.len() - 1].parse().unwrap(),
                children: suffix
                    .replace(",", "")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            })
        } else {
            let (name, weight_section) = s.trim().split_once(" ").unwrap();
            Ok(Self {
                name: name.to_string(),
                weight: weight_section[1..weight_section.len() - 1].parse().unwrap(),

                children: vec![],
            })
        }
    }
}

fn main() {
    let towers = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<Tower>().unwrap())
        .collect::<Vec<_>>();

    let mut root_tower: Option<&Tower> = None;
    for tower_a in &towers {
        let mut root = true;
        for tower_b in &towers {
            if tower_b.children.contains(&tower_a.name) {
                root = false;
                break;
            }
        }

        if root {
            root_tower = Some(tower_a);
            break;
        }
    }

    print_out_of_balance(&root_tower.unwrap(), &towers);
}

fn get_tower_by_name<'a>(name: &String, towers: &'a Vec<Tower>) -> &'a Tower {
    towers.iter().find(|t| &t.name == name).unwrap()
}

fn get_weight(root: &Tower, towers: &Vec<Tower>) -> usize {
    root.weight
        + root
            .children
            .iter()
            .map(|t| get_weight(get_tower_by_name(t, towers), towers))
            .sum::<usize>()
}

fn print_out_of_balance(root: &Tower, towers: &Vec<Tower>) {
    let children = root
        .children
        .iter()
        .map(|t| get_tower_by_name(t, towers))
        .collect::<Vec<_>>();
    let weights = children
        .iter()
        .map(|t| get_weight(t, towers))
        .collect::<Vec<_>>();
    let mut weight_set: HashMap<usize, usize> = HashMap::new();
    for weight in &weights {
        weight_set
            .entry(*weight)
            .and_modify(|w| *w += 1)
            .or_insert(1);
    }

    if weight_set.len() > 1 {
        println!("Wrong Weight: {}", root.name);
        for child in &children {
            print!("{} = {} | ", child.name, get_weight(&child, towers));
        }
        println!();

        let (wrong_weight, _) = weight_set.iter().min_by_key(|w| w.1).unwrap();

        let wrong_weight_pos = weights.iter().position(|x| x == wrong_weight).unwrap();

        println!("Problem Tower: {:?}", children[wrong_weight_pos]);

        print_out_of_balance(children[wrong_weight_pos], towers);
    }
}
