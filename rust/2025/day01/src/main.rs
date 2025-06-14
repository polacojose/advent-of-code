use std::{collections::HashMap, fs};

fn main() {
    let (list_a, list_b) = get_lists();

    println!("Part1: {}", part1(&mut list_a.clone(), &mut list_b.clone()));
    println!("Part2: {}", part2(&list_a, &list_b));
}

fn part1(list_a: &mut [u32], list_b: &mut [u32]) -> u32 {
    list_a.sort();
    list_b.sort();

    list_a
        .into_iter()
        .zip(list_b)
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(list_a: &[u32], list_b: &[u32]) -> u32 {
    let occurance_map: HashMap<&u32, u32> = list_b.iter().fold(HashMap::new(), |mut acc, a| {
        acc.entry(a).and_modify(|x| *x += 1).or_insert(1);
        acc
    });

    list_a
        .iter()
        .map(|a| a * occurance_map.get(&a).unwrap_or(&0))
        .sum()
}

fn get_lists_from_str(str: &str) -> (Vec<u32>, Vec<u32>) {
    str.trim()
        .lines()
        .filter_map(|l| l.trim().split_once(" "))
        .map(|(a, b)| {
            (
                a.trim().parse::<u32>().unwrap(),
                b.trim().parse::<u32>().unwrap(),
            )
        })
        .unzip()
}

fn get_lists() -> (Vec<u32>, Vec<u32>) {
    get_lists_from_str(&fs::read_to_string("input.txt").unwrap().trim())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r"3   4
                                4   3
                                2   5
                                1   3
                                3   9
                                3   3";

    #[test]
    fn test_part1() {
        let (mut list_a, mut list_b) = get_lists_from_str(TEST_DATA);
        let result = part1(&mut list_a, &mut list_b);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let (list_a, list_b) = get_lists_from_str(TEST_DATA);
        let result = part2(&list_a, &list_b);
        assert_eq!(result, 31);
    }
}
