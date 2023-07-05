use std::{collections::HashMap, fs};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut positions: Vec<HashMap<char, u32>> = Vec::new();
    for line in file_string.lines() {
        for (position, c) in line.chars().enumerate() {
            if let Some(count_map) = positions.get_mut(position) {
                count_map.entry(c).and_modify(|n| *n += 1).or_insert(1);
            } else {
                let mut count_map = HashMap::new();
                count_map.insert(c, 1);
                positions.push(count_map);
            }
        }
    }

    let pass = positions
        .into_iter()
        .map(|x| {
            x.into_iter()
                .min_by(|a, b| a.1.cmp(&b.1))
                .map(|x| x.0)
                .unwrap()
        })
        .collect::<Vec<_>>();

    println!("{:#?}", String::from_iter(pass));
}
