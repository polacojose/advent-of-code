use std::collections::HashSet;

use lazy_static::lazy_static;
use part01::get_dense_hash;

lazy_static! {
    static ref DEFRAG_MAP: Vec<Vec<char>> = {
        let mut map = Vec::new();
        for i in 0..128 {
            let out = get_dense_hash(format!("ugkiagan-{}", i), 256);
            map.push(get_defrag_line(out).chars().collect::<Vec<_>>());
        }
        map
    };
}

fn main() {
    let mut used: HashSet<(usize, usize)> = Default::default();
    let mut groups = Vec::new();
    for y in 0..128 {
        for x in 0..128 {
            if used.contains(&(x, y)) {
                continue;
            }

            if !is_used(&(x as isize, y as isize)) {
                continue;
            }

            used.insert((x, y));
            let mut group = get_group((x, y), &mut used);
            group.insert((x, y));
            for coord in group.iter() {
                used.insert(coord.clone());
            }
            groups.push(group)
        }
    }
    println!("{:?}", groups.len());
    print_groups(groups);
}

fn print_groups(groups: Vec<HashSet<(usize, usize)>>) {
    for y in 0..128 {
        for x in 0..128 {
            if let Some(index) = groups.iter().position(|a| a.contains(&(x, y))) {
                print!("{:3}", index + 1);
            } else {
                print!("  .");
            }
        }
        println!()
    }
}

fn get_group(coord: (usize, usize), used: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut group = HashSet::new();
    for neighbor in get_adjacent_used(coord.0, coord.1) {
        if used.contains(&neighbor) {
            continue;
        }

        group.insert(neighbor);
        used.insert(neighbor);
        for sub_neighbor in get_group(neighbor, used) {
            group.insert(sub_neighbor);
        }
    }

    group
}

fn is_used(coord: &(isize, isize)) -> bool {
    let mut is_used = false;
    if let Some(row) = DEFRAG_MAP.get(coord.1 as usize) {
        if let Some(c) = row.get(coord.0 as usize) {
            is_used = c == &'#';
        }
    }
    is_used
}

fn get_adjacent_used(x: usize, y: usize) -> Vec<(usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(|offset| (offset.0 + x, offset.1 + y))
        .filter(|coord| coord.0 >= 0 && coord.1 >= 0)
        .filter(is_used)
        .map(|coord| (coord.0 as usize, coord.1 as usize))
        .collect::<Vec<_>>()
}

fn get_defrag_line(dense_hash: String) -> String {
    dense_hash
        .chars()
        .into_iter()
        .map(|x| format!("{:04b}", i8::from_str_radix(&x.to_string(), 16).unwrap()))
        .collect::<String>()
        .chars()
        .into_iter()
        .map(|x| if x == '1' { '#' } else { '.' })
        .collect::<String>()
}
