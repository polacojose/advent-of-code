use std::{collections::HashMap, fs};

fn main() {
    part1();
    part2();
}

fn part1() {
    let digit_isolations = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| isolate_digits(s))
        .collect::<Vec<u32>>();

    println!("Part1 sum: {}", digit_isolations.into_iter().sum::<u32>());
}

fn part2() {
    let digit_isolations = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| isolate_digits_real(s))
        .collect::<Vec<u32>>();

    println!("Part2 sum: {}", digit_isolations.into_iter().sum::<u32>());
}

fn isolate_digits(s: impl AsRef<str>) -> u32 {
    let digits = s
        .as_ref()
        .chars()
        .into_iter()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<_>>();

    vec![digits.first().unwrap(), digits.last().unwrap()]
        .into_iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn replace_word_digits(s: impl AsRef<str>) -> String {
    let s = s.as_ref().trim().to_lowercase();

    let mut num_map = HashMap::new();
    num_map.insert("zero", "0");
    num_map.insert("one", "1");
    num_map.insert("two", "2");
    num_map.insert("three", "3");
    num_map.insert("four", "4");
    num_map.insert("five", "5");
    num_map.insert("six", "6");
    num_map.insert("seven", "7");
    num_map.insert("eight", "8");
    num_map.insert("nine", "9");

    let mut digits = Vec::new();

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if c.is_digit(10) {
            digits.push(c);
        }

        for (k, v) in &num_map {
            if let Some(ii) = s[i..].find(k) {
                if ii == 0 {
                    digits.push(v.chars().nth(0).unwrap());
                    break;
                }
            }
        }
    }

    digits.into_iter().collect()
}

fn isolate_digits_real(s: impl AsRef<str>) -> u32 {
    let digits = replace_word_digits(s)
        .chars()
        .into_iter()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<_>>();

    vec![digits.first().unwrap(), digits.last().unwrap()]
        .into_iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_digit_isolation() {
        let digit_isolations = fs::read_to_string("test-input.txt")
            .unwrap()
            .lines()
            .map(|s| isolate_digits(s))
            .collect::<Vec<u32>>();

        assert_eq!(digit_isolations[0], 12);
        assert_eq!(digit_isolations[1], 38);
        assert_eq!(digit_isolations[2], 15);
        assert_eq!(digit_isolations[3], 77);
        assert_eq!(digit_isolations.into_iter().sum::<u32>(), 142);
    }

    #[test]
    fn test_digit_isolation_real() {
        let digit_isolations = fs::read_to_string("test-input2.txt")
            .unwrap()
            .lines()
            .map(|s| isolate_digits_real(s))
            .collect::<Vec<u32>>();

        assert_eq!(digit_isolations[0], 29);
        assert_eq!(digit_isolations[1], 83);
        assert_eq!(digit_isolations[2], 13);
        assert_eq!(digit_isolations[3], 24);
        assert_eq!(digit_isolations[4], 42);
        assert_eq!(digit_isolations[5], 14);
        assert_eq!(digit_isolations[6], 76);
        assert_eq!(digit_isolations.into_iter().sum::<u32>(), 281);
    }
}
